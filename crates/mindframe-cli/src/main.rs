mod materials;
mod pipeline;

use std::{fs, path::PathBuf};

use anyhow::{Result, ensure};
use clap::{Args, Parser, Subcommand, ValueEnum};
use mindframe_core::{Storyboard, Timeline, materials::{Assets, MotionPlan, Project}};
use pipeline::Config;

#[derive(Parser)]
#[command(name = "mindframe", version, about = "把聊天创作的知识内容整理成剪辑素材，并可渲染轻量知识讲解视频")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Preset { Bilibili, Douyin, Both }

#[derive(Args)]
pub struct RenderOptions {
    #[arg(long, value_enum, default_value = "bilibili")]
    preset: Preset,
    #[arg(long, default_value = "renderer/remotion")]
    renderer: PathBuf,
    /// Resolution multiplier. 1 is full HD; smaller values are explicit previews.
    #[arg(long, default_value_t = 1.0)]
    scale: f64,
}

#[derive(Subcommand)]
enum Command {
    /// Create a local source snapshot and an authoring handoff for chat. No API calls.
    Init {
        input: PathBuf,
        #[arg(long)] out: PathBuf,
        #[arg(long, value_parser = ["douyin", "bilibili"], default_value = "douyin")]
        preset: String,
    },
    /// Import an authored directory containing storyboard.json, assets.json and actual media.
    Import {
        project: PathBuf,
        #[arg(long)] from: PathBuf,
    },
    /// Export standard media and editing instructions, not a native editor draft.
    Export {
        project: PathBuf,
        #[arg(long, value_parser = ["jianying"], default_value = "jianying")]
        target: String,
        #[arg(long)] out: PathBuf,
    },
    /// Render a chat-material project as a lightweight knowledge lecture. Requires real WAV + reviewed cues.
    Motion {
        project: PathBuf,
        #[arg(long)] out: PathBuf,
        #[arg(long, value_parser = ["douyin", "bilibili"], default_value = "douyin")]
        preset: String,
        #[arg(long, default_value = "renderer/remotion")]
        renderer: PathBuf,
        #[arg(long, default_value_t = 1.0)]
        scale: f64,
    },
    /// Validate chat materials or a legacy storyboard project, without model calls.
    Validate { project: PathBuf },
    /// Generate JSON Schemas from the Rust contract types.
    Schema { #[arg(long, default_value = "schemas")] out: PathBuf },
    /// Optional API workflow: plan content through the configured LLM (may incur charges).
    Plan {
        input: PathBuf,
        #[arg(long)] out: PathBuf,
        #[arg(long, default_value = "mindframe.toml")] config: PathBuf,
    },
    /// Optional API workflow: planning, media production and MP4 rendering.
    Build {
        input: PathBuf,
        #[arg(long)] out: PathBuf,
        #[arg(long, default_value = "mindframe.toml")] config: PathBuf,
        #[command(flatten)] render: RenderOptions,
    },
    /// Optional API workflow: produce media from an edited legacy storyboard project.
    Produce {
        project: PathBuf,
        #[arg(long, default_value = "mindframe.toml")] config: PathBuf,
        #[command(flatten)] render: RenderOptions,
    },
    /// Re-render an existing legacy timed-media project without model calls.
    Render {
        project: PathBuf,
        #[command(flatten)] render: RenderOptions,
    },
    /// Curated legacy demo with local eSpeak, not AI-generated content.
    Demo {
        #[arg(long)] out: PathBuf,
        #[command(flatten)] render: RenderOptions,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Init { input, out, preset } => materials::init(&input, &out, preset)?,
        Command::Import { project, from } => materials::import(&project, &from)?,
        Command::Export { project, target: _, out } => materials::export(&project, &out)?,
        Command::Motion { project, out, preset, renderer, scale } => {
            ensure!(scale.is_finite() && (0.1..=1.0).contains(&scale), "scale must be in 0.1..=1.0");
            materials::motion(&project, &out, &preset, &renderer, scale)?;
        }
        Command::Validate { project } => {
            if project.join("project.json").symlink_metadata().is_ok() {
                materials::validate(&project)?;
            } else {
                pipeline::load_board(&project)?;
                println!("storyboard and source references: valid (semantic accuracy still needs human review)");
            }
        }
        Command::Schema { out } => {
            fs::create_dir_all(&out)?;
            pipeline::write_json(&out.join("storyboard.schema.json"), &schemars::schema_for!(Storyboard))?;
            pipeline::write_json(&out.join("timeline.schema.json"), &schemars::schema_for!(Timeline))?;
            pipeline::write_json(&out.join("project.schema.json"), &schemars::schema_for!(Project))?;
            pipeline::write_json(&out.join("assets.schema.json"), &schemars::schema_for!(Assets))?;
            pipeline::write_json(&out.join("motion.schema.json"), &schemars::schema_for!(MotionPlan))?;
        }
        Command::Plan { input, out, config } => {
            let source = pipeline::read_source(&input)?;
            let config = Config::load(&config)?;
            pipeline::plan(&source, &out, &config)?;
        }
        Command::Build { input, out, config, render } => {
            // 1. 检查输入、配置和本地依赖；不为预检调用收费 API。
            let source = pipeline::read_source(&input)?;
            let config = Config::load(&config)?;
            config.check_media()?;
            pipeline::check_renderer(&render)?;
            // 2. 先持久化可审查的内容，再生成声音、图片和视频。
            pipeline::plan(&source, &out, &config)?;
            pipeline::produce(&out, &config)?;
            pipeline::render(&out, &render)?;
        }
        Command::Produce { project, config, render } => {
            ensure!(!project.join("project.json").try_exists()?, "chat-material projects use export --target jianying; produce is the separate optional API workflow");
            let config = Config::load(&config)?;
            pipeline::check_renderer(&render)?;
            pipeline::produce(&project, &config)?;
            pipeline::render(&project, &render)?;
        }
        Command::Render { project, render } => {
            ensure!(!project.join("project.json").try_exists()?, "chat-material projects use export --target jianying; direct Remotion rendering of this format is not implemented");
            pipeline::check_renderer(&render)?;
            pipeline::render(&project, &render)?;
        }
        Command::Demo { out, render } => {
            let config = Config::demo();
            config.check_media()?;
            pipeline::check_renderer(&render)?;
            pipeline::new_project(&out)?;
            fs::write(out.join("source.md"), include_str!("../../../examples/demo/source.md"))?;
            fs::write(out.join("storyboard.json"), include_str!("../../../examples/demo/storyboard.json"))?;
            fs::write(out.join("DEMO.txt"), "Curated content fixture. Local eSpeak voice. Not a live LLM/TTS/image-provider quality test.\n")?;
            pipeline::produce(&out, &config)?;
            pipeline::render(&out, &render)?;
        }
    }
    Ok(())
}

impl RenderOptions {
    fn validate(&self) -> Result<()> {
        ensure!(self.scale.is_finite() && (0.1..=1.0).contains(&self.scale), "scale must be in 0.1..=1.0");
        Ok(())
    }
}
