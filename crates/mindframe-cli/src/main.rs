mod editor_export;
mod materials;
mod pipeline;
mod preview;

use std::{fs, path::PathBuf};

use anyhow::{Result, ensure};
use clap::{Args, Parser, Subcommand, ValueEnum};
use mindframe_core::{Storyboard, Timeline, editor::Layers, materials::{Assets, MotionPlan, Project}};
use pipeline::Config;

#[derive(Parser)]
#[command(name = "mindframe", version, about = "把知识创作整理成剪映素材包；配音、字幕、特效与成片在剪映完成")]
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
    /// Resolution multiplier for the optional legacy renderer.
    #[arg(long, default_value_t = 1.0)]
    scale: f64,
}

#[derive(Subcommand)]
enum Command {
    /// Create a source snapshot and chat handoff for an editor material pack. No API calls.
    Init {
        input: PathBuf,
        #[arg(long)] out: PathBuf,
        #[arg(long, value_parser = ["douyin", "bilibili"], default_value = "douyin")]
        preset: String,
    },
    /// Import storyboard/assets, optional layers/edit plan and actual files. No media tools.
    Import {
        project: PathBuf,
        #[arg(long)] from: PathBuf,
    },
    /// Inspect local materials; file validity is not a publishing-quality approval.
    Validate { project: PathBuf },
    /// Export images, plain scripts and human editing instructions. No audio/timing required.
    Export {
        project: PathBuf,
        #[arg(long, value_parser = ["jianying"], default_value = "jianying")]
        target: String,
        #[arg(long)] out: PathBuf,
    },
    /// Generate JSON Schemas from the Rust contract types.
    Schema { #[arg(long, default_value = "schemas")] out: PathBuf },
    /// Optional structural preview, NOT a final video. Requires Node, real WAV and reviewed cues.
    #[command(alias = "motion")]
    Preview {
        project: PathBuf,
        #[arg(long)] out: PathBuf,
        #[arg(long, value_parser = ["douyin", "bilibili"], default_value = "douyin")]
        preset: String,
        #[arg(long, default_value = "renderer/remotion")]
        renderer: PathBuf,
        #[arg(long, default_value_t = 1.0)]
        scale: f64,
    },
    /// Legacy opt-in API workflow (may incur charges); not needed for editor packs.
    Plan {
        input: PathBuf,
        #[arg(long)] out: PathBuf,
        #[arg(long, default_value = "mindframe.toml")] config: PathBuf,
    },
    /// Legacy opt-in API/media workflow; not the primary delivery path.
    Build {
        input: PathBuf,
        #[arg(long)] out: PathBuf,
        #[arg(long, default_value = "mindframe.toml")] config: PathBuf,
        #[command(flatten)] render: RenderOptions,
    },
    /// Legacy opt-in API media production from a root-level storyboard project.
    Produce {
        project: PathBuf,
        #[arg(long, default_value = "mindframe.toml")] config: PathBuf,
        #[command(flatten)] render: RenderOptions,
    },
    /// Legacy timed-media re-render. Use export for editor material projects.
    Render {
        project: PathBuf,
        #[command(flatten)] render: RenderOptions,
    },
    /// Legacy curated eSpeak demo for development, not publication quality.
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
        Command::Preview { project, out, preset, renderer, scale } => preview::render(&project, &out, &preset, &renderer, scale)?,
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
            pipeline::write_json(&out.join("layers.schema.json"), &schemars::schema_for!(Layers))?;
        }
        Command::Plan { input, out, config } => {
            let source = pipeline::read_source(&input)?;
            let config = Config::load(&config)?;
            pipeline::plan(&source, &out, &config)?;
        }
        Command::Build { input, out, config, render } => {
            let source = pipeline::read_source(&input)?;
            let config = Config::load(&config)?;
            config.check_media()?;
            pipeline::check_renderer(&render)?;
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
            ensure!(!project.join("project.json").try_exists()?, "chat-material projects use export --target jianying; use preview only for optional structural checks");
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
