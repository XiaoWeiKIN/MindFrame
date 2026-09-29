//! File contracts for the local, chat-authored material workflow.
use std::collections::HashSet;

use anyhow::{Result, ensure};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Storyboard;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub schema_version: u32,
    pub preset: String,
}

impl Project {
    pub fn validate(&self) -> Result<()> {
        ensure!(self.schema_version == 1, "unsupported project schema_version");
        ensure!(matches!(self.preset.as_str(), "douyin" | "bilibili"), "preset must be douyin or bilibili");
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SceneImage {
    pub scene_id: String,
    /// Relative to the material bundle. Never a URL or an attachment identifier.
    pub file: String,
    #[serde(default)]
    pub screen_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Cue {
    pub scene_id: String,
    /// Zero-based index into the scene's narration array.
    pub utterance_index: usize,
    /// The exact spoken text; prevents silently pairing a new script with old timings.
    pub text: String,
    pub start_ms: u64,
    pub end_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Assets {
    pub schema_version: u32,
    /// Exactly one actual raster image per scene, explicitly mapped by scene ID.
    pub images: Vec<SceneImage>,
    #[serde(default)]
    pub cover: Option<String>,
    /// Optional PCM 16-bit WAV recording; narration text is not audio.
    #[serde(default)]
    pub audio: Option<String>,
    /// Optional user-supplied, reviewed timings. Missing timings stay missing.
    #[serde(default)]
    pub cues: Vec<Cue>,
}

impl Assets {
    /// The caller validates the Storyboard first, then checks actual files at the I/O boundary.
    pub fn validate(&self, board: &Storyboard) -> Result<()> {
        ensure!(self.schema_version == 1, "unsupported assets schema_version");
        ensure!(self.images.len() == board.scenes.len(), "supply exactly one image for every scene");
        let expected: HashSet<_> = board.scenes.iter().map(|s| s.id.as_str()).collect();
        let mut seen = HashSet::new();
        for image in &self.images {
            ensure!(expected.contains(image.scene_id.as_str()), "unknown image scene: {}", image.scene_id);
            ensure!(seen.insert(&image.scene_id), "duplicate image scene: {}", image.scene_id);
            safe_relative_path(&image.file)?;
        }
        for path in self.cover.iter().chain(self.audio.iter()) {
            safe_relative_path(path)?;
        }
        if self.cues.is_empty() {
            return Ok(());
        }
        ensure!(self.audio.is_some(), "timed cues require an actual audio file");
        let expected_count: usize = board.scenes.iter().map(|s| s.narration.len()).sum();
        ensure!(self.cues.len() == expected_count, "timings must cover every narration utterance exactly once");
        let mut index = 0;
        let mut previous_end = 0;
        for scene in &board.scenes {
            for (utterance_index, text) in scene.narration.iter().enumerate() {
                let cue = &self.cues[index];
                ensure!(cue.scene_id == scene.id && cue.utterance_index == utterance_index, "timing order does not match storyboard");
                ensure!(cue.text == *text, "timed text changed for scene {}; re-align the recording", scene.id);
                ensure!(cue.start_ms >= previous_end && cue.end_ms > cue.start_ms && cue.end_ms <= 3_600_000, "timings must be positive, non-overlapping, and within one hour");
                previous_end = cue.end_ms;
                index += 1;
            }
        }
        Ok(())
    }

    /// Call only after contract and recording-duration validation. Empty means no SRT.
    pub fn subtitles(&self) -> Option<String> {
        if self.cues.is_empty() { return None; }
        Some(self.cues.iter().enumerate().map(|(i, cue)| {
            format!("{}\n{} --> {}\n{}\n\n", i + 1, timestamp(cue.start_ms), timestamp(cue.end_ms), cue.text)
        }).collect())
    }
}

pub fn safe_relative_path(path: &str) -> Result<()> {
    ensure!(!path.is_empty() && !path.contains(['\\', ':']) && !path.chars().any(char::is_control), "asset path must be a portable relative path: {path}");
    ensure!(path.split('/').all(|part| !matches!(part, "" | "." | "..")), "asset path contains an absolute or traversal component: {path}");
    Ok(())
}

pub fn timestamp(ms: u64) -> String {
    format!("{:02}:{:02}:{:02},{:03}", ms / 3_600_000, ms / 60_000 % 60, ms / 1000 % 60, ms % 1000)
}

/// Spreadsheet-safe display value. Original narration remains unchanged in JSON/Markdown.
pub fn csv_cell(text: &str) -> String {
    let escaped = text.replace('"', "\"\"");
    let prefix = if text.trim_start().starts_with(['=', '+', '-', '@']) { "'" } else { "" };
    format!("\"{prefix}{escaped}\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board() -> Storyboard {
        serde_json::from_str(include_str!("../../../examples/demo/storyboard.json")).unwrap()
    }

    fn assets(board: &Storyboard) -> Assets {
        Assets { schema_version: 1, images: board.scenes.iter().map(|s| SceneImage {
            scene_id: s.id.clone(), file: format!("images/{}.png", s.id), screen_text: String::new(),
        }).collect(), cover: None, audio: None, cues: vec![] }
    }

    #[test]
    fn untimed_materials_never_invent_srt() {
        let b = board();
        let a = assets(&b);
        a.validate(&b).unwrap();
        assert!(a.subtitles().is_none());
    }

    #[test]
    fn image_map_is_exact_and_portable() {
        let b = board();
        for path in ["/tmp/a.png", "../a.png", "a/../../b.png", "C:\\a.png", "https://host/a.png", "a//b.png", "./a.png", "a\n.png"] {
            let mut a = assets(&b);
            a.images[0].file = path.into();
            assert!(a.validate(&b).is_err(), "{path}");
        }
        let mut a = assets(&b);
        a.images[1].scene_id = a.images[0].scene_id.clone();
        assert!(a.validate(&b).is_err());
        a.images.pop();
        assert!(a.validate(&b).is_err());
        safe_relative_path("images/中文 image.png").unwrap();
    }

    #[test]
    fn timed_text_must_match_every_utterance() {
        let b = board();
        let mut a = assets(&b);
        a.audio = Some("voice.wav".into());
        for scene in &b.scenes {
            for (i, text) in scene.narration.iter().enumerate() {
                let start = a.cues.len() as u64 * 2000;
                a.cues.push(Cue { scene_id: scene.id.clone(), utterance_index: i, text: text.clone(), start_ms: start, end_ms: start + 1234 });
            }
        }
        a.validate(&b).unwrap();
        assert!(a.subtitles().unwrap().contains("00:00:00,000 --> 00:00:01,234"));
        for case in 0..6 {
            let mut bad = a.clone();
            match case {
                0 => bad.audio = None,
                1 => bad.cues[0].text = "new script".into(),
                2 => bad.cues[1].start_ms = 0,
                3 => bad.cues[0].utterance_index = 99,
                4 => { bad.cues.pop(); },
                _ => bad.cues[0].end_ms = 0,
            }
            assert!(bad.validate(&b).is_err(), "case {case}");
        }
    }

    #[test]
    fn csv_quotes_and_formula_prefixes_are_safe() {
        assert_eq!(csv_cell("中文,\"引用\"\n换行"), "\"中文,\"\"引用\"\"\n换行\"");
        assert_eq!(csv_cell(" =1+1"), "\"' =1+1\"");
        assert_eq!(csv_cell("normal"), "\"normal\"");
    }
}
