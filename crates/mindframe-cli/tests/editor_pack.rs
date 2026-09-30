//! Real CLI regression fixtures: no model keys, media executables or artistic-quality claims.
use std::{fs, io::Cursor, path::{Path, PathBuf}, process::{Command, Output}};

use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use mindframe_core::{Storyboard, editor::Layers};
use serde_json::{Value, json};
use tempfile::TempDir;

const SOURCE: &str = "# 原创测试\n有限干预影响后续反馈。\n";
const FORMULA: &str = r"S_{t+1} = F(S_t, A_t, E_t, \epsilon_t)";

fn call(cwd: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mindframe")).current_dir(cwd).args(args).env_clear().env("PATH", "").output().unwrap()
}
fn ok(cwd: &Path, args: &[&str]) -> Output {
    let output = call(cwd, args);
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    output
}
fn save(path: &Path, value: &Value) { fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap(); }
fn read(path: &Path) -> Value { serde_json::from_slice(&fs::read(path).unwrap()).unwrap() }
fn stat(id: &str, value: &str, emphasis: bool) -> Value {
    json!({"type":"stat","id":id,"label":id,"value":value,"slot":if id=="you" {"left"} else {"right"},"emphasis":emphasis})
}

fn setup(with_layers: bool) -> (TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    fs::write(root.join("source.md"), SOURCE).unwrap();
    ok(root, &["init", "source.md", "--out", "project"]);
    let bundle = root.join("bundle");
    fs::create_dir(&bundle).unwrap();
    let mut bytes = Cursor::new(Vec::new());
    DynamicImage::new_rgb8(32, 18).write_to(&mut bytes, ImageFormat::Png).unwrap();
    fs::write(bundle.join("base.png"), bytes.into_inner()).unwrap();
    save(&bundle.join("storyboard.json"), &json!({
        "schema_version":1,"title":"标题不应被朗读","summary":"原创合成协议测试。",
        "key_points":[{"text":"有限干预影响反馈。","sources":[{"line_start":2,"line_end":2,"quote":"有限干预影响后续反馈。"}]}],
        "scenes":[
            {"id":"logic","narration":["先看当前条件。","接着调整一次行动。","再观察反馈。"],"point_refs":[1],"visual":{"type":"key_point","title":"路径依赖","body":"这里是画面意图，不是额外口播。"},"transition":"fade"},
            {"id":"formula","narration":["这是示意模型，不是预测定律。"],"point_refs":[1],"visual":{"type":"code","language":"latex","code":FORMULA},"transition":"cut"}
        ]
    }));
    save(&bundle.join("assets.json"), &json!({"schema_version":1,"images":[
        {"scene_id":"logic","file":"base.png","screen_text":"过去影响现在\n≠ 过去决定未来"},
        {"scene_id":"formula","file":"base.png","screen_text":FORMULA}
    ]}));
    save(&bundle.join("motion.json"), &json!({"schema_version":1,"scenes":[
        {"scene_id":"logic","steps":[
            {"utterance_index":0,"elements":[stat("you","18:00",false),stat("wang","18:00",false),{"type":"relation","id":"edge","from":"you","to":"wang","label":"比较","slot":"center"}]},
            {"utterance_index":1,"elements":[stat("you","18:00",false),stat("wang","19:00",true),{"type":"text","id":"point","text":"只更新发生变化的信息","slot":"top","emphasis":true}]},
            {"utterance_index":2,"elements":[]}
        ]},
        {"scene_id":"formula","steps":[{"utterance_index":0,"elements":[
            {"type":"formula","id":"f","text":FORMULA,"highlight":"A_t","note":"保留约束条件","slot":"center"},
            {"type":"matrix","id":"m","title":"合成决策表","headers":["条件","选择"],"rows":[["A","B"]],"slot":"bottom"}
        ]}]}
    ]}));
    fs::write(bundle.join("script.md"), "DO-NOT-READ: stale derivative").unwrap();
    fs::write(bundle.join("review.md"), "private draft").unwrap();
    fs::write(bundle.join("motion.mp4"), "not a real video; must not be copied").unwrap();
    if with_layers {
        let mut overlays = Vec::new();
        for (i, (name, format)) in [("alpha.png", ImageFormat::Png), ("alpha.webp", ImageFormat::WebP), ("opaque.jpg", ImageFormat::Jpeg)].into_iter().enumerate() {
            let image = if format == ImageFormat::Jpeg { DynamicImage::new_rgb8(16, 16) } else {
                DynamicImage::ImageRgba8(RgbaImage::from_pixel(16, 16, Rgba([200, 200, 200, 128])))
            };
            let mut bytes = Cursor::new(Vec::new());
            image.write_to(&mut bytes, format).unwrap();
            fs::write(bundle.join(name), bytes.into_inner()).unwrap();
            overlays.push(json!({"scene_id":"logic","id":format!("layer-{i}"),"file":name,"label":"独立图解","utterance_index":1}));
        }
        save(&bundle.join("layers.json"), &json!({"schema_version":1,"overlays":overlays}));
    }
    (temp, bundle)
}

