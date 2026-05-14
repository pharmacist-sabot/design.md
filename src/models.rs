use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignSystem {
  pub name: String,
  pub slug: String,
  pub source_inspiration: String,
  pub status: DesignStatus,
  pub sections: Vec<DesignSection>,
  pub path: PathBuf,
  pub raw_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DesignStatus {
  Complete,
  InProgress,
  Draft,
}

impl std::fmt::Display for DesignStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DesignStatus::Complete => write!(f, "Complete"),
      DesignStatus::InProgress => write!(f, "In Progress"),
      DesignStatus::Draft => write!(f, "Draft"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignSection {
  pub number: u8,
  pub title: String,
  pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintResult {
  pub design_name: String,
  pub passed: bool,
  pub checks: Vec<LintCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintCheck {
  pub name: String,
  pub passed: bool,
  pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPair {
  pub foreground: String,
  pub background: String,
  pub role: String,
}

pub const REQUIRED_SECTIONS: &[(u8, &str)] = &[
  (1, "Visual Theme & Atmosphere"),
  (2, "Color Palette & Roles"),
  (3, "Typography Rules"),
  (4, "Component Stylings"),
  (5, "Layout Principles"),
  (6, "Depth & Elevation"),
  (7, "Responsive Behavior"),
  (8, "Accessibility & States"),
  (9, "Agent Prompt Guide"),
];
