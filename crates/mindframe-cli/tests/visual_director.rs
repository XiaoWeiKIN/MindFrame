//! Tests prove bundled guidance and file contracts, not generated artwork quality.
use std::{fs, io::Cursor, path::Path, process::{Command, Output}};

use image::{DynamicImage, ImageFormat};
use mindframe_core::{Storyboard, Visual, materials::Assets};
use serde_json::Value;

const DIRECTOR: &str = include_str!("../../../skills/mindframe-visual-director/SKILL.md");
const TEMPLATE: &str = include_str!("../../../skills/mindframe-visual-director/references/page-plan.md");
const VISUAL_REVIEW: &str = include_str!("../../../skills/mindframe-visual-director/references/visual-review.md");
const AUTHOR: &str = include_str!("../../../prompts/chat-authoring.md");
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
fn init_embeds_complete_visual_guidance_outside_the_repository() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(temp.path().join("article.md"), SOURCE).unwrap();
    ok(temp.path(), &["init", "article.md", "--out", "project", "--preset", "douyin"]);
    let request = fs::read_to_string(temp.path().join("project/chat-request.md")).unwrap();
    for guidance in [AUTHOR, DIRECTOR, TEMPLATE, VISUAL_REVIEW] {
        assert!(request.contains(guidance), "init must embed canonical guidance, not just repository links");
    }
    assert!(request.contains("目标布局：douyin"));
    assert!(request.contains("## Storyboard JSON Schema"));
    assert!(request.contains("## Assets JSON Schema"));
    assert_eq!(fs::read_to_string(temp.path().join("project/source.md")).unwrap(), SOURCE);
    assert!(!temp.path().join("project/content").exists());
    // Reinstalling/reinitializing must not overwrite an existing project's handoff.
    assert!(!call(temp.path(), &["init", "article.md", "--out", "project"]).status.success());
    assert_eq!(fs::read_to_string(temp.path().join("project/chat-request.md")).unwrap(), request);
}

#[test]
fn editorial_example_uses_existing_contract_and_preserves_formula() {
    let board: Storyboard = serde_json::from_str(BOARD).unwrap();
    board.validate(SOURCE).unwrap();
    let assets: Assets = serde_json::from_str(ASSETS).unwrap();
    assets.validate(&board).unwrap();
    assert!(assets.subtitles().is_none());
    assert!(assets.audio.is_none());
    assert_eq!(assets.images.len(), 3);
    assert!(assets.images.iter().all(|i| i.screen_text.is_empty()));
    let formula = r"S_{t+1} = F(S_t, A_t, E_t, \epsilon_t)";
    assert!(SOURCE.contains(formula));
    match &board.scenes[2].visual {
        Visual::Image { prompt } => assert!(prompt.contains(formula)),
        _ => panic!("formula intent must use an existing visual type"),
    }
    // Unsupported director metadata must remain in Markdown sidecars, not creep into schema v1.
    let mut value: Value = serde_json::from_str(BOARD).unwrap();
    value["scenes"][0]["importance"] = "high".into();
    assert!(serde_json::from_value::<Storyboard>(value).is_err());
    let mut value: Value = serde_json::from_str(ASSETS).unwrap();
    value["images"][0]["P0"] = "title".into();
    assert!(serde_json::from_value::<Assets>(value).is_err());
}

#[test]
fn editorial_fixture_imports_exports_and_does_not_publish_working_notes() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(temp.path().join("article.md"), SOURCE).unwrap();
    ok(temp.path(), &["init", "article.md", "--out", "project"]);
    let bundle = temp.path().join("bundle");
    fs::create_dir_all(bundle.join("images")).unwrap();
    fs::write(bundle.join("storyboard.json"), BOARD).unwrap();
    fs::write(bundle.join("assets.json"), ASSETS).unwrap();
    fs::write(bundle.join("visual-plan.md"), "draft, not public export").unwrap();
    fs::write(bundle.join("review.md"), "all artwork unverified").unwrap();
    let assets: Assets = serde_json::from_str(ASSETS).unwrap();
    // Deliberately tiny synthetic fixtures: not generated art or mobile-readability evidence.
    for asset in &assets.images {
        let mut bytes = Cursor::new(Vec::new());
        DynamicImage::new_rgb8(18, 32).write_to(&mut bytes, ImageFormat::Png).unwrap();
        fs::write(bundle.join(&asset.file), bytes.into_inner()).unwrap();
    }
    ok(temp.path(), &["import", "project", "--from", "bundle"]);
    ok(temp.path(), &["validate", "project"]);
    ok(temp.path(), &["export", "project", "--target", "jianying", "--out", "export"]);
    for name in ["visual-plan.md", "review.md", "source.md", "chat-request.md", "subtitles.srt"] {
        assert!(!temp.path().join("export").join(name).exists(), "not exported: {name}");
    }
    let exported: Assets = serde_json::from_slice(&fs::read(temp.path().join("export/assets.json")).unwrap()).unwrap();
    for (before, after) in assets.images.iter().zip(&exported.images) {
        assert_eq!(before.scene_id, after.scene_id);
        assert_eq!(fs::read(bundle.join(&before.file)).unwrap(), fs::read(temp.path().join("export").join(&after.file)).unwrap());
        assert!(after.screen_text.is_empty());
    }
    assert!(fs::read_to_string(temp.path().join("export/script.md")).unwrap().contains("反身性"));
}

#[test]
fn portable_skill_references_and_delivery_guards_are_present() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../skills/mindframe-visual-director");
    for relative in ["references/page-plan.md", "references/sources.md", "examples/walkthrough.md", "examples/source.md", "examples/storyboard.example.json", "examples/assets.example.json"] {
        assert!(root.join(relative).is_file(), "missing bundled reference: {relative}");
    }
    assert!(DIRECTOR.starts_with("---\nname: mindframe-visual-director\n"));
    for guard in ["原文引用 / 忠实改写", "P0", "P1", "P2", "禁止把 8 页或 9 页画成同一张九宫格", "Future = P(...)", "pass / revise / unverified", "当前 import/export 不会自动保留"] {
        assert!(DIRECTOR.contains(guard), "missing documented regression guard: {guard}");
    }
    // These assertions prevent missing guidance. They do not prove an agent obeys it or an image passes.
}
