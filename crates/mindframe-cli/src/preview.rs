//! Optional legacy Motion preview. Never invoked by editor import/validate/export.
use std::{fs, path::Path, process::Command};

use anyhow::{Context, Result, ensure};
use mindframe_core::{Scene, Visual, FPS};
use serde_json::{Value, json};

use crate::{materials::{Bundle, load_bundle, load_project, publish}, pipeline::write_json};

fn frame_at_or_after(ms: u64) -> Result<u32> {
    let frames = ms.checked_mul(u64::from(FPS))
        .and_then(|value| value.checked_add(999))
        .context("motion timestamp overflow")? / 1000;
    ensure!(frames <= u64::from(FPS) * 60 * 60, "motion timestamp exceeds one hour");
    Ok(u32::try_from(frames)?)
}

fn emphasis_text(scene: &Scene, screen_text: &str) -> String {
    if !screen_text.trim().is_empty() { return screen_text.to_owned(); }
    match &scene.visual {
        Visual::Title { text } => text.clone(),
        Visual::KeyPoint { title, .. } => title.clone(),
        _ => String::new(),
    }
}

fn default_motion_elements(scene: &Scene, screen_text: &str) -> Vec<Value> {
    let text = emphasis_text(scene, screen_text);
    if text.trim().is_empty() { Vec::new() } else {
        vec![json!({"type":"text","id":"headline","text":text,"slot":"center","emphasis":true})]
    }
}

fn motion_steps(bundle: &Bundle, scene: &Scene, scene_start: u32, scene_end: u32, screen_text: &str) -> Result<Vec<Value>> {
    let Some(scene_plan) = bundle.motion.as_ref().and_then(|plan| plan.scenes.iter().find(|candidate| candidate.scene_id == scene.id)) else {
        return Ok(vec![json!({
            "utterance_index":0,"start_frame":scene_start,"duration_frames":scene_end-scene_start,
            "elements":default_motion_elements(scene, screen_text)
        })]);
    };
    let mut starts = Vec::with_capacity(scene_plan.steps.len());
    for step in &scene_plan.steps {
        let start = if step.utterance_index == 0 { scene_start } else {
            let cue = bundle.assets.cues.iter().find(|cue| cue.scene_id == scene.id && cue.utterance_index == step.utterance_index)
                .expect("validated cues cover every motion step");
            frame_at_or_after(cue.start_ms)?
        };
        starts.push(start);
    }
    let mut output = Vec::with_capacity(scene_plan.steps.len());
    for (index, step) in scene_plan.steps.iter().enumerate() {
        let start_frame = starts[index];
        let end_frame = starts.get(index + 1).copied().unwrap_or(scene_end);
        ensure!(end_frame > start_frame, "motion step for {} collapses below one frame; adjust cue timing", scene.id);
        output.push(json!({"utterance_index":step.utterance_index,"start_frame":start_frame,"duration_frames":end_frame-start_frame,"elements":step.elements}));
    }
    Ok(output)
}

fn motion_input(bundle: &Bundle, preset: &str) -> Result<Value> {
    let audio = bundle.assets.audio.as_deref().context("motion rendering requires an actual WAV recording")?;
    ensure!(audio == "audio/narration.wav", "motion audio must use the normalized imported recording");
    ensure!(!bundle.assets.cues.is_empty(), "motion rendering requires reviewed cues for every narration utterance");
    let duration_ms = bundle.audio_duration_ms.context("motion rendering requires inspected audio")?;
    let duration_frames = frame_at_or_after(duration_ms)?;
    ensure!(duration_frames > 0, "motion recording is empty");
    let (width, height) = match preset {
        "bilibili" => (1920, 1080),
        "douyin" => (1080, 1920),
        _ => anyhow::bail!("motion preset must be douyin or bilibili"),
    };
    let mut scenes = Vec::with_capacity(bundle.board.scenes.len());
    for (index, scene) in bundle.board.scenes.iter().enumerate() {
        let image = bundle.assets.images.iter().find(|image| image.scene_id == scene.id).expect("validated complete scene image mapping");
        let first_cue = bundle.assets.cues.iter().find(|cue| cue.scene_id == scene.id).expect("validated cues cover every scene");
        let start_frame = if index == 0 { 0 } else { frame_at_or_after(first_cue.start_ms)? };
        let end_frame = if let Some(next) = bundle.board.scenes.get(index + 1) {
            let next_cue = bundle.assets.cues.iter().find(|cue| cue.scene_id == next.id).expect("validated cues cover every scene");
            frame_at_or_after(next_cue.start_ms)?
        } else { duration_frames };
        ensure!(end_frame > start_frame, "scene {} collapses below one video frame; adjust cue timing", scene.id);
        let steps = motion_steps(bundle, scene, start_frame, end_frame, &image.screen_text)?;
        scenes.push(json!({"id":scene.id,"start_frame":start_frame,"duration_frames":end_frame-start_frame,"image":image.file,"screen_text":emphasis_text(scene, &image.screen_text),"steps":steps}));
    }
    let mut subtitles = Vec::with_capacity(bundle.assets.cues.len());
    for cue in &bundle.assets.cues {
        let start_frame = frame_at_or_after(cue.start_ms)?;
        let end_frame = frame_at_or_after(cue.end_ms)?;
        ensure!(end_frame > start_frame, "subtitle for {} is shorter than one video frame", cue.scene_id);
        ensure!(end_frame <= duration_frames, "subtitle timing exceeds rendered audio");
        subtitles.push(json!({"scene_id":cue.scene_id,"utterance_index":cue.utterance_index,"start_frame":start_frame,"end_frame":end_frame,"text":cue.text}));
    }
    Ok(json!({"schema_version":1,"fps":FPS,"width":width,"height":height,"duration_frames":duration_frames,"title":bundle.board.title,"audio":audio,"scenes":scenes,"subtitles":subtitles}))
}

