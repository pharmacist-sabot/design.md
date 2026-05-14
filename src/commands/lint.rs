use colored::Colorize;

use crate::lint;
use crate::utils;

pub fn run(_fix: bool) -> anyhow::Result<()> {
  let designs = utils::discover_designs(&std::path::PathBuf::from("designs"))?;

  if designs.is_empty() {
    println!("No design systems found in designs/");
    return Ok(());
  }

  let mut all_passed = true;

  for design in &designs {
    let result = lint::check_design(design);

    if result.passed {
      println!(
        "{} {} {}",
        "✓".green().bold(),
        design.name.cyan(),
        "(all checks passed)".dimmed()
      );
    } else {
      all_passed = false;
      println!(
        "{} {} {}",
        "✗".red().bold(),
        design.name.red(),
        "(failures below)".dimmed()
      );

      for check in &result.checks {
        if !check.passed {
          let msg = check.message.as_deref().unwrap_or("Unknown issue");
          println!("  {} {} {}", "•".red(), check.name.red(), msg.dimmed());
        }
      }
    }
  }

  println!();
  if all_passed {
    println!("{} All designs passed linting!", "✓".green().bold());
  } else {
    println!("{} Some designs have linting issues.", "✗".red().bold());
    std::process::exit(1);
  }

  Ok(())
}
