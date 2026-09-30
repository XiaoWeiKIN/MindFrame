//! Local material import/export. No model client, TTS, browser or media subprocess.
use std::{fs, io::Cursor, path::{Path, PathBuf}};

use anyhow::{Context, Result, ensure};
use image::{GenericImageView, ImageFormat, ImageReader, Limits};
use mindframe_core::{Storyboard, editor::Layers, materials::{Assets, MotionPlan, Project}};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::pipeline::{read_source, write_json};

pub(super) struct Bundle {
    pub board: Storyboard,
    pub assets: Assets,
    pub motion: Option<MotionPlan>,
    pub layers: Option<Layers>,
    pub files: Vec<(PathBuf, String)>,
    pub dimensions: Vec<Value>,
    pub overlay_dimensions: Vec<Value>,
    pub cover_dimensions: Option<Value>,
    pub audio_duration_ms: Option<u64>,
}

// Publish a complete directory, never a half-imported project. This is a single-writer CLI.
pub(super) fn publish(out: &Path, write: impl FnOnce(&Path) -> Result<()>) -> Result<()> {
    ensure!(out.symlink_metadata().is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound), "destination already exists or cannot be inspected: {}", out.display());
    let parent = out.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    fs::create_dir_all(parent)?;
    let staging = tempfile::Builder::new().prefix(".mindframe-").tempdir_in(parent)?;
    write(staging.path())?;
    ensure!(out.symlink_metadata().is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound), "destination appeared during preparation: {}", out.display());
    fs::rename(staging.path(), out).with_context(|| format!("publish {}", out.display()))?;
    Ok(())
}

// Relative paths are already contract-validated; reject symlinks at each filesystem component.
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

fn optional_json<T: DeserializeOwned>(root: &Path, name: &str) -> Result<Option<T>> {
    match root.join(name).symlink_metadata() {
        Ok(_) => read_json(root, name).map(Some),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("inspect optional material {name}")),
    }
}

fn inspect_image(path: &Path) -> Result<(String, u32, u32, bool)> {
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
    // Report real transparent pixels, not just a .png extension or an alpha-channel claim.
    let transparent = decoded.color().has_alpha() && decoded.pixels().any(|(_, _, pixel)| pixel.0[3] < 255);
    let canonical = if format == ImageFormat::Jpeg { "jpg" } else { extension.as_str() };
    Ok((canonical.to_owned(), decoded.width(), decoded.height(), transparent))
}

fn inspect_audio(path: &Path) -> Result<u64> {
    let mut reader = hound::WavReader::open(path).context("audio must be PCM 16-bit WAV; convert other formats explicitly first")?;
    let spec = reader.spec();
    ensure!(spec.sample_format == hound::SampleFormat::Int && spec.bits_per_sample == 16 && (1..=2).contains(&spec.channels) && spec.sample_rate > 0, "audio must be mono/stereo PCM 16-bit WAV");
    let frames = u64::from(reader.duration());
    let duration = frames * 1000 / u64::from(spec.sample_rate);
    ensure!(duration > 0 && duration <= 3_600_000, "recording must be nonempty and at most one hour");
    for sample in reader.samples::<i16>() { sample.context("invalid or truncated WAV samples")?; }
    Ok(duration)
}

pub(super) fn load_bundle(root: &Path, source: &str) -> Result<Bundle> {
    let board: Storyboard = read_json(root, "storyboard.json")?;
    board.validate(source)?;
    let mut assets: Assets = read_json(root, "assets.json")?;
    assets.validate(&board)?;
    let motion: Option<MotionPlan> = optional_json(root, "motion.json")?;
    if let Some(plan) = &motion { plan.validate(&board)?; }
    let mut layers: Option<Layers> = optional_json(root, "layers.json")?;
    if let Some(layers) = &layers { layers.validate(&board)?; }
    let mut files = Vec::new();
    let mut dimensions = Vec::new();
    let mut overlay_dimensions = Vec::new();
    let mut cover_dimensions = None;

    // 1. Explicit base-image mapping and actual bytes. Never flatten or regenerate assets.
    for (index, scene) in board.scenes.iter().enumerate() {
        let image = assets.images.iter_mut().find(|image| image.scene_id == scene.id).expect("validated complete scene mapping");
        let path = regular_file(root, &image.file, 50 * 1024 * 1024)?;
        let (extension, width, height, transparent) = inspect_image(&path)?;
        image.file = format!("images/{:03}-{}.{}", index + 1, scene.id, extension);
        dimensions.push(json!({"scene_id":scene.id,"file":image.file,"width":width,"height":height,"has_transparency":transparent}));
        files.push((path, image.file.clone()));
    }
    if let Some(cover) = &mut assets.cover {
        let path = regular_file(root, cover, 50 * 1024 * 1024)?;
        let (extension, width, height, transparent) = inspect_image(&path)?;
        *cover = format!("cover.{extension}");
        cover_dimensions = Some(json!({"file":cover,"width":width,"height":height,"has_transparency":transparent}));
        files.push((path, cover.clone()));
    }
    // 2. Optional overlays are independent files with explicit scene/utterance references.
    if let Some(layers) = &mut layers {
        for (index, overlay) in layers.overlays.iter_mut().enumerate() {
            let path = regular_file(root, &overlay.file, 50 * 1024 * 1024)?;
            let (extension, width, height, transparent) = inspect_image(&path)?;
            overlay.file = format!("overlays/{:03}-{}-{}.{}", index + 1, overlay.scene_id, overlay.id, extension);
            overlay_dimensions.push(json!({"scene_id":overlay.scene_id,"id":overlay.id,"file":overlay.file,"width":width,"height":height,"has_transparency":transparent}));
            files.push((path, overlay.file.clone()));
        }
    }
    // 3. Optional recording/timing. The normal pack is valid without either.
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
    Ok(Bundle { board, assets, motion, layers, files, dimensions, overlay_dimensions, cover_dimensions, audio_duration_ms })
}