#[test]
fn layered_pack_has_clean_scripts_real_layers_and_semantic_editing_notes_without_audio() {
    let (temp, bundle) = setup(true);
    let root = temp.path();
    ok(root, &["import", "project", "--from", "bundle"]);
    ok(root, &["validate", "project"]);
    ok(root, &["export", "project", "--out", "export"]);
    let out = root.join("export");
    let narration = fs::read_to_string(out.join("narration.txt")).unwrap();
    assert_eq!(narration, "先看当前条件。\n接着调整一次行动。\n再观察反馈。\n这是示意模型，不是预测定律。\n");
    assert!(!narration.contains("标题"));
    assert!(!narration.contains("DO-NOT-READ"));
    assert_eq!(fs::read_to_string(out.join("scripts/001-logic.txt")).unwrap(), "先看当前条件。\n接着调整一次行动。\n再观察反馈。\n");
    assert_eq!(fs::read_to_string(out.join("screen-text/002-formula.txt")).unwrap(), FORMULA);
    let guide = fs::read_to_string(out.join("edit-guide.md")).unwrap();
    for fragment in ["narration[1]", "新增 you", "保持 you", "更新 wang", "移除 wang", "18:00", "19:00", "关系：you → wang", "矩阵：合成决策表", FORMULA, "强调：A_t", "时间：未定时"] {
        assert!(guide.contains(fragment), "missing {fragment}");
    }
    assert!(!guide.contains("00:00:"));
    let imported = read(&root.join("project/content/layers.json"));
    let exported = read(&out.join("layers.json"));
    assert_eq!(imported, exported);
    let original = read(&bundle.join("layers.json"));
    for (before, after) in original["overlays"].as_array().unwrap().iter().zip(exported["overlays"].as_array().unwrap()) {
        assert_eq!(fs::read(bundle.join(before["file"].as_str().unwrap())).unwrap(), fs::read(out.join(after["file"].as_str().unwrap())).unwrap());
    }
    let report = read(&out.join("material-report.json"));
    assert_eq!(report["timing"], "absent");
    assert_eq!(report["overlays"][0]["has_transparency"], true);
    assert_eq!(report["overlays"][1]["has_transparency"], true);
    assert_eq!(report["overlays"][2]["has_transparency"], false);
    assert_eq!(report["final_video_included"], false);
    assert_eq!(report["artwork_quality_verified"], false);
    for code in ["below_reference_canvas", "aspect_ratio_review", "opaque_overlay", "cover_not_provided"] {
        assert!(report["warnings"].as_array().unwrap().iter().any(|w| w["code"] == code));
    }
    for path in ["subtitles.srt", "audio", "source.md", "review.md", "motion.mp4", "preview-report.json"] { assert!(!out.join(path).exists(), "unexpected {path}"); }
}

