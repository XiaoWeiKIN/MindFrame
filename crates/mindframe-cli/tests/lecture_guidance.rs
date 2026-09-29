//! Guidance packaging and existing file behavior only; not animation or speech acceptance.
use std::{fs, io::Cursor, path::Path, process::{Command, Output}};

use image::{DynamicImage, ImageFormat};
use mindframe_core::materials::Assets;
use serde_json::Value;

const DIRECTOR: &str = include_str!("../../../skills/mindframe-visual-director/SKILL.md");
const TEMPLATE: &str = include_str!("../../../skills/mindframe-visual-director/references/page-plan.md");
const SOURCE: &str = include_str!("../../../skills/mindframe-visual-director/examples/source.md");
const BOARD: &str = include_str!("../../../skills/mindframe-visual-director/examples/storyboard.example.json");
const ASSETS: &str = include_str!("../../../skills/mindframe-visual-director/examples/assets.example.json");

fn call(cwd: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mindframe"))
        .current_dir(cwd).args(args).env_clear().env("PATH", "").output().unwrap()
}
fn ok(cwd: &Path, args: &[&str]) {
    let output = call(cwd, args);
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn lecture_handoff_embeds_layers_style_and_truthful_timing_boundaries() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(temp.path().join("article.md"), SOURCE).unwrap();
    ok(temp.path(), &["init", "article.md", "--out", "project"]);
    let request = fs::read_to_string(temp.path().join("project/chat-request.md")).unwrap();
    assert!(request.contains(DIRECTOR));
    assert!(request.contains(TEMPLATE));
    for rule in ["text-first-lecture", "背景层 / 重点层 / 口播层 / 字幕层", "Ocean Depth / 深海星辰", "静态图不等于动态背景", "语义触发", "词级对齐", "当前 import/export 不会自动保留"] {
        assert!(request.contains(rule), "missing embedded rule: {rule}");
    }
    assert!(request.contains("### 逐句触发（没有录音时不用秒数）"));
    assert_eq!(fs::read_to_string(temp.path().join("project/source.md")).unwrap(), SOURCE);
    assert!(!temp.path().join("project/content").exists());
}

#[test]
fn explicit_shared_background_preserves_bytes_and_separate_screen_text() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(temp.path().join("article.md"), SOURCE).unwrap();
    ok(temp.path(), &["init", "article.md", "--out", "project"]);
    let bundle = temp.path().join("bundle");
    fs::create_dir(&bundle).unwrap();
    let mut bytes = Cursor::new(Vec::new());
    DynamicImage::new_rgb8(32, 18).write_to(&mut bytes, ImageFormat::Png).unwrap();
    let bytes = bytes.into_inner();
    fs::write(bundle.join("background.png"), &bytes).unwrap();
    fs::write(bundle.join("storyboard.json"), BOARD).unwrap();
    let mut assets: Assets = serde_json::from_str(ASSETS).unwrap();
    for (index, image) in assets.images.iter_mut().enumerate() {
        image.file = "background.png".into();
        image.screen_text = format!("独立重点层 {}", index + 1);
    }
    fs::write(bundle.join("assets.json"), serde_json::to_vec(&assets).unwrap()).unwrap();
    fs::write(bundle.join("visual-plan.md"), "motion not produced; sentence timing unverified").unwrap();
    fs::write(bundle.join("review.md"), "synthetic static fixture only").unwrap();
    ok(temp.path(), &["import", "project", "--from", "bundle"]);
    ok(temp.path(), &["validate", "project"]);
    ok(temp.path(), &["export", "project", "--out", "export"]);
    let exported: Assets = serde_json::from_slice(&fs::read(temp.path().join("export/assets.json")).unwrap()).unwrap();
    for (before, after) in assets.images.iter().zip(&exported.images) {
        assert_eq!(before.scene_id, after.scene_id);
        assert_eq!(before.screen_text, after.screen_text);
        assert_eq!(fs::read(temp.path().join("export").join(&after.file)).unwrap(), bytes);
    }
    assert_eq!(exported.images.len(), assets.images.len());
    assert!(exported.audio.is_none());
    assert!(exported.cues.is_empty());
    for path in ["subtitles.srt", "visual-plan.md", "review.md", "background.mp4"] {
        assert!(!temp.path().join("export").join(path).exists());
    }
}

#[test]
fn lecture_intent_does_not_extend_assets_schema() {
    for field in ["motion", "layers", "background_video"] {
        let mut value: Value = serde_json::from_str(ASSETS).unwrap();
        value[field] = "not implemented".into();
        assert!(serde_json::from_value::<Assets>(value).is_err(), "unsupported field: {field}");
    }
}