pub(super) fn load_project(project: &Path) -> Result<(Project, String)> {
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
    if let Some(motion) = &bundle.motion { write_json(&out.join("motion.json"), motion)?; }
    if let Some(layers) = &bundle.layers { write_json(&out.join("layers.json"), layers)?; }
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
        let request = format!(
            "{}\n\n目标布局：{}。\n\n## 视觉导演工作流（完整内嵌）\n{}\n\n## 逐页工作表（完整内嵌）\n{}\n\n## 知识讲解口播规则（完整内嵌）\n{}\n\n## 主图与母版评审（完整内嵌）\n{}\n\n## Storyboard JSON Schema\n```json\n{}\n```\n\n## Assets JSON Schema\n```json\n{}\n```\n\n## Motion JSON Schema（可选）\n```json\n{}\n```\n\n## Layers JSON Schema（可选）\n```json\n{}\n```\n",
            include_str!("../../../prompts/chat-authoring.md"), project.preset,
            include_str!("../../../skills/mindframe-visual-director/SKILL.md"),
            include_str!("../../../skills/mindframe-visual-director/references/page-plan.md"),
            include_str!("../../../skills/mindframe-author/references/narration.md"),
            include_str!("../../../skills/mindframe-visual-director/references/visual-review.md"),
            serde_json::to_string_pretty(&schemars::schema_for!(Storyboard))?,
            serde_json::to_string_pretty(&schemars::schema_for!(Assets))?,
            serde_json::to_string_pretty(&schemars::schema_for!(MotionPlan))?,
            serde_json::to_string_pretty(&schemars::schema_for!(Layers))?
        );
        fs::write(dir.join("chat-request.md"), request)?;
        Ok(())
    })?;
    println!("initialized {}; use chat-request.md and source.md in chat; deliver an editor material pack, not a final video; no model API was called", out.display());
    Ok(())
}

pub fn import(project: &Path, from: &Path) -> Result<()> {
    let (_, source) = load_project(project)?;
    ensure!(!project.join("content").try_exists()?, "content already exists; edit its storyboard.json/assets.json directly, or import into a new project");
    let bundle = load_bundle(from, &source)?;
    publish(&project.join("content"), |dir| copy_materials(&bundle, dir))?;
    println!("imported {} scene images, {} overlays; motion_plan={}; {}", bundle.board.scenes.len(), bundle.overlay_dimensions.len(), bundle.motion.is_some(), if bundle.assets.cues.is_empty() { "no supplied timing: subtitles remain untimed" } else { "recording bounds checked; listen to verify actual speech alignment" });
    Ok(())
}

pub fn validate(project: &Path) -> Result<()> {
    let (manifest, source) = load_project(project)?;
    let bundle = load_bundle(&project.join("content"), &source)?;
    println!("valid local materials: {} scenes, {} overlays; audio={} timing={}; this is not an artwork or publishing-quality approval", bundle.board.scenes.len(), bundle.overlay_dimensions.len(), bundle.audio_duration_ms.is_some(), !bundle.assets.cues.is_empty());
    for warning in crate::editor_export::warnings(&bundle, &manifest.preset) {
        println!("review: {}", warning["message"].as_str().expect("generated warning text"));
    }
    Ok(())
}

pub fn export(project: &Path, out: &Path) -> Result<()> {
    let (manifest, source) = load_project(project)?;
    let bundle = load_bundle(&project.join("content"), &source)?;
    publish(out, |dir| {
        copy_materials(&bundle, dir)?;
        crate::editor_export::write(&bundle, dir, &manifest.preset)
    })?;
    println!("exported {}: editor material pack; voice, captions, effects and final editing belong in Jianying; no model API or media subprocess was called", out.display());
    Ok(())
}