pub fn render(project: &Path, out: &Path, preset: &str, renderer: &Path, scale: f64) -> Result<()> {
    ensure!(scale.is_finite() && (0.1..=1.0).contains(&scale), "scale must be in 0.1..=1.0");
    let (_, source) = load_project(project)?;
    let bundle = load_bundle(&project.join("content"), &source)?;
    ensure!(bundle.layers.is_none(), "preview does not composite layers.json; use export --target jianying for the complete layered material pack");
    let input = motion_input(&bundle, preset)?;
    let renderer = renderer.canonicalize().with_context(|| format!("motion renderer directory not found: {}", renderer.display()))?;
    let script = renderer.join("motion-render.mjs");
    ensure!(script.is_file(), "motion renderer entrypoint missing: {}", script.display());
    publish(out, |dir| {
        eprintln!("Structural preview only; this is not a final publication video. Use export for Jianying materials.");
        let check = Command::new("node").arg(&script).arg("--check").output().context("start Node motion renderer check")?;
        ensure!(check.status.success(), "motion renderer preflight failed: {}", String::from_utf8_lossy(&check.stderr).trim());
        let assets_dir = dir.join("assets");
        fs::create_dir_all(&assets_dir)?;
        for (source, relative) in &bundle.files {
            if !relative.starts_with("images/") && relative != "audio/narration.wav" { continue; }
            let destination = assets_dir.join(relative);
            if let Some(parent) = destination.parent() { fs::create_dir_all(parent)?; }
            fs::copy(source, destination)?;
        }
        write_json(&dir.join("motion-input.json"), &input)?;
        if let Some(motion) = &bundle.motion { write_json(&dir.join("motion.json"), motion)?; }
        if let Some(srt) = bundle.assets.subtitles() { fs::write(dir.join("subtitles.srt"), srt)?; }
        let rendered = Command::new("node").arg(&script).arg(dir).arg(scale.to_string()).output().context("start Node motion renderer")?;
        ensure!(rendered.status.success(), "motion renderer failed: {}", String::from_utf8_lossy(&rendered.stderr).trim());
        ensure!(dir.join("motion.mp4").is_file() && dir.join("cover.png").is_file(), "motion renderer did not publish expected media");
        write_json(&dir.join("preview-report.json"), &json!({"schema_version":1,"purpose":"structural_preview","publish_ready":false,"media":"motion.mp4","overlay_compositing_supported":false,"speech_alignment_verified":false}))?;
        let warning = "MindFrame 结构预览 / STRUCTURAL PREVIEW ONLY\n\n不是正式发布成片，也不是剪映工程。motion.mp4/cover.png 保留原输出名以兼容旧命令。\n画面只用于内部核对内容结构与已提供的句级时间；不代表自然配音、艺术质量或同步实听验收。\n正式制作请使用 export --target jianying 的素材包。\n";
        fs::write(dir.join("PREVIEW.txt"), warning)?;
        fs::write(dir.join("README.txt"), warning)?;
        Ok(())
    })?;
    println!("structural preview saved to {}; preset={preset}, scale={scale}; not a final publication video; no model API was called", out.display());
    Ok(())
}
