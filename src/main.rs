mod commands;
mod lint;
mod models;
mod site;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
  name = "design-md",
  version,
  about = "Manage, lint, and build design system documentation"
)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// List all available design systems
  List,
  /// Show a specific design system
  Show { name: String },
  /// Lint all design files for structure and contrast issues
  Lint {
    /// Automatically fix fixable issues
    #[arg(long)]
    fix: bool,
  },
  /// Build a static site from all designs
  Build {
    /// Output directory for the generated site
    #[arg(long, default_value = "out")]
    out_dir: String,
  },
  /// Scaffold a new design system
  New {
    /// Name of the design system (e.g., "figma")
    name: String,
    /// Source URL inspiration
    #[arg(long)]
    source: Option<String>,
  },
}

fn main() -> anyhow::Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Commands::List => commands::list::run()?,
    Commands::Show { name } => commands::show::run(&name)?,
    Commands::Lint { fix } => commands::lint::run(fix)?,
    Commands::Build { out_dir } => commands::build::run(&out_dir)?,
    Commands::New { name, source } => commands::new_::run(&name, source.as_deref())?,
  }

  Ok(())
}