#[test]
fn old_minimum_pack_stays_usable_and_new_exports_follow_current_json() {
    let (temp, bundle) = setup(false);
    let root = temp.path();
    fs::remove_file(bundle.join("motion.json")).unwrap();
    ok(root, &["import", "project", "--from", "bundle"]);
    ok(root, &["export", "project", "--out", "v1"]);
    assert!(!root.join("v1/layers.json").exists());
    assert!(!root.join("v1/motion.json").exists());
    let mut board = read(&root.join("project/content/storyboard.json"));
    board["scenes"][0]["narration"][0] = "改稿后这句才是编辑源。".into();
    save(&root.join("project/content/storyboard.json"), &board);
    let mut assets = read(&root.join("project/content/assets.json"));
    assets["images"][0]["screen_text"] = "新的屏幕文字".into();
    save(&root.join("project/content/assets.json"), &assets);
    fs::write(root.join("project/content/script.md"), "stale generated file").unwrap();
    ok(root, &["export", "project", "--out", "v2"]);
    assert!(fs::read_to_string(root.join("v2/narration.txt")).unwrap().starts_with("改稿后这句"));
    assert!(fs::read_to_string(root.join("v2/edit-notes/001-logic.md")).unwrap().contains("改稿后这句"));
    assert_eq!(fs::read_to_string(root.join("v2/screen-text/001-logic.txt")).unwrap(), "新的屏幕文字");
    let before = fs::read(root.join("v1/narration.txt")).unwrap();
    assert!(!call(root, &["export", "project", "--out", "v1"]).status.success());
    assert_eq!(fs::read(root.join("v1/narration.txt")).unwrap(), before);
}

#[test]
fn invalid_layer_contracts_and_files_fail_without_partial_publication() {
    for case in 0..11 {
        let (temp, bundle) = setup(true);
        let root = temp.path();
        let mut layers = read(&bundle.join("layers.json"));
        match case {
            0 => layers["schema_version"] = 2.into(),
            1 => layers["overlays"][0]["scene_id"] = "missing".into(),
            2 => layers["overlays"][0]["utterance_index"] = 99.into(),
            3 => layers["overlays"][0]["file"] = "../alpha.png".into(),
            4 => layers["overlays"][0]["id"] = "../escape".into(),
            5 => layers["overlays"][1]["id"] = "layer-0".into(),
            6 => layers["overlays"][0]["script"] = "not allowed".into(),
            7 => layers["overlays"][0]["label"] = "bad\nlabel".into(),
            8 => { fs::remove_file(bundle.join("alpha.png")).unwrap(); },
            9 => { fs::copy(bundle.join("opaque.jpg"), bundle.join("alpha.png")).unwrap(); },
            _ => layers["overlays"][0]["file"] = "https://host/a.png".into(),
        }
        save(&bundle.join("layers.json"), &layers);
        assert!(!call(root, &["import", "project", "--from", "bundle"]).status.success(), "case {case}");
        assert!(!root.join("project/content").exists(), "case {case}");
    }
}

#[test]
fn layer_counts_and_unknown_manifest_fields_are_strict() {
    let (_temp, bundle) = setup(true);
    let board: Storyboard = serde_json::from_value(read(&bundle.join("storyboard.json"))).unwrap();
    let mut value = read(&bundle.join("layers.json"));
    value["overlays"] = json!([]);
    assert!(serde_json::from_value::<Layers>(value.clone()).unwrap().validate(&board).is_err());
    value["overlays"] = (0..13).map(|i| json!({"scene_id":"logic","id":format!("x{i}"),"file":"alpha.png","label":"测试"})).collect::<Vec<_>>().into();
    assert!(serde_json::from_value::<Layers>(value.clone()).unwrap().validate(&board).is_err());
    value["run_shell"] = "no".into();
    assert!(serde_json::from_value::<Layers>(value).is_err());
}

