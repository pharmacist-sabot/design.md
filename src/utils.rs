use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::models::DesignSystem;

pub fn discover_designs(designs_dir: &Path) -> Result<Vec<DesignSystem>> {
  let mut designs = Vec::new();

  if !designs_dir.exists() {
    return Ok(designs);
  }

  for entry in WalkDir::new(designs_dir)
    .max_depth(3)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    if entry.file_name() == "DESIGN.md" {
      let path = entry.path().to_path_buf();
      let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;

      let slug = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

      let name = slug.to_uppercase();
      let source = extract_source(&content);

      let sections = extract_sections(&content);

      designs.push(DesignSystem {
        name,
        slug,
        source_inspiration: source,
        status: crate::models::DesignStatus::Complete,
        sections,
        path,
        raw_content: content,
      });
    }
  }

  designs.sort_by(|a, b| a.slug.cmp(&b.slug));
  Ok(designs)
}

pub fn extract_source(content: &str) -> String {
  for line in content.lines() {
    if line.contains("Source Inspiration") {
      if let Some(url) = line.split('|').nth(2) {
        return url.trim().to_string();
      }
    }
  }
  String::new()
}

pub fn extract_sections(content: &str) -> Vec<crate::models::DesignSection> {
  let mut sections = Vec::new();
  let mut current_num: Option<u8> = None;
  let mut current_title = String::new();
  let mut current_lines: Vec<String> = Vec::new();
  let mut in_section = false;

  for line in content.lines() {
    if line.starts_with("## ") {
      if let Some(num) = current_num {
        sections.push(crate::models::DesignSection {
          number: num,
          title: current_title.clone(),
          content: current_lines.join("\n"),
        });
      }
      current_lines.clear();

      let header = &line[3..];
      if let Some((num_str, rest)) = header.split_once(". ") {
        if let Ok(num) = num_str.trim().parse::<u8>() {
          current_num = Some(num);
          current_title = rest.to_string();
          in_section = true;
          continue;
        }
      }
      in_section = false;
    }

    if in_section {
      current_lines.push(line.to_string());
    }
  }

  if let Some(num) = current_num {
    sections.push(crate::models::DesignSection {
      number: num,
      title: current_title.clone(),
      content: current_lines.join("\n"),
    });
  }

  sections
}

pub fn read_template(template_path: &Path) -> Result<String> {
  if template_path.exists() {
    fs::read_to_string(template_path).with_context(|| "Failed to read template")
  } else {
    Ok(DEFAULT_TEMPLATE.to_string())
  }
}

const DEFAULT_TEMPLATE: &str = r#"# DESIGN-[NAME]

> A design system inspired by [Product Name](https://example.com)

## Design System Overview

**Source Inspiration:** | [Product Name](https://example.com) |
**Status:** | Draft |

---

## 1. Visual Theme & Atmosphere

<!-- Describe the visual philosophy and key characteristics -->

## 2. Color Palette & Roles

<!-- Token definitions, semantic colors, light/dark values -->

## 3. Typography Rules

<!-- Font stack, scale, hierarchy -->

## 4. Component Stylings

<!-- Buttons, cards, inputs, navigation -->

## 5. Layout Principles

<!-- Spacing, grid, whitespace -->

## 6. Depth & Elevation

<!-- Shadow system, layering -->

## 7. Responsive Behavior

<!-- Breakpoints, touch targets -->

## 8. Accessibility & States

<!-- Focus, contrast, motion preferences -->

## 9. Agent Prompt Guide

<!-- Quick reference prompts for AI agents -->
"#;
