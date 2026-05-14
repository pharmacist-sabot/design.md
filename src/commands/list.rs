use colored::Colorize;

use crate::utils;

pub fn run() -> anyhow::Result<()> {
  let designs = utils::discover_designs(&std::path::PathBuf::from("designs"))?;

  if designs.is_empty() {
    println!("No design systems found in designs/");
    return Ok(());
  }

  println!("{}", "Available Design Systems:".bold().green());
  println!("{}", "─".repeat(60));
  println!(
    "{:<20} {:<25} {}",
    "Name".bold(),
    "Source".bold(),
    "Sections".bold()
  );
  println!("{}", "─".repeat(60));

  for d in &designs {
    let source = if d.source_inspiration.is_empty() {
      "—".to_string()
    } else {
      truncate(&d.source_inspiration, 24)
    };
    println!(
      "{:<20} {:<25} {}",
      d.name.green(),
      source,
      d.sections.len().to_string().yellow()
    );
  }

  println!("{}", "─".repeat(60));
  println!("Total: {} design systems", designs.len());

  Ok(())
}

fn truncate(s: &str, max: usize) -> String {
  if s.len() <= max {
    s.to_string()
  } else {
    format!("{}…", &s[..max.saturating_sub(1)])
  }
}
