use std::collections::HashSet;

use anyhow::{Result, ensure};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const FPS: u32 = 30;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SourceRef {
    pub line_start: usize,
    pub line_end: usize,
    pub quote: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct KeyPoint {
    pub text: String,
    pub sources: Vec<SourceRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Storyboard {
    pub schema_version: u32,
    pub title: String,
    pub summary: String,
    pub key_points: Vec<KeyPoint>,
    pub scenes: Vec<Scene>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub id: String,
    /// One short utterance per item. Each is synthesized and timed separately.
    pub narration: Vec<String>,
    /// One-based indices into key_points; these are provenance, not fact proofs.
    pub point_refs: Vec<usize>,
    pub visual: Visual,
    pub transition: Transition,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Transition {
    Cut,
    Fade,
    Slide,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Visual {
    Title { text: String },
    KeyPoint { title: String, body: String },
    Image { prompt: String },
    Quote { text: String, source: String },
    Diagram { mermaid: String },
    Code { language: String, code: String },
}

impl Storyboard {
    /// Validate the external planning boundary once, against the exact source snapshot.
    pub fn validate(&self, source: &str) -> Result<()> {
        ensure!(self.schema_version == 1, "unsupported storyboard schema_version");
        ensure!(!self.title.trim().is_empty(), "title is empty");
        ensure!(!self.summary.trim().is_empty(), "summary is empty");
        ensure!(!self.key_points.is_empty(), "key_points is empty");
        ensure!(!self.scenes.is_empty() && self.scenes.len() <= 80, "expected 1..80 scenes");
        let lines: Vec<_> = source.lines().collect();
        for (index, point) in self.key_points.iter().enumerate() {
            ensure!(!point.text.trim().is_empty() && !point.sources.is_empty(), "key point {} lacks text or sources", index + 1);
            for reference in &point.sources {
                ensure!(reference.line_start > 0 && reference.line_start <= reference.line_end && reference.line_end <= lines.len(), "key point {} has an invalid source range", index + 1);
                ensure!(!reference.quote.trim().is_empty() && lines[reference.line_start - 1..reference.line_end].join("\n").contains(&reference.quote), "key point {} quote is not present in its cited lines", index + 1);
            }
        }
        let mut ids = HashSet::new();
        for scene in &self.scenes {
            ensure!(!scene.id.is_empty() && scene.id.len() <= 64 && scene.id.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_'), "scene id must be 1..64 ASCII letters, digits, '-' or '_'");
            ensure!(ids.insert(&scene.id), "duplicate scene id: {}", scene.id);
            ensure!(!scene.narration.is_empty() && scene.narration.len() <= 40, "scene {} needs 1..40 utterances", scene.id);
            for text in &scene.narration {
                ensure!(!text.trim().is_empty() && text.chars().count() <= 160 && !text.contains('\n') && !text.contains('\r'), "scene {} narration items must be nonempty single lines of at most 160 characters", scene.id);
            }
            ensure!(!scene.point_refs.is_empty() && scene.point_refs.iter().all(|i| *i > 0 && *i <= self.key_points.len()), "scene {} has invalid point_refs", scene.id);
            let fields: Vec<&str> = match &scene.visual {
                Visual::Title { text } => vec![text],
                Visual::KeyPoint { title, body } => vec![title, body],
                Visual::Image { prompt } => vec![prompt],
                Visual::Quote { text, source: attribution } => {
                    ensure!(source.contains(text.as_str()), "scene {} quote must occur verbatim in source", scene.id);
                    vec![text, attribution]
                }
                Visual::Diagram { mermaid } => {
                    let code = mermaid.trim();
                    ensure!((code.starts_with("flowchart ") || code.starts_with("graph ")) && !code.contains("%%{") && !code.contains('<') && !code.contains("click "), "V1 supports plain Mermaid flowcharts without HTML, configuration directives or links");
                    vec![mermaid]
                },
                Visual::Code { language, code } => vec![language, code],
            };
            ensure!(fields.iter().all(|s| !s.trim().is_empty()), "scene {} has empty visual fields", scene.id);
        }
        Ok(())
    }

    pub fn script_markdown(&self) -> String {
        let mut text = format!("# {}\n\n<!-- Generated export. Edit narration in storyboard.json, not this file. -->\n", self.title);
        for scene in &self.scenes {
            text.push_str(&format!("\n## {}\n\n{}\n", scene.id, scene.narration.join("\n\n")));
        }
        text
    }

    pub fn points_markdown(&self) -> String {
        let mut text = format!("# {}\n\n{}\n", self.title, self.summary);
        for (i, point) in self.key_points.iter().enumerate() {
            text.push_str(&format!("\n## {}. {}\n", i + 1, point.text));
            for reference in &point.sources {
                text.push_str(&format!("\nsource.md:L{}-L{}\n\n> {}\n", reference.line_start, reference.line_end, reference.quote.replace('\n', "\n> ")));
            }
        }
        text
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Clip {
    pub scene_index: usize,
    pub utterance_index: usize,
    pub start_frame: u32,
    pub duration_frames: u32,
    /// A generated basename within assets/, never a URL or arbitrary filesystem path.
    pub audio: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Timeline {
    pub schema_version: u32,
    pub fps: u32,
    pub storyboard: Storyboard,
    pub clips: Vec<Clip>,
}

impl Timeline {
    pub fn validate(&self, source: &str) -> Result<()> {
        self.storyboard.validate(source)?;
        ensure!(self.schema_version == 1 && self.fps == FPS, "unsupported timeline version or fps");
        let mut index = 0;
        let mut frame = 0u32;
        for (scene_index, scene) in self.storyboard.scenes.iter().enumerate() {
            for utterance_index in 0..scene.narration.len() {
                let clip = self.clips.get(index).ok_or_else(|| anyhow::anyhow!("missing audio clip"))?;
                ensure!(clip.scene_index == scene_index && clip.utterance_index == utterance_index && clip.start_frame == frame && clip.duration_frames > 0, "audio clips must be ordered, positive and contiguous");
                ensure!(clip.audio == format!("{scene_index:03}-{utterance_index:03}.wav"), "invalid audio asset name");
                frame = frame.checked_add(clip.duration_frames).ok_or_else(|| anyhow::anyhow!("timeline too long"))?;
                index += 1;
            }
        }
        ensure!(index == self.clips.len() && frame <= FPS * 60 * 60, "extra clips or timeline exceeds one hour");
        Ok(())
    }

    pub fn subtitles(&self) -> String {
        let timestamp = |frame: u32| {
            let ms = u64::from(frame) * 1000 / u64::from(FPS);
            format!("{:02}:{:02}:{:02},{:03}", ms / 3_600_000, ms / 60_000 % 60, ms / 1000 % 60, ms % 1000)
        };
        self.clips.iter().enumerate().map(|(i, clip)| {
            format!("{}\n{} --> {}\n{}\n\n", i + 1, timestamp(clip.start_frame), timestamp(clip.start_frame + clip.duration_frames), self.storyboard.scenes[clip.scene_index].narration[clip.utterance_index])
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Storyboard {
        serde_json::from_str(include_str!("../../../examples/demo/storyboard.json")).unwrap()
    }
    const SOURCE: &str = include_str!("../../../examples/demo/source.md");

    #[test]
    fn fixture_is_grounded() { fixture().validate(SOURCE).unwrap(); }

    #[test]
    fn invalid_plans_fail() {
        for case in 0..10 {
            let mut b = fixture();
            match case {
                0 => b.schema_version = 2,
                1 => b.title.clear(),
                2 => b.key_points[0].sources[0].line_start = 0,
                3 => b.key_points[0].sources[0].quote = "invented quotation".into(),
                4 => b.scenes[0].id = "../escape".into(),
                5 => b.scenes[1].id = b.scenes[0].id.clone(),
                6 => b.scenes[0].point_refs = vec![999],
                7 => b.scenes[0].narration.clear(),
                8 => b.scenes[0].narration = vec!["x".repeat(161)],
                _ => b.scenes.clear(),
            }
            assert!(b.validate(SOURCE).is_err(), "case {case}");
        }
    }

    #[test]
    fn unknown_fields_are_rejected() {
        let mut value = serde_json::to_value(fixture()).unwrap();
        value["run_shell"] = "not permitted".into();
        assert!(serde_json::from_value::<Storyboard>(value).is_err());
    }

    #[test]
    fn subtitles_use_frame_boundaries() {
        let board = fixture();
        let clips = board.scenes.iter().enumerate().map(|(i, _)| Clip { scene_index: i, utterance_index: 0, start_frame: i as u32 * 31, duration_frames: 31, audio: format!("{i:03}-000.wav") }).collect();
        let mut timeline = Timeline { schema_version: 1, fps: FPS, storyboard: board, clips };
        timeline.validate(SOURCE).unwrap();
        assert!(timeline.subtitles().contains("00:00:00,000 --> 00:00:01,033"));
        timeline.clips[1].start_frame += 1;
        assert!(timeline.validate(SOURCE).is_err());
    }
}
