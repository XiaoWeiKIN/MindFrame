//! Local import/export only: no model client, browser, TTS or editor subprocess.
use std::{fs, io::Cursor, path::{Path, PathBuf}};

use anyhow::{Context, Result, ensure};
use image::{ImageFormat, ImageReader, Limits};
use mindframe_core::{Storyboard, materials::{Assets, Project, csv_cell, timestamp}};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::pipeline::{read_source, write_json};

struct Bundle {
    board: Storyboard,
    assets: Assets,
    files: Vec<(PathBuf, String)>,
    dimensions: Vec<Value>,
    audio_duration_ms: Option<u64>,
}

// Publish a complete directory, never a half-imported project. This is a single-writer CLI.
fn publish(out: &Path, write: impl FnOnce(&Path) -> Result<()>) -> Result<()> {
    ensure!(out.symlink_metadata().is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound), "destination already exists or cannot be inspected: {}", out.display());
    let parent = out.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    fs::create_dir_all(parent)?;
    let staging = tempfile::Builder::new().prefix(".mindframe-").tempdir_in(parent)?;
    write(staging.path())?;
    ensure!(out.symlink_metadata().is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound), "destination appeared during preparation: {}", out.display());
    fs::rename(staging.path(), out).with_context(|| format!("publish {}", out.display()))?;
    Ok(())
}

// All relative paths are contract-validated; reject symlinks at each filesystem component.
fn regular_file(root: &Path, relative: &str, max_bytes: u64) -> Result<PathBuf> {
    ensure!(root.symlink_metadata()?.is_dir(), "material root must be a real directory, not a symlink: {}", root.display());
    let mut path = root.to_path_buf();
    for component in relative.split('/') {
        path.push(component);
        let metadata = path.symlink_metadata().with_context(|| format!("missing material {}", path.display()))?;
        ensure!(!metadata.file_type().is_symlink(), "material symlinks are not supported: {}", path.display());
    }
    let metadata = path.metadata()?;
    ensure!(metadata.is_file() && metadata.len() <= max_bytes, "material is not a regular file or exceeds {max_bytes} bytes: {}", path.display());
    Ok(path)
}

fn read_json<T: DeserializeOwned>(root: &Path, name: &str) -> Result<T> {
    let path = regular_file(root, name, 2 * 1024 * 1024)?;
    serde_json::from_slice(&fs::read(&path)?).with_context(|| format!("invalid JSON contract: {}", path.display()))
}

fn inspect_image(path: &Path) -> Result<(String, u32, u32)> {
    let bytes = fs::read(path)?;
    let format = image::guess_format(&bytes).context("image must contain real PNG, JPEG or WebP bytes")?;
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_ascii_lowercase();
    let expected = match extension.as_str() {
        "png" => ImageFormat::Png,
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "webp" => ImageFormat::WebP,
        _ => anyhow::bail!("supported image extensions: .png, .jpg, .jpeg, .webp"),
    };
    ensure!(format == expected, "image extension does not match its bytes: {}", path.display());
    let mut reader = ImageReader::with_format(Cursor::new(bytes), format);
    let mut limits = Limits::default();
    limits.max_image_width = Some(8192);
    limits.max_image_height = Some(8192);
    limits.max_alloc = Some(256 * 1024 * 1024);
    reader.limits(limits);
    let decoded = reader.decode().with_context(|| format!("decode image {}", path.display()))?;
    ensure!(decoded.width() > 0 && decoded.height() > 0, "empty image");
    let canonical = if format == ImageFormat::Jpeg { "jpg" } else { extension.as_str() };
    Ok((canonical.to_owned(), decoded.width(), decoded.height()))
}

fn inspect_audio(path: &Path) -> Result<u64> {
    let mut reader = hound::WavReader::open(path).context("audio must be PCM 16-bit WAV; convert other formats explicitly first")?;
    let spec = reader.spec();
    ensure!(spec.sample_format == hound::SampleFormat::Int && spec.bits_per_sample == 16 && (1..=2).contains(&spec.channels) && spec.sample_rate > 0, "audio must be mono/stereo PCM 16-bit WAV");
    let frames = u64::from(reader.duration());
    let duration = frames * 1000 / u64::from(spec.sample_rate);
    ensure!(duration > 0 && duration <= 3_600_000, "recording must be nonempty and at most one hour");
    // Read all samples to reject truncated recordings, not just a plausible WAV header.
    for sample in reader.samples::<i16>() { sample.context("invalid or truncated WAV samples")?; }
    Ok(duration)
}

