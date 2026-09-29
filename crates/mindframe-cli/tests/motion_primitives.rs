use std::{fs, io::Cursor, path::Path, process::{Command, Output}};

use image::{DynamicImage, ImageFormat};
use mindframe_core::materials::{Assets, MotionElement, MotionPlan, MotionScenePlan, MotionSlot, MotionStep};

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
fn optional_motion_plan_survives_import_validate_and_export() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(temp.path().join("source.md"), SOURCE).unwrap();
    ok(temp.path(), &["init", "source.md", "--out", "project"]);

    let bundle = temp.path().join("bundle");
    fs::create_dir_all(bundle.join("images")).unwrap();
    fs::write(bundle.join("storyboard.json"), BOARD).unwrap();
    fs::write(bundle.join("assets.json"), ASSETS).unwrap();
    let assets: Assets = serde_json::from_str(ASSETS).unwrap();
    for asset in &assets.images {
        let mut bytes = Cursor::new(Vec::new());
        DynamicImage::new_rgb8(32, 18).write_to(&mut bytes, ImageFormat::Png).unwrap();
        fs::write(bundle.join(&asset.file), bytes.into_inner()).unwrap();
    }

    let plan = MotionPlan {
        schema_version: 1,
        scenes: vec![MotionScenePlan {
            scene_id: "scene-01".into(),
            steps: vec![
                MotionStep {
                    utterance_index: 0,
                    elements: vec![
                        MotionElement::Stat { id: "you".into(), label: "你".into(), value: "18:00".into(), slot: MotionSlot::Left, emphasis: false },
                        MotionElement::Stat { id: "wang".into(), label: "小王".into(), value: "18:00".into(), slot: MotionSlot::Right, emphasis: false },
                    ],
                },
                MotionStep {
                    utterance_index: 1,
                    elements: vec![
                        MotionElement::Stat { id: "you".into(), label: "你".into(), value: "18:00".into(), slot: MotionSlot::Left, emphasis: false },
                        MotionElement::Stat { id: "wang".into(), label: "小王".into(), value: "19:00".into(), slot: MotionSlot::Right, emphasis: true },
                        MotionElement::Relation { id: "shift".into(), from: "you".into(), to: "wang".into(), label: Some("相对变化".into()), slot: MotionSlot::Center },
                    ],
                },
            ],
        }],
    };
    fs::write(bundle.join("motion.json"), serde_json::to_vec_pretty(&plan).unwrap()).unwrap();

    ok(temp.path(), &["import", "project", "--from", "bundle"]);
    ok(temp.path(), &["validate", "project"]);
    ok(temp.path(), &["export", "project", "--out", "export"]);

    let imported: MotionPlan = serde_json::from_slice(&fs::read(temp.path().join("project/content/motion.json")).unwrap()).unwrap();
    imported.validate(&serde_json::from_str(BOARD).unwrap()).unwrap();
    let exported: MotionPlan = serde_json::from_slice(&fs::read(temp.path().join("export/motion.json")).unwrap()).unwrap();
    assert_eq!(serde_json::to_value(imported).unwrap(), serde_json::to_value(exported).unwrap());

    let request = fs::read_to_string(temp.path().join("project/chat-request.md")).unwrap();
    assert!(request.contains("## Motion JSON Schema（可选）"));
    assert!(request.contains("\"matrix\""));
    assert!(request.contains("\"formula\""));
}
