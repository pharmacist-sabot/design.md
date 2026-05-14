use regex::Regex;

use crate::models::{ColorPair, DesignSystem, LintCheck};

pub fn check_colors(design: &DesignSystem) -> LintCheck {
  let color_pairs = extract_color_pairs(&design.raw_content);
  let mut issues = Vec::new();

  for pair in &color_pairs {
    let ratio = relative_luminance_ratio(&pair.foreground, &pair.background);
    if ratio < 4.5 {
      issues.push(format!(
        "{} on {} ({:.2}:1) — fails WCAG AA (needs 4.5:1) — role: {}",
        pair.foreground, pair.background, ratio, pair.role
      ));
    }
  }

  let passed = issues.is_empty();
  LintCheck {
    name: "contrast".to_string(),
    passed,
    message: if issues.is_empty() {
      None
    } else {
      Some(format!("Contrast issues:\n{}", issues.join("\n")))
    },
  }
}

fn extract_color_pairs(content: &str) -> Vec<ColorPair> {
  let mut pairs = Vec::new();
  let hex_re = Regex::new(r"(?i)#[0-9a-f]{6}(?:[0-9a-f]{2})?").unwrap();
  let mut colors: Vec<String> = Vec::new();

  for line in content.lines() {
    for cap in hex_re.find_iter(line) {
      let color = cap.as_str().to_lowercase();
      if color.len() == 7 {
        colors.push(color);
      }
    }
  }

  for i in 0..colors.len().saturating_sub(1) {
    pairs.push(ColorPair {
      foreground: colors[i].clone(),
      background: colors[i + 1].clone(),
      role: "adjacent colors".to_string(),
    });
  }

  if pairs.len() > 10 {
    pairs.truncate(10);
  }

  pairs
}

fn relative_luminance(hex: &str) -> f64 {
  let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0) as f64 / 255.0;
  let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0) as f64 / 255.0;
  let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0) as f64 / 255.0;

  let r = if r <= 0.04045 {
    r / 12.92
  } else {
    ((r + 0.055) / 1.055).powf(2.4)
  };
  let g = if g <= 0.04045 {
    g / 12.92
  } else {
    ((g + 0.055) / 1.055).powf(2.4)
  };
  let b = if b <= 0.04045 {
    b / 12.92
  } else {
    ((b + 0.055) / 1.055).powf(2.4)
  };

  0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn relative_luminance_ratio(hex1: &str, hex2: &str) -> f64 {
  let l1 = relative_luminance(hex1);
  let l2 = relative_luminance(hex2);

  let lighter = l1.max(l2);
  let darker = l1.min(l2);

  (lighter + 0.05) / (darker + 0.05)
}
