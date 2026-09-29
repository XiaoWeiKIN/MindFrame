use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Storyboard {
    pub title: String,
    pub scenes: Vec<Scene>,
}

impl Storyboard {
    pub fn validate(&self) -> Result<(), StoryboardError> {
        if self.title.trim().is_empty() {
            return Err(StoryboardError::EmptyTitle);
        }
        if self.scenes.is_empty() {
            return Err(StoryboardError::NoScenes);
        }

        for scene in &self.scenes {
            scene.validate()?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub duration_ms: u64,
    pub narration: String,
    pub visual: Visual,
}

impl Scene {
    fn validate(&self) -> Result<(), StoryboardError> {
        if self.id.trim().is_empty() {
            return Err(StoryboardError::EmptySceneId);
        }
        if self.duration_ms == 0 {
            return Err(StoryboardError::ZeroDuration {
                scene_id: self.id.clone(),
            });
        }
        if self.narration.trim().is_empty() {
            return Err(StoryboardError::EmptyNarration {
                scene_id: self.id.clone(),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Visual {
    Title {
        text: String,
    },
    KeyPoint {
        title: String,
        body: String,
    },
    Image {
        prompt: String,
    },
    Quote {
        text: String,
        source: Option<String>,
    },
    Diagram {
        mermaid: String,
    },
    Code {
        language: String,
        code: String,
    },
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum StoryboardError {
    #[error("storyboard title must not be empty")]
    EmptyTitle,
    #[error("storyboard must contain at least one scene")]
    NoScenes,
    #[error("scene id must not be empty")]
    EmptySceneId,
    #[error("scene '{scene_id}' duration must be greater than zero")]
    ZeroDuration { scene_id: String },
    #[error("scene '{scene_id}' narration must not be empty")]
    EmptyNarration { scene_id: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_storyboard_passes_validation() {
        let storyboard = Storyboard {
            title: "Why MVCC exists".into(),
            scenes: vec![Scene {
                id: "intro".into(),
                duration_ms: 5_000,
                narration: "Two transactions want different views of the same row.".into(),
                visual: Visual::KeyPoint {
                    title: "The conflict".into(),
                    body: "Readers and writers need isolation without blocking everything.".into(),
                },
            }],
        };

        assert_eq!(storyboard.validate(), Ok(()));
    }

    #[test]
    fn zero_duration_fails_at_domain_boundary() {
        let storyboard = Storyboard {
            title: "Example".into(),
            scenes: vec![Scene {
                id: "intro".into(),
                duration_ms: 0,
                narration: "Narration".into(),
                visual: Visual::Title {
                    text: "Example".into(),
                },
            }],
        };

        assert_eq!(
            storyboard.validate(),
            Err(StoryboardError::ZeroDuration {
                scene_id: "intro".into()
            })
        );
    }
}
