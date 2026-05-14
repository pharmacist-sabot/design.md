use regex::Regex;

use crate::models::{DesignSystem, LintCheck, REQUIRED_SECTIONS};

pub fn check_sections(design: &DesignSystem) -> LintCheck {
  let mut missing = Vec::new();
  let mut found_extra = false;

  for (num, title) in REQUIRED_SECTIONS {
    let found = design.sections.iter().any(|s| s.number == *num);
    if !found {
      missing.push(format!("  {}. {}", num, title));
    }
  }

  if design.sections.len() > REQUIRED_SECTIONS.len() {
    found_extra = true;
  }

  let mut messages = Vec::new();
  if !missing.is_empty() {
    messages.push(format!("Missing sections:\n{}", missing.join("\n")));
  }
  if found_extra {
    messages.push(format!(
      "Found {} extra section(s) beyond the 9 standard sections",
      design.sections.len() - REQUIRED_SECTIONS.len()
    ));
  }

  let all_section_headers_present = check_section_headers(design, &mut messages);

  let passed = missing.is_empty() && !found_extra && all_section_headers_present;

  LintCheck {
    name: "structure".to_string(),
    passed,
    message: if messages.is_empty() {
      None
    } else {
      Some(messages.join("; "))
    },
  }
}

fn check_section_headers(design: &DesignSystem, messages: &mut Vec<String>) -> bool {
  let header_re = Regex::new(r"^## \d+\. ").unwrap();
  let mut all_good = true;
  let mut found_headers = 0;

  for line in design.raw_content.lines() {
    if header_re.is_match(line) {
      found_headers += 1;
    }
  }

  if found_headers < 9 {
    messages.push(format!(
      "Found only {} section headers (expected at least 9)",
      found_headers
    ));
    all_good = false;
  }

  all_good
}
