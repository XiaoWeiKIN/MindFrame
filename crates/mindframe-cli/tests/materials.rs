use std::{fs, io::Cursor, path::{Path, PathBuf}, process::{Command, Output}};

use image::{DynamicImage, ImageFormat};
use serde_json::{Value, json};
use tempfile::TempDir;

const SOURCE: &str = "# Fixture\nA snapshot preserves a version.\n";

fn call(args: &[&str]) -> Output {
    // A primary-workflow regression must not acquire a key or launch node/ffmpeg from PATH.
    Command::new(env!("CARGO_BIN_EXE_mindframe"))
        .args(args).env_clear().env("PATH", "").output().unwrap()
}
fn ok(args: &[&str]) -> Output {
    let result = call(args);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    result
}
fn save(path: &Path, value: &Value) { fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap(); }
fn read(path: &Path) -> Value { serde_json::from_slice(&fs::read(path).unwrap()).unwrap() }
fn text(path: &Path) -> &str { path.to_str().unwrap() }

fn setup() -> (TempDir, PathBuf, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source.md");
    fs::write(&source, SOURCE).unwrap();
    let project = temp.path().join("project");
    ok(&["init", text(&source), "--out", text(&project)]);
    let bundle = temp.path().join("bundle");
    fs::create_dir(&bundle).unwrap();
    let mut images = vec![];
    let mut scenes = vec![];
    for (index, (extension, format)) in [("png", ImageFormat::Png), ("jpg", ImageFormat::Jpeg), ("webp", ImageFormat::WebP)].into_iter().enumerate() {
        let name = format!("drawing-{index}.{extension}");
        let mut bytes = Cursor::new(Vec::new());
        DynamicImage::new_rgb8(18, 32).write_to(&mut bytes, format).unwrap();
        fs::write(bundle.join(&name), bytes.into_inner()).unwrap();
        let id = format!("scene-{index}");
        images.push(json!({"scene_id":id,"file":name,"screen_text":"中文,\"引用\"\n第二行"}));
        scenes.push(json!({"id":id,"narration":["A snapshot preserves a version."],"point_refs":[1],"visual":{"type":"image","prompt":"fixture only"},"transition":"fade"}));
    }
    save(&bundle.join("storyboard.json"), &json!({"schema_version":1,"title":"Fixture","summary":"A snapshot example.","key_points":[{"text":"A snapshot preserves a version.","sources":[{"line_start":2,"line_end":2,"quote":"A snapshot preserves a version."}]}],"scenes":scenes}));
    save(&bundle.join("assets.json"), &json!({"schema_version":1,"images":images}));
    fs::write(bundle.join("do-not-copy.txt"), "unrelated download").unwrap();
    (temp, project, bundle)
}

fn wav(path: &Path, duration_ms: u32) {
    let spec = hound::WavSpec { channels: 1, sample_rate: 8000, bits_per_sample: 16, sample_format: hound::SampleFormat::Int };
    let mut writer = hound::WavWriter::create(path, spec).unwrap();
    for _ in 0..duration_ms * 8 { writer.write_sample(0i16).unwrap(); }
    writer.finalize().unwrap();
}

fn add_recording(bundle: &Path, with_cues: bool) {
    wav(&bundle.join("voice.wav"), 3000);
    let mut a = read(&bundle.join("assets.json"));
    a["audio"] = "voice.wav".into();
    if with_cues {
        a["cues"] = (0..3).map(|i| json!({"scene_id":format!("scene-{i}"),"utterance_index":0,"text":"A snapshot preserves a version.","start_ms":i*1000,"end_ms":i*1000+750})).collect::<Vec<_>>().into();
    }
    save(&bundle.join("assets.json"), &a);
}

#[test]
fn full_untimed_import_export_without_keys_or_external_programs() {
    let (temp, project, bundle) = setup();
    assert!(project.join("chat-request.md").exists());
    assert!(!project.join("content").exists());
    ok(&["import", text(&project), "--from", text(&bundle)]);
    ok(&["validate", text(&project)]);
    let out = temp.path().join("export");
    ok(&["export", text(&project), "--target", "jianying", "--out", text(&out)]);
    assert!(!out.join("subtitles.srt").exists());
    assert!(!out.join("source.md").exists());
    assert!(!out.join("do-not-copy.txt").exists());
    assert!(out.join("subtitles.txt").exists());
    let report = read(&out.join("material-report.json"));
    assert_eq!(report["timing"], "absent");
    assert_eq!(report["native_editor_project"], false);
    for (i, ext) in ["png", "jpg", "webp"].iter().enumerate() {
        assert_eq!(fs::read(bundle.join(format!("drawing-{i}.{ext}"))).unwrap(), fs::read(out.join(format!("images/{:03}-scene-{i}.{ext}", i+1))).unwrap());
    }
    let csv = fs::read_to_string(out.join("shot-list.csv")).unwrap();
    assert!(csv.starts_with('\u{feff}'));
    assert!(csv.contains("\"scene-0\",\"\",\"\""));
    assert!(csv.contains("\"\"引用\"\""));
}

