//! Editor handoff contracts and plain-text instructions, not a video or editor backend.
use std::collections::{HashMap, HashSet};

use anyhow::{Result, ensure};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Scene, Storyboard, Visual, materials::{Cue, MotionElement, MotionPlan, MotionScenePlan, MotionSlot, SceneImage, safe_relative_path, timestamp}};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Layers {
    pub schema_version: u32,
    /// Additional actual raster files, never imaginary layers extracted from a flat image.
    pub overlays: Vec<Overlay>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Overlay {
    pub scene_id: String,
    pub id: String,
    pub file: String,
    pub label: String,
    /// Optional semantic trigger. No audio or seconds are required for editor instructions.
    #[serde(default)]
    pub utterance_index: Option<usize>,
}

impl Layers {
    /// Storyboard is validated by the caller. Actual file decoding belongs to the CLI.
    pub fn validate(&self, board: &Storyboard) -> Result<()> {
        ensure!(self.schema_version == 1, "unsupported layers schema_version");
        ensure!(!self.overlays.is_empty() && self.overlays.len() <= board.scenes.len() * 12, "layers needs 1..12 overlays per storyboard scene");
        let mut ids = HashSet::new();
        let mut counts = HashMap::new();
        for overlay in &self.overlays {
            let scene = board.scenes.iter().find(|scene| scene.id == overlay.scene_id)
                .ok_or_else(|| anyhow::anyhow!("overlay references unknown scene: {}", overlay.scene_id))?;
            ensure!(!overlay.id.is_empty() && overlay.id.len() <= 64 && overlay.id.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_'), "overlay id must be 1..64 ASCII letters, digits, '-' or '_'");
            ensure!(ids.insert((&overlay.scene_id, &overlay.id)), "duplicate overlay id for scene {}: {}", overlay.scene_id, overlay.id);
            let count = counts.entry(&overlay.scene_id).or_insert(0usize);
            *count += 1;
            ensure!(*count <= 12, "at most 12 overlays per scene");
            safe_relative_path(&overlay.file)?;
            ensure!(!overlay.label.trim().is_empty() && overlay.label.chars().count() <= 120 && !overlay.label.chars().any(char::is_control), "overlay label must be a readable single line of 1..120 characters");
            if let Some(index) = overlay.utterance_index {
                ensure!(index < scene.narration.len(), "overlay trigger is outside scene narration: {}", overlay.scene_id);
            }
        }
        Ok(())
    }
}

pub fn narration_text(board: &Storyboard) -> String {
    let lines: Vec<_> = board.scenes.iter().flat_map(|scene| scene.narration.iter().map(String::as_str)).collect();
    format!("{}\n", lines.join("\n"))
}

// Indented blocks preserve literal code/LaTeX and do not turn source HTML or Markdown into instructions.
fn block(text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() { out.push_str(&format!("    {line}\n")); }
    out.push('\n');
    out
}

fn slot_label(slot: &MotionSlot) -> &'static str {
    match slot {
        MotionSlot::Top => "上方",
        MotionSlot::Left => "左侧",
        MotionSlot::Center => "中央",
        MotionSlot::Right => "右侧",
        MotionSlot::Bottom => "下方，避开字幕区",
    }
}

fn element_description(element: &MotionElement) -> (&str, String) {
    let (id, slot, content) = match element {
        MotionElement::Text { id, text, slot, emphasis } => (id, slot, format!("文字{}：{text}", if *emphasis { "【重点】" } else { "" })),
        MotionElement::Stat { id, label, value, slot, emphasis } => (id, slot, format!("{label}{}：{value}", if *emphasis { "【重点】" } else { "" })),
        MotionElement::Relation { id, from, to, label, slot } => (id, slot, format!("关系：{from} → {to}{}", label.as_ref().map(|s| format!("；{s}")).unwrap_or_default())),
        MotionElement::Matrix { id, title, headers, rows, slot } => {
            let rows = rows.iter().map(|row| row.join(" | ")).collect::<Vec<_>>().join("\n");
            (id, slot, format!("矩阵：{title}\n{}\n{rows}", headers.join(" | ")))
        }
        MotionElement::Formula { id, text, highlight, note, slot } => {
            let mut content = format!("公式原样：{text}\n显示字符串，不代表已完成数学排版。");
            if let Some(part) = highlight { content.push_str(&format!("\n强调：{part}")); }
            if let Some(note) = note { content.push_str(&format!("\n说明：{note}")); }
            (id, slot, content)
        }
    };
    (id.as_str(), format!("位置：{}\n{content}", slot_label(slot)))
}

/// Review validated authoring data without inferring quality, vocal delivery or timing.
pub fn narration_review(board: &Storyboard, motion: Option<&MotionPlan>) -> String {
    let mut out = String::from("# 口播审阅稿\n\n由当前 storyboard.json 与可选 motion.json 派生；修改编辑源后重新导出。此文件含来源和审阅注释，不用于配音；配音使用 narration.txt。\n\n以下是审阅材料，不是质量评分或验收结论。画面强调不等于声音重音；未生成停顿或时间点。\n\n## 全文审阅\n\n- [ ] 按所选体裁，观众能独立理解问题与主张。\n- [ ] 关键专业判断及其限定得到保留，有铺垫、推理和具体解释。\n- [ ] 各段理解路径衔接，避免只改一个例句或反复套同一种句式。\n- [ ] 改稿后重新核对画面语义触发；听感与同步须另行实听。\n\n## 标题\n\n");
    out.push_str(&block(&board.title));
    for scene in &board.scenes {
        out.push_str(&format!("## 场景 {}\n\n### 来源与观点\n\n", scene.id));
        for index in &scene.point_refs {
            let point = &board.key_points[index - 1];
            out.push_str(&format!("观点 {index}（创作者选取，不代表已验证）：\n\n"));
            out.push_str(&block(&point.text));
            for source in &point.sources {
                out.push_str(&format!("原文 L{}–L{}：\n\n", source.line_start, source.line_end));
                out.push_str(&block(&source.quote));
            }
        }
        let plan = motion.and_then(|plan| plan.scenes.iter().find(|plan| plan.scene_id == scene.id));
        if plan.is_none() {
            out.push_str("未提供本幕逐句话面计划；仍需审阅口播，不据此认定缺少讲解重点。\n\n");
        }
        out.push_str("### 实际口播与本句触发的画面强调\n\n只列显式 emphasis 或 formula.highlight；保持中的画面和完整变化见 edit-notes。空缺不表示本句没有演讲重音。\n\n");
        for (index, text) in scene.narration.iter().enumerate() {
            out.push_str(&format!("#### narration[{index}]\n\n"));
            out.push_str(&block(text));
            if let Some(step) = plan.and_then(|plan| plan.steps.iter().find(|step| step.utterance_index == index)) {
                for element in &step.elements {
                    if matches!(element, MotionElement::Text { emphasis: true, .. } | MotionElement::Stat { emphasis: true, .. } | MotionElement::Formula { highlight: Some(_), .. }) {
                        let (id, description) = element_description(element);
                        out.push_str(&format!("画面强调 {id}（不作为配音输入）：\n\n"));
                        out.push_str(&block(&description));
                    }
                }
            }
        }
    }
    out
}

/// Produce human editing instructions from validated contracts. No inference of speech timing.
pub fn scene_edit_note(scene: &Scene, image: &SceneImage, cues: &[Cue], plan: Option<&MotionScenePlan>, layers: Option<&Layers>) -> String {
    let mut out = format!("# {}\n\n剪辑说明，不是剪映轨道。配音、字幕、转场和成片导出由剪辑者在剪映中完成。\n\n## 主画面\n\n{}\n\n", scene.id, image.file);
    out.push_str("## 后期屏幕文字\n\n");
    if image.screen_text.trim().is_empty() {
        out.push_str("未提供后期叠字。不要把已经印在图片里的文字重复叠加。\n\n");
    } else { out.push_str(&block(&image.screen_text)); }
    out.push_str("## 画面意图（不是额外生成的图片）\n\n");
    match &scene.visual {
        Visual::Title { text } => out.push_str(&block(text)),
        Visual::KeyPoint { title, body } => out.push_str(&block(&format!("{title}\n{body}"))),
        Visual::Image { prompt } => out.push_str(&block(prompt)),
        Visual::Quote { text, source } => out.push_str(&block(&format!("{text}\n署名：{source}"))),
        Visual::Diagram { mermaid } => {
            out.push_str("以下是 Mermaid 逻辑稿，剪映不会将它自动变成图层：\n\n");
            out.push_str(&block(mermaid));
        }
        Visual::Code { language, code } => out.push_str(&block(&format!("{language}\n{code}"))),
    }
    if let Some(layers) = layers {
        for overlay in layers.overlays.iter().filter(|o| o.scene_id == scene.id) {
            out.push_str(&format!("## 叠加图片 {}\n\n{}\n\n", overlay.id, overlay.file));
            out.push_str(&block(&overlay.label));
            match overlay.utterance_index {
                Some(index) => {
                    out.push_str(&format!("语义触发：narration[{index}]\n\n"));
                    out.push_str(&block(&scene.narration[index]));
                }
                None => out.push_str("镜头内使用；具体入场位置由剪辑者决定。\n\n"),
            }
            out.push_str("它是独立位图，不是可逐字修改的文字层；透明性见素材报告。\n\n");
        }
    }
    out.push_str("## 逐句口播与时间\n\n");
    for (index, text) in scene.narration.iter().enumerate() {
        out.push_str(&format!("### narration[{index}]\n\n"));
        out.push_str(&block(text));
        if let Some(cue) = cues.iter().find(|c| c.scene_id == scene.id && c.utterance_index == index) {
            out.push_str(&format!("已有时间：{} → {}；仅文件/范围校验，未自动实听核对。\n\n", timestamp(cue.start_ms), timestamp(cue.end_ms)));
        } else {
            out.push_str("时间：未定时，待剪映配音后确定。\n\n");
        }
    }
    if let Some(plan) = plan {
        out.push_str("## 随口播变化的画面（建议，需在剪映中设置）\n\n");
        let mut previous: Vec<(&str, String)> = Vec::new();
        for step in &plan.steps {
            out.push_str(&format!("### 讲到 narration[{}] 时\n\n", step.utterance_index));
            out.push_str(&block(&scene.narration[step.utterance_index]));
            let current: Vec<_> = step.elements.iter().map(element_description).collect();
            if current.is_empty() { out.push_str("本步清空重点元素，保留背景与字幕。\n\n"); }
            for (id, description) in &current {
                match previous.iter().find(|(before_id, _)| before_id == id) {
                    Some((_, before)) if before == description => out.push_str(&format!("保持 {id}，不要重复入场。\n\n")),
                    Some((_, before)) => {
                        out.push_str(&format!("更新 {id}；只改变该元素（替换/强调由剪辑者设置）：\n\n原状态：\n\n"));
                        out.push_str(&block(before));
                        out.push_str("新状态：\n\n");
                        out.push_str(&block(description));
                    }
                    None => {
                        out.push_str(&format!("新增 {id}：\n\n"));
                        out.push_str(&block(description));
                    }
                }
            }
            for (id, _) in &previous {
                if !current.iter().any(|(next_id, _)| next_id == id) { out.push_str(&format!("移除 {id}。\n\n")); }
            }
            previous = current;
        }
    }
    out.push_str(&format!("## 转场意图\n\n{:?}；仅为建议，不会自动应用剪映效果。\n\n更换配音、剪句或改语速后，重新制作字幕与时间点；旧 SRT 不会自动适配。\n", scene.transition));
    out
}
