use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use colored::Colorize;

use crate::utils;

pub fn run(name: &str, source: Option<&str>) -> anyhow::Result<()> {
  let slug = name.to_lowercase().replace(' ', "-");
  let dir = PathBuf::from("designs").join(&slug);

  if dir.exists() {
    eprintln!(
      "{} Design '{}' already exists at designs/{}/",
      "Error:".red().bold(),
      name,
      slug
    );
    std::process::exit(1);
  }

  fs::create_dir_all(&dir)
    .with_context(|| format!("Failed to create directory designs/{}", slug))?;

  let mut content = utils::read_template(&PathBuf::from("template/DESIGN-TEMPLATE.md"))?;
  content = content.replace("[NAME]", &name.to_uppercase());
  content = content.replace("[Product Name]", name);

  if let Some(url) = source {
    content = content.replace("https://example.com", url);
  }

  let file_path = dir.join("DESIGN.md");
  fs::write(&file_path, &content)
    .with_context(|| format!("Failed to write {}", file_path.display()))?;

  println!(
    "{} Created new design '{}' at {}",
    "✓".green().bold(),
    name,
    file_path.display().to_string().cyan()
  );

  Ok(())
}