#[test]
fn supplied_timing_produces_exact_srt_and_rejects_stale_narration() {
    let (temp, project, bundle) = setup();
    add_recording(&bundle, true);
    ok(&["import", text(&project), "--from", text(&bundle)]);
    let out = temp.path().join("timed");
    ok(&["export", text(&project), "--out", text(&out)]);
    let srt = fs::read_to_string(out.join("subtitles.srt")).unwrap();
    assert!(srt.contains("00:00:02,000 --> 00:00:02,750"));
    assert_eq!(read(&out.join("material-report.json"))["audio_duration_ms"], 3000);
    let path = project.join("content/storyboard.json");
    let mut b = read(&path);
    b["scenes"][0]["narration"][0] = "Edited narration.".into();
    save(&path, &b);
    let failed = temp.path().join("stale");
    assert!(!call(&["export", text(&project), "--out", text(&failed)]).status.success());
    assert!(!failed.exists());
}

#[test]
fn audio_without_alignment_does_not_invent_cues() {
    let (temp, project, bundle) = setup();
    add_recording(&bundle, false);
    ok(&["import", text(&project), "--from", text(&bundle)]);
    let out = temp.path().join("audio-only");
    ok(&["export", text(&project), "--out", text(&out)]);
    assert!(out.join("audio/narration.wav").exists());
    assert!(!out.join("subtitles.srt").exists());
}

#[test]
fn invalid_inputs_leave_no_partial_content() {
    for case in 0..9 {
        let (_temp, project, bundle) = setup();
        let path = bundle.join("assets.json");
        let mut a = read(&path);
        match case {
            0 => { fs::remove_file(bundle.join("drawing-2.webp")).unwrap(); },
            1 => { fs::write(bundle.join("drawing-0.png"), "a prompt is not an image").unwrap(); },
            2 => a["images"][0]["file"] = "../outside.png".into(),
            3 => a["images"][0]["scene_id"] = "unknown-scene".into(),
            4 => a["images"][1]["scene_id"] = "scene-0".into(),
            5 => { a["images"].as_array_mut().unwrap().pop(); },
            6 => a["execute"] = "not allowed".into(),
            7 => { fs::copy(bundle.join("drawing-1.jpg"), bundle.join("drawing-0.png")).unwrap(); },
            _ => {
                let mut b = read(&bundle.join("storyboard.json"));
                b["key_points"][0]["sources"][0]["quote"] = "fabricated quote".into();
                save(&bundle.join("storyboard.json"), &b);
            }
        }
        save(&path, &a);
        assert!(!call(&["import", text(&project), "--from", text(&bundle)]).status.success(), "case {case}");
        assert!(!project.join("content").exists(), "case {case}");
        assert_eq!(fs::read_to_string(project.join("source.md")).unwrap(), SOURCE);
    }
}

#[test]
fn truncated_or_too_short_audio_fails_before_import() {
    for truncate in [false, true] {
        let (_temp, project, bundle) = setup();
        add_recording(&bundle, true);
        if truncate {
            let file = fs::OpenOptions::new().write(true).open(bundle.join("voice.wav")).unwrap();
            file.set_len(48).unwrap();
        } else {
            wav(&bundle.join("voice.wav"), 1000);
        }
        assert!(!call(&["import", text(&project), "--from", text(&bundle)]).status.success());
        assert!(!project.join("content").exists());
    }
}

#[test]
fn existing_destinations_are_preserved() {
    let (temp, project, bundle) = setup();
    ok(&["import", text(&project), "--from", text(&bundle)]);
    let before = fs::read(project.join("content/storyboard.json")).unwrap();
    assert!(!call(&["import", text(&project), "--from", text(&bundle)]).status.success());
    assert_eq!(fs::read(project.join("content/storyboard.json")).unwrap(), before);
    let out = temp.path().join("existing");
    fs::create_dir(&out).unwrap();
    fs::write(out.join("mine.txt"), "keep").unwrap();
    assert!(!call(&["export", text(&project), "--out", text(&out)]).status.success());
    assert_eq!(fs::read_to_string(out.join("mine.txt")).unwrap(), "keep");
}

#[test]
fn malformed_project_marker_does_not_fall_back_to_legacy_mode() {
    let (_temp, project, _) = setup();
    fs::write(project.join("project.json"), "broken").unwrap();
    let result = call(&["validate", text(&project)]);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("project.json"));
}

#[cfg(unix)]
#[test]
fn symlink_assets_are_not_followed() {
    let (temp, project, bundle) = setup();
    let outside = temp.path().join("outside.png");
    fs::rename(bundle.join("drawing-0.png"), &outside).unwrap();
    std::os::unix::fs::symlink(outside, bundle.join("drawing-0.png")).unwrap();
    assert!(!call(&["import", text(&project), "--from", text(&bundle)]).status.success());
    assert!(!project.join("content").exists());
}