#[cfg(unix)]
#[test]
fn layer_symlinks_and_malformed_optional_manifest_are_not_silently_ignored() {
    for manifest in [true, false] {
        let (temp, bundle) = setup(true);
        let root = temp.path();
        let name = if manifest { "layers.json" } else { "alpha.png" };
        let outside = root.join(format!("outside-{name}"));
        fs::rename(bundle.join(name), &outside).unwrap();
        std::os::unix::fs::symlink(outside, bundle.join(name)).unwrap();
        assert!(!call(root, &["import", "project", "--from", "bundle"]).status.success());
        assert!(!root.join("project/content").exists());
    }
    let (temp, bundle) = setup(true);
    fs::write(bundle.join("layers.json"), "{broken").unwrap();
    assert!(!call(temp.path(), &["import", "project", "--from", "bundle"]).status.success());
    assert!(!temp.path().join("project/content").exists());
}

#[test]
fn schema_and_help_keep_editor_pack_primary_and_preview_optional() {
    let (temp, _) = setup(false);
    let root = temp.path();
    let request = fs::read_to_string(root.join("project/chat-request.md")).unwrap();
    for rule in ["## Layers JSON Schema（可选）", "## Motion JSON Schema（可选）", "剪映素材包", "不默认生成 MP4", "不要求音频或 cues"] { assert!(request.contains(rule), "missing {rule}"); }
    ok(root, &["schema", "--out", "schemas"]);
    assert_eq!(read(&root.join("schemas/layers.schema.json"))["additionalProperties"], false);
    let help = String::from_utf8(ok(root, &["--help"]).stdout).unwrap();
    assert!(help.contains("剪映素材包"));
    assert!(help.contains("preview"));
    for name in ["motion", "preview"] {
        let help = String::from_utf8(ok(root, &[name, "--help"]).stdout).unwrap();
        assert!(help.contains("NOT a final video"));
    }
}

#[test]
fn preview_does_not_silently_drop_editor_layers_or_generate_timing() {
    for layers in [false, true] {
        let (temp, _) = setup(layers);
        let root = temp.path();
        ok(root, &["import", "project", "--from", "bundle"]);
        for command in ["preview", "motion"] {
            let result = call(root, &[command, "project", "--out", "preview"]);
            assert!(!result.status.success());
            let error = String::from_utf8_lossy(&result.stderr);
            assert!(error.contains(if layers { "preview does not composite layers.json" } else { "actual WAV recording" }), "{error}");
            assert!(!root.join("preview").exists());
        }
    }
}

#[test]
fn supplied_cues_appear_in_notes_but_remain_separate_from_the_plan() {
    let (temp, bundle) = setup(false);
    let root = temp.path();
    let mut assets = read(&bundle.join("assets.json"));
    let board = read(&bundle.join("storyboard.json"));
    let mut cues = Vec::new();
    for scene in board["scenes"].as_array().unwrap() {
        for (i, text) in scene["narration"].as_array().unwrap().iter().enumerate() {
            let start = cues.len() as u64 * 1000;
            cues.push(json!({"scene_id":scene["id"],"utterance_index":i,"text":text,"start_ms":start,"end_ms":start+750}));
        }
    }
    let spec = hound::WavSpec { channels:1,sample_rate:8000,bits_per_sample:16,sample_format:hound::SampleFormat::Int };
    let mut writer = hound::WavWriter::create(bundle.join("voice.wav"), spec).unwrap();
    for _ in 0..32_000 { writer.write_sample(0i16).unwrap(); }
    writer.finalize().unwrap();
    assets["audio"] = "voice.wav".into();
    assets["cues"] = cues.into();
    save(&bundle.join("assets.json"), &assets);
    ok(root, &["import", "project", "--from", "bundle"]);
    ok(root, &["export", "project", "--out", "export"]);
    let notes = fs::read_to_string(root.join("export/edit-guide.md")).unwrap();
    assert!(notes.contains("00:00:01,000 → 00:00:01,750"));
    assert!(notes.contains("更换配音"));
    assert!(root.join("export/subtitles.srt").exists());
    assert_eq!(read(&root.join("export/material-report.json"))["speech_alignment_verified"], false);
}
