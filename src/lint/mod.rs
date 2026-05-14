mod contrast;
mod structure;

use crate::models::{DesignSystem, LintResult};

pub fn check_design(design: &DesignSystem) -> LintResult {
  let mut checks = Vec::new();

  checks.push(structure::check_sections(design));
  checks.push(contrast::check_colors(design));

  let passed = checks.iter().all(|c| c.passed);

  LintResult {
    design_name: design.name.clone(),
    passed,
    checks,
  }
}
