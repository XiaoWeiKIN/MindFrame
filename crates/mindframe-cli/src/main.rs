mod pipeline;

use std::{fs, path::PathBuf};

use anyhow::{Result, ensure};
use clap::{Args, Parser, Subcommand, ValueEnum};
use mindframe_core::{Storyboard, Timeline};
use pipeline::Config;

#[derive(Parser)]
#[command(name = "mindframe", version, about = "把自己的知识材料变成可审改的视频")]
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
    /// Extract grounded key points and write an editable storyboard; no media API calls.
    Plan {
        input: PathBuf,
        #[arg(long)] out: PathBuf,
        #[arg(long, default_value = "mindframe.toml")] config: PathBuf,
    },
    /// Run planning, media production and MP4 rendering.
    Build {
        input: PathBuf,
        #[arg(long)] out: PathBuf,
        #[arg(long, default_value = "mindframe.toml")] config: PathBuf,
        #[command(flatten)] render: RenderOptions,
    },
    /// Produce media from an approved/edited storyboard without calling the LLM again.
    Produce {
        project: PathBuf,
        #[arg(long, default_value = "mindframe.toml")] config: PathBuf,
        #[command(flatten)] render: RenderOptions,
    },
    /// Re-render existing timed media without any model calls.
    Render {
        project: PathBuf,
        #[command(flatten)] render: RenderOptions,
    },
    /// Explicit offline content fixture with local eSpeak speech, not AI-generated knowledge.
    Demo {
        #[arg(long)] out: PathBuf,
        #[command(flatten)] render: RenderOptions,
    },
    /// Validate the storyboard and its literal source references.
    Validate { project: PathBuf },
    /// Generate JSON Schemas from the Rust contract types.
    Schema { #[arg(long, default_value = "schemas")] out: PathBuf },
}

fn main() -> Result<()> {
    match Cli::parse().command {
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
            let config = Config::load(&config)?;
            pipeline::check_renderer(&render)?;
            pipeline::produce(&project, &config)?;
            pipeline::render(&project, &render)?;
        }
        Command::Render { project, render } => {
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
        Command::Validate { project } => {
            pipeline::load_board(&project)?;
            println!("storyboard and source references: valid (semantic accuracy still needs human review)");
        }
        Command::Schema { out } => {
            fs::create_dir_all(&out)?;
            pipeline::write_json(&out.join("storyboard.schema.json"), &schemars::schema_for!(Storyboard))?;
            pipeline::write_json(&out.join("timeline.schema.json"), &schemars::schema_for!(Timeline))?;
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