fn load_bundle(root: &Path, source: &str) -> Result<Bundle> {
    let board: Storyboard = read_json(root, "storyboard.json")?;
    board.validate(source)?;
    let mut assets: Assets = read_json(root, "assets.json")?;
    assets.validate(&board)?;
    let mut files = Vec::new();
    let mut dimensions = Vec::new();
    // 1. 核对真实图片与场景映射；保留原图字节，不猜文件顺序、不拉伸或重生成。
    for (index, scene) in board.scenes.iter().enumerate() {
        let image = assets.images.iter_mut().find(|image| image.scene_id == scene.id).expect("validated complete scene mapping");
        let path = regular_file(root, &image.file, 50 * 1024 * 1024)?;
        let (extension, width, height) = inspect_image(&path)?;
        image.file = format!("images/{:03}-{}.{}", index + 1, scene.id, extension);
        dimensions.push(json!({"scene_id":scene.id,"file":image.file,"width":width,"height":height}));
        files.push((path, image.file.clone()));
    }
    if let Some(cover) = &mut assets.cover {
        let path = regular_file(root, cover, 50 * 1024 * 1024)?;
        let (extension, _, _) = inspect_image(&path)?;
        *cover = format!("cover.{extension}");
        files.push((path, cover.clone()));
    }
    // 2. 配音和定时字幕可缺省；存在时检查录音长度和逐句对应，不估算时间。
    let audio_duration_ms = if let Some(audio) = &mut assets.audio {
        let path = regular_file(root, audio, 256 * 1024 * 1024)?;
        let duration = inspect_audio(&path)?;
        if let Some(last) = assets.cues.last() {
            ensure!(last.end_ms <= duration, "subtitle timing exceeds the actual recording duration ({duration} ms)");
        }
        *audio = "audio/narration.wav".into();
        files.push((path, audio.clone()));
        Some(duration)
    } else { None };
    Ok(Bundle { board, assets, files, dimensions, audio_duration_ms })
}

fn load_project(project: &Path) -> Result<(Project, String)> {
    let manifest: Project = read_json(project, "project.json")?;
    manifest.validate()?;
    let source_path = regular_file(project, "source.md", 65_536)?;
    Ok((manifest, read_source(&source_path)?))
}

fn copy_materials(bundle: &Bundle, out: &Path) -> Result<()> {
    for (source, relative) in &bundle.files {
        let destination = out.join(relative);
        if let Some(parent) = destination.parent() { fs::create_dir_all(parent)?; }
        fs::copy(source, destination)?;
    }
    write_json(&out.join("storyboard.json"), &bundle.board)?;
    write_json(&out.join("assets.json"), &bundle.assets)?;
    fs::write(out.join("script.md"), bundle.board.script_markdown())?;
    fs::write(out.join("key-points.md"), bundle.board.points_markdown())?;
    Ok(())
}

pub fn init(input: &Path, out: &Path, preset: String) -> Result<()> {
    let source = read_source(input)?;
    let project = Project { schema_version: 1, preset };
    project.validate()?;
    publish(out, |dir| {
        write_json(&dir.join("project.json"), &project)?;
        fs::write(dir.join("source.md"), &source)?;
        let numbered = source.lines().enumerate().map(|(i, line)| format!("{}: {line}\n", i + 1)).collect::<String>();
        fs::write(dir.join("source.numbered.txt"), numbered)?;
        // Embed the canonical skill and worksheet so an installed CLI needs no repository files at runtime.
        let request = format!(
            "{}\n\n目标布局：{}。\n\n## 视觉导演工作流（完整内嵌）\n{}\n\n## 逐页工作表（完整内嵌）\n{}\n\n## Storyboard JSON Schema\n```json\n{}\n```\n\n## Assets JSON Schema\n```json\n{}\n```\n",
            include_str!("../../../prompts/chat-authoring.md"), project.preset,
            include_str!("../../../skills/mindframe-visual-director/SKILL.md"),
            include_str!("../../../skills/mindframe-visual-director/references/page-plan.md"),
            serde_json::to_string_pretty(&schemars::schema_for!(Storyboard))?,
            serde_json::to_string_pretty(&schemars::schema_for!(Assets))?
        );
        fs::write(dir.join("chat-request.md"), request)?;
        Ok(())
    })?;
    println!("initialized {}; use chat-request.md and source.md in chat; no model API was called", out.display());
    Ok(())
}

pub fn import(project: &Path, from: &Path) -> Result<()> {
    let (_, source) = load_project(project)?;
    ensure!(!project.join("content").try_exists()?, "content already exists; edit its storyboard.json/assets.json directly, or import into a new project");
    let bundle = load_bundle(from, &source)?;
    publish(&project.join("content"), |dir| copy_materials(&bundle, dir))?;
    println!("imported {} scene images; {}", bundle.board.scenes.len(), if bundle.assets.cues.is_empty() { "no supplied timing: subtitles remain untimed" } else { "recording bounds checked; listen to verify actual speech alignment" });
    Ok(())
}

