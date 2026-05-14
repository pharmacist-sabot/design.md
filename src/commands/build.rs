use std::path::PathBuf;

use colored::Colorize;

use crate::site;
use crate::utils;

pub fn run(out_dir: &str) -> anyhow::Result<()> {
  let designs = utils::discover_designs(&PathBuf::from("designs"))?;

  if designs.is_empty() {
    eprintln!(
      "{} No design systems found in designs/",
      "Error:".red().bold()
    );
    std::process::exit(1);
  }

  let out_path = PathBuf::from(out_dir);

  site::renderer::render_all(&designs, &out_path)?;

  println!(
    "{} Generated site for {} design(s) in {}",
    "✓".green().bold(),
    designs.len().to_string().cyan(),
    out_path.display().to_string().cyan()
  );

  Ok(())
}
