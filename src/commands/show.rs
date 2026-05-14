use colored::Colorize;

use crate::utils;

pub fn run(name: &str) -> anyhow::Result<()> {
  let designs = utils::discover_designs(&std::path::PathBuf::from("designs"))?;
  let slug = name.to_lowercase();

  let design = designs.iter().find(|d| d.slug == slug);

  match design {
    Some(d) => {
      println!("{}", d.name.bold().green());
      println!("{}", "═".repeat(d.name.len()));
      println!("Source: {}", d.source_inspiration.cyan());
      println!("Path: {}", d.path.display().to_string().dimmed());
      println!();

      for section in &d.sections {
        println!(
          "  {} {}",
          format!("{}.", section.number).bold().blue(),
          section.title.bold()
        );
      }
    }
    None => {
      eprintln!(
        "{} Design '{}' not found in designs/",
        "Error:".red().bold(),
        name
      );
      std::process::exit(1);
    }
  }

  Ok(())
}