pub fn validate(project: &Path) -> Result<()> {
    let (_, source) = load_project(project)?;
    let bundle = load_bundle(&project.join("content"), &source)?;
    println!("valid local materials: {} scenes, {} images; audio={} timing={}; factual accuracy and visual/speech alignment require human review", bundle.board.scenes.len(), bundle.dimensions.len(), bundle.audio_duration_ms.is_some(), !bundle.assets.cues.is_empty());
    Ok(())
}

pub fn export(project: &Path, out: &Path) -> Result<()> {
    let (manifest, source) = load_project(project)?;
    let bundle = load_bundle(&project.join("content"), &source)?;
    publish(out, |dir| {
        // 1. 只复制显式素材，不导出知识库原文、聊天请求、配置或其他下载文件。
        copy_materials(&bundle, dir)?;
        let plain = bundle.board.scenes.iter().flat_map(|s| &s.narration).cloned().collect::<Vec<_>>().join("\n");
        fs::write(dir.join("subtitles.txt"), format!("{plain}\n"))?;
        if let Some(srt) = bundle.assets.subtitles() { fs::write(dir.join("subtitles.srt"), srt)?; }
        // 2. 输出剪辑清单。没有定时数据时留空时间列，转场只是剪辑建议。
        let mut csv = String::from("\u{feff}scene_id,start,end,image,narration,screen_text,transition\r\n");
        let mut shots = format!("# {}\n\n这是剪辑说明，不是剪映工程。按场景顺序放置图片；转场需在编辑器中设置。\n", bundle.board.title);
        for scene in &bundle.board.scenes {
            let image = bundle.assets.images.iter().find(|i| i.scene_id == scene.id).expect("validated image mapping");
            let cues: Vec<_> = bundle.assets.cues.iter().filter(|c| c.scene_id == scene.id).collect();
            let start = cues.first().map(|c| timestamp(c.start_ms)).unwrap_or_default();
            let end = cues.last().map(|c| timestamp(c.end_ms)).unwrap_or_default();
            let transition = serde_json::to_value(&scene.transition)?.as_str().context("transition must serialize as text")?.to_owned();
            let narration = scene.narration.join(" ");
            csv.push_str(&[scene.id.as_str(), &start, &end, &image.file, &narration, &image.screen_text, &transition].iter().map(|s| csv_cell(s)).collect::<Vec<_>>().join(","));
            csv.push_str("\r\n");
            shots.push_str(&format!("\n## {}\n\n画面：{}\n\n屏幕文字：{}\n\n口播：{}\n\n建议转场：{}\n\n时间：{} → {}\n", scene.id, image.file, image.screen_text, narration, transition, if start.is_empty() { "未定时" } else { &start }, if end.is_empty() { "未定时" } else { &end }));
        }
        fs::write(dir.join("shot-list.csv"), csv)?;
        fs::write(dir.join("storyboard.md"), shots)?;
        write_json(&dir.join("material-report.json"), &json!({
            "schema_version":1,"target":"jianying-materials","preset":manifest.preset,
            "images":bundle.dimensions,"audio_duration_ms":bundle.audio_duration_ms,
            "timing":if bundle.assets.cues.is_empty() { "absent" } else { "user_supplied_recording_bounds_checked" },
            "native_editor_project":false,"source_snapshot_included":false,
            "semantic_accuracy_verified":false,"speech_alignment_verified":false
        }))?;
        fs::write(dir.join("README.txt"), concat!(
            "MindFrame 剪映素材包（不是原生草稿或自动时间线）\n\n",
            "1. 将 images/ 内的图片作为媒体导入，按 shot-list.csv 顺序放置。\n",
            "2. 有 audio/ 时导入已有录音；只有 script.md 时自行录音或在编辑器中配音。\n",
            "3. 只有明确提供了录音和逐句时间点，才会生成 subtitles.srt。\n",
            "   没有 SRT 时使用 subtitles.txt，并在配音确定后制作/识别字幕。\n",
            "4. SRT 导入入口依编辑器版本而异；本包未在你的剪映版本中做 UI 验收。\n",
            "5. storyboard.md / shot-list.csv 是说明，不会自动变成剪映轨道或转场。\n",
            "6. 原图未裁剪或拉伸；按目标比例调整画布，并人工检查字大小与安全区域。\n",
            "7. 已提供的时间点仅通过顺序、文本和录音长度校验，不等于语音对齐已验证。\n",
            "8. 公开前审核内容、图片、读音和素材使用权。包内不含完整原文快照或 API 密钥；关键点中仍保留选取的原文引用。\n"
        ))?;
        Ok(())
    })?;
    println!("exported {}: standard editor materials, not a native Jianying project; no model API was called", out.display());
    Ok(())
}
