//! Human-facing editor pack, derived only from validated manifests. No media renderer.
use std::{fs, path::Path};

use anyhow::{Context, Result};
use mindframe_core::{editor::{narration_review, narration_text, scene_edit_note}, materials::{csv_cell, timestamp}};
use serde_json::{Value, json};

use crate::{materials::Bundle, pipeline::write_json};

fn reference_canvas(preset: &str) -> (u64, u64) {
    match preset {
        "douyin" => (1080, 1920),
        "bilibili" => (1920, 1080),
        _ => unreachable!("validated Project preset"),
    }
}

/// Review warnings, never a promise that a file is publication-quality. Do not silently upscale.
pub(super) fn warnings(bundle: &Bundle, preset: &str) -> Vec<Value> {
    let (target_width, target_height) = reference_canvas(preset);
    let mut warnings = Vec::new();
    for image in bundle.dimensions.iter().chain(bundle.cover_dimensions.iter()) {
        let width = image["width"].as_u64().expect("inspected raster width");
        let height = image["height"].as_u64().expect("inspected raster height");
        let file = image["file"].as_str().expect("normalized raster path");
        if width < target_width || height < target_height {
            warnings.push(json!({"code":"below_reference_canvas","file":file,"message":format!("{file}: {width}×{height}，低于参考画布 {target_width}×{target_height}；检查清晰度，不自动放大。") }));
        }
        if (width * target_height).abs_diff(height * target_width) * 100 > height * target_width {
            warnings.push(json!({"code":"aspect_ratio_review","file":file,"message":format!("{file}: 比例与参考画布不同，请决定裁切/留边；不会拉伸素材。") }));
        }
    }
    for overlay in &bundle.overlay_dimensions {
        if overlay["has_transparency"] == false {
            let file = overlay["file"].as_str().expect("normalized overlay path");
            warnings.push(json!({"code":"opaque_overlay","file":file,"message":format!("{file}: 未检测到透明像素，叠加时会遮住底图；这是提示，不禁止不透明图解。") }));
        }
    }
    if bundle.assets.cover.is_none() {
        warnings.push(json!({"code":"cover_not_provided","message":"未提供封面；可在剪映另做，不会生成占位封面。"}));
    }
    warnings
}

