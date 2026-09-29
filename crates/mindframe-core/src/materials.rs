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


#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MotionPlan {
    pub schema_version: u32,
    pub scenes: Vec<MotionScenePlan>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MotionScenePlan {
    pub scene_id: String,
    /// Ordered full-state snapshots. The first step starts with narration[0].
    pub steps: Vec<MotionStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MotionStep {
    /// Zero-based narration index. The step begins at that reviewed cue.
    pub utterance_index: usize,
    /// Complete visual state for this step. Repeating an id updates/replaces that element.
    #[serde(default)]
    pub elements: Vec<MotionElement>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MotionSlot {
    Top,
    Left,
    #[default]
    Center,
    Right,
    Bottom,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum MotionElement {
    Text {
        id: String,
        text: String,
        #[serde(default)]
        slot: MotionSlot,
        #[serde(default)]
        emphasis: bool,
    },
    Stat {
        id: String,
        label: String,
        value: String,
        #[serde(default)]
        slot: MotionSlot,
        #[serde(default)]
        emphasis: bool,
    },
    Relation {
        id: String,
        from: String,
        to: String,
        #[serde(default)]
        label: Option<String>,
        #[serde(default)]
        slot: MotionSlot,
    },
    Matrix {
        id: String,
        title: String,
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
        #[serde(default)]
        slot: MotionSlot,
    },
    Formula {
        id: String,
        text: String,
        #[serde(default)]
        highlight: Option<String>,
        #[serde(default)]
        note: Option<String>,
        #[serde(default)]
        slot: MotionSlot,
    },
}

impl MotionElement {
    fn id(&self) -> &str {
        match self {
            Self::Text { id, .. }
            | Self::Stat { id, .. }
            | Self::Relation { id, .. }
            | Self::Matrix { id, .. }
            | Self::Formula { id, .. } => id,
        }
    }

    fn is_relation(&self) -> bool {
        matches!(self, Self::Relation { .. })
    }
}

fn motion_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
}

fn motion_text(value: &str, max_chars: usize, multiline: bool) -> bool {
    !value.trim().is_empty()
        && value.chars().count() <= max_chars
        && !value.chars().any(|c| c.is_control() && !(multiline && c == '\n'))
}

impl MotionPlan {
    pub fn validate(&self, board: &Storyboard) -> Result<()> {
        ensure!(self.schema_version == 1, "unsupported motion schema_version");
        ensure!(!self.scenes.is_empty() && self.scenes.len() <= board.scenes.len(), "motion plan must contain 1..storyboard scene count entries");
        let mut scene_ids = HashSet::new();
        for scene_plan in &self.scenes {
            let scene = board.scenes.iter().find(|scene| scene.id == scene_plan.scene_id)
                .ok_or_else(|| anyhow::anyhow!("motion plan references unknown scene: {}", scene_plan.scene_id))?;
            ensure!(scene_ids.insert(&scene_plan.scene_id), "duplicate motion scene: {}", scene_plan.scene_id);
            ensure!(!scene_plan.steps.is_empty() && scene_plan.steps.len() <= scene.narration.len(), "motion scene {} needs 1..narration-count steps", scene_plan.scene_id);
            ensure!(scene_plan.steps[0].utterance_index == 0, "motion scene {} first step must start at narration[0]", scene_plan.scene_id);

            let mut previous_index = None;
            for step in &scene_plan.steps {
                ensure!(step.utterance_index < scene.narration.len(), "motion step exceeds narration count for {}", scene_plan.scene_id);
                if let Some(previous) = previous_index {
                    ensure!(step.utterance_index > previous, "motion steps must use strictly increasing narration indices for {}", scene_plan.scene_id);
                }
                previous_index = Some(step.utterance_index);
                ensure!(step.elements.len() <= 12, "motion step supports at most 12 elements");

                let mut ids = HashSet::new();
                let mut targets = HashSet::new();
                for element in &step.elements {
                    ensure!(motion_id(element.id()), "invalid motion element id: {}", element.id());
                    ensure!(ids.insert(element.id()), "duplicate motion element id in one step: {}", element.id());
                    if !element.is_relation() {
                        targets.insert(element.id());
                    }
                    match element {
                        MotionElement::Text { text, .. } => {
                            ensure!(motion_text(text, 160, true), "motion text must be 1..160 readable characters");
                        }
                        MotionElement::Stat { label, value, .. } => {
                            ensure!(motion_text(label, 48, false) && motion_text(value, 48, false), "motion stat label/value must be 1..48 readable characters");
                        }
                        MotionElement::Relation { label, .. } => {
                            if let Some(label) = label {
                                ensure!(motion_text(label, 48, false), "motion relation label must be 1..48 readable characters");
                            }
                        }
                        MotionElement::Matrix { title, headers, rows, .. } => {
                            ensure!(motion_text(title, 80, false), "motion matrix title must be 1..80 readable characters");
                            ensure!((2..=4).contains(&headers.len()), "motion matrix needs 2..4 columns");
                            ensure!(headers.iter().all(|cell| motion_text(cell, 32, false)), "motion matrix headers must be 1..32 readable characters");
                            ensure!((1..=4).contains(&rows.len()), "motion matrix needs 1..4 rows");
                            ensure!(rows.iter().all(|row| row.len() == headers.len() && row.iter().all(|cell| motion_text(cell, 40, false))), "motion matrix rows must match headers and contain 1..40 character cells");
                        }
                        MotionElement::Formula { text, highlight, note, .. } => {
                            ensure!(motion_text(text, 180, false), "motion formula must be 1..180 readable characters");
                            if let Some(highlight) = highlight {
                                ensure!(motion_text(highlight, 80, false) && text.contains(highlight), "motion formula highlight must occur verbatim in formula text");
                            }
                            if let Some(note) = note {
                                ensure!(motion_text(note, 120, true), "motion formula note must be 1..120 readable characters");
                            }
                        }
                    }
                }
                for element in &step.elements {
                    if let MotionElement::Relation { from, to, .. } = element {
                        ensure!(from != to && targets.contains(from.as_str()) && targets.contains(to.as_str()), "motion relation endpoints must reference two non-relation element ids in the same step");
                    }
                }
            }
        }
        Ok(())
    }
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


    fn motion_plan(board: &Storyboard) -> MotionPlan {
        MotionPlan {
            schema_version: 1,
            scenes: vec![MotionScenePlan {
                scene_id: board.scenes[0].id.clone(),
                steps: vec![MotionStep {
                    utterance_index: 0,
                    elements: vec![
                        MotionElement::Stat {
                            id: "a".into(),
                            label: "A".into(),
                            value: "18:00".into(),
                            slot: MotionSlot::Left,
                            emphasis: false,
                        },
                        MotionElement::Stat {
                            id: "b".into(),
                            label: "B".into(),
                            value: "19:00".into(),
                            slot: MotionSlot::Right,
                            emphasis: true,
                        },
                        MotionElement::Relation {
                            id: "edge".into(),
                            from: "a".into(),
                            to: "b".into(),
                            label: Some("变化".into()),
                            slot: MotionSlot::Center,
                        },
                        MotionElement::Formula {
                            id: "formula".into(),
                            text: "Sₜ₊₁ = F(Sₜ, Aₜ, Eₜ, εₜ)".into(),
                            highlight: Some("Aₜ".into()),
                            note: Some("行动只是输入之一".into()),
                            slot: MotionSlot::Bottom,
                        },
                    ],
                }],
            }],
        }
    }

    #[test]
    fn motion_primitives_are_bounded_and_grounded_in_storyboard_steps() {
        let board = board();
        motion_plan(&board).validate(&board).unwrap();

        for case in 0..6 {
            let mut plan = motion_plan(&board);
            match case {
                0 => plan.schema_version = 2,
                1 => plan.scenes[0].scene_id = "missing".into(),
                2 => plan.scenes[0].steps[0].utterance_index = 99,
                3 => plan.scenes[0].steps[0].elements.push(MotionElement::Text {
                    id: "a".into(), text: "duplicate".into(), slot: MotionSlot::Center, emphasis: false,
                }),
                4 => {
                    if let MotionElement::Relation { to, .. } = &mut plan.scenes[0].steps[0].elements[2] {
                        *to = "missing".into();
                    }
                }
                _ => {
                    if let MotionElement::Formula { highlight, .. } = &mut plan.scenes[0].steps[0].elements[3] {
                        *highlight = Some("not-in-formula".into());
                    }
                }
            }
            assert!(plan.validate(&board).is_err(), "case {case}");
        }
    }

    #[test]
    fn motion_unknown_fields_are_rejected() {
        let board = board();
        let plan = motion_plan(&board);
        let mut value = serde_json::to_value(plan).unwrap();
        value["scenes"][0]["steps"][0]["elements"][0]["script"] = "alert(1)".into();
        assert!(serde_json::from_value::<MotionPlan>(value).is_err());
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