pub(super) fn write(bundle: &Bundle, out: &Path, preset: &str) -> Result<()> {
    // 1. Copy-ready text comes only from current JSON, never from an older script.md export.
    let narration = narration_text(&bundle.board);
    fs::write(out.join("narration.txt"), &narration)?;
    fs::write(out.join("narration-review.md"), narration_review(&bundle.board, bundle.motion.as_ref()))?;
    fs::write(out.join("subtitles.txt"), &narration)?;
    if let Some(srt) = bundle.assets.subtitles() { fs::write(out.join("subtitles.srt"), srt)?; }
    for dir in ["scripts", "screen-text", "edit-notes"] { fs::create_dir(out.join(dir))?; }
    let mut guide = String::from("# 剪映制作指南\n\n这是一份素材包，不是剪映原生工程，也没有自动排轨。\n\n先看素材报告中的尺寸、透明性与检查提醒。将图片/叠加图作为独立素材导入；使用 narration.txt 或 scripts/ 下的纯文本配音。完成配音后制作字幕，再设置重点层、转场与效果。最终时长以实际声音为准。\n\nnarration-review.md 对照来源、口播和画面强调，供全文审阅，不代表自动验收。script.md 是简版审稿；不要把审阅标题和注释一起拿去朗读。screen-text/ 仅含后期重点文字，不是字幕。更换声音或语速后不要沿用旧 SRT。\n\n");
    let mut csv = String::from("\u{feff}scene_id,start,end,image,narration,screen_text,transition\r\n");
    for (index, scene) in bundle.board.scenes.iter().enumerate() {
        let stem = format!("{:03}-{}", index + 1, scene.id);
        let image = bundle.assets.images.iter().find(|image| image.scene_id == scene.id).expect("validated image mapping");
        let plan = bundle.motion.as_ref().and_then(|plan| plan.scenes.iter().find(|p| p.scene_id == scene.id));
        let note = scene_edit_note(scene, image, &bundle.assets.cues, plan, bundle.layers.as_ref());
        fs::write(out.join(format!("scripts/{stem}.txt")), format!("{}\n", scene.narration.join("\n")))?;
        fs::write(out.join(format!("screen-text/{stem}.txt")), &image.screen_text)?;
        fs::write(out.join(format!("edit-notes/{stem}.md")), &note)?;
        guide.push_str(&format!("## {:03} / {}\n\n口播：scripts/{stem}.txt\n\n屏幕文字：screen-text/{stem}.txt\n\n单镜说明：edit-notes/{stem}.md\n\n{}\n", index + 1, scene.id, note));
        let cues: Vec<_> = bundle.assets.cues.iter().filter(|c| c.scene_id == scene.id).collect();
        let start = cues.first().map(|c| timestamp(c.start_ms)).unwrap_or_default();
        let end = cues.last().map(|c| timestamp(c.end_ms)).unwrap_or_default();
        let transition = serde_json::to_value(&scene.transition)?.as_str().context("transition must serialize as text")?.to_owned();
        let narration = scene.narration.join(" ");
        csv.push_str(&[scene.id.as_str(), &start, &end, &image.file, &narration, &image.screen_text, &transition].iter().map(|s| csv_cell(s)).collect::<Vec<_>>().join(","));
        csv.push_str("\r\n");
    }
    fs::write(out.join("edit-guide.md"), &guide)?;
    // Keep the old file name for existing consumers. Both are derived editorial instructions.
    fs::write(out.join("storyboard.md"), &guide)?;
    fs::write(out.join("shot-list.csv"), csv)?;

    // 2. Structural inspection is distinct from actual artistic/editor acceptance.
    let review_warnings = warnings(bundle, preset);
    let (width, height) = reference_canvas(preset);
    write_json(&out.join("material-report.json"), &json!({
        "schema_version":1,"target":"jianying-materials","preset":preset,
        "delivery":"editor_material_pack","final_video_included":false,
        "images":bundle.dimensions,"overlays":bundle.overlay_dimensions,"cover":bundle.cover_dimensions,
        "reference_canvas":{"width":width,"height":height,"basis":"project reference, not verified platform requirements"},
        "warnings":review_warnings,"motion_plan_included":bundle.motion.is_some(),
        "audio_duration_ms":bundle.audio_duration_ms,
        "timing":if bundle.assets.cues.is_empty() { "absent" } else { "user_supplied_recording_bounds_checked" },
        "native_editor_project":false,"source_snapshot_included":false,
        "semantic_accuracy_verified":false,"speech_alignment_verified":false,
        "artwork_quality_verified":false,"editor_ui_verified":false
    }))?;
    fs::write(out.join("README.txt"), concat!(
        "MindFrame 剪映素材包（不是原生草稿或自动时间线）\n\n",
        "从 edit-guide.md 开始；shot-list.csv 保留场景顺序。\n",
        "narration.txt 是全片纯口播；scripts/ 是逐镜纯口播。不要把 script.md 的审稿标题拿去配音。\n",
        "narration-review.md 对照每幕的来源、口播与显式画面强调；用于人工审阅，不是质量评分或配音输入。\n",
        "images/ 是主画面；有 overlays/ 时作为独立叠加位图导入，不会自动排轨。\n",
        "screen-text/ 是可复制的重点文字；edit-notes/ 说明讲到哪句话时出现、保持或替换。\n",
        "配音、字幕时序、BGM、转场、特效与最终导出在剪映完成；具体入口由你的版本决定。\n",
        "没有录音/时间点时只有 subtitles.txt，不编造 SRT。已有 SRT 仅通过范围和文字校验。\n",
        "换声音或语速后请重新做字幕。把成片 MP4 中的烧入字幕换成 SRT 并不能恢复文字图层。\n",
        "图片字节未改；material-report.json 报告清晰度参考、比例、透明性和封面缺失提醒，非发布质量证书。\n",
        "formula 位图不能逐字编辑；保留公式原文并在排版后逐符号校对。\n",
        "完整原文、聊天请求、配置、任意工作笔记和预览 MP4 均不导出。关键点仍包含选取的原文引文。\n"
    ))?;
    Ok(())
}
