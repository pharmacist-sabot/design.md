use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use pulldown_cmark::{html, Parser};

use crate::models::DesignSystem;
use crate::site::assets;

pub fn render_all(designs: &[DesignSystem], out_dir: &Path) -> Result<()> {
  fs::create_dir_all(out_dir.join("designs"))
    .with_context(|| format!("Failed to create {}", out_dir.display()))?;
  fs::create_dir_all(out_dir.join("assets")).with_context(|| "Failed to create assets dir")?;

  assets::write_assets(out_dir)?;

  render_index(designs, out_dir)?;

  for design in designs {
    render_design(design, out_dir)?;
  }

  render_search_index(designs, out_dir)?;

  Ok(())
}

fn render_index(designs: &[DesignSystem], out_dir: &Path) -> Result<()> {
  let mut items = String::new();

  for d in designs {
    items.push_str(&format!(
      r##"
            <a href="designs/{slug}/index.html" class="design-card">
                <h2>{name}</h2>
                <p class="source">{source}</p>
                <p class="sections">{count} sections</p>
            </a>"##,
      slug = d.slug,
      name = d.name,
      source = d.source_inspiration,
      count = d.sections.len(),
    ));
  }

  let html = format!(
    r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Design System Collection</title>
    <link rel="stylesheet" href="assets/style.css">
</head>
<body>
    <header>
        <h1>Design System Collection</h1>
        <p>A curated collection of production-ready design system documentation</p>
        <input type="text" id="search" placeholder="Search designs..." onkeyup="filterDesigns()">
    </header>
    <main>
        <div class="design-grid" id="design-grid">
            {items}
        </div>
    </main>
    <footer>
        <p>Built with <a href="https://github.com/suradet-ps/design.md">design-md</a></p>
    </footer>
    <script src="assets/script.js"></script>
</body>
</html>"##
  );

  fs::write(out_dir.join("index.html"), &html).with_context(|| "Failed to write index.html")?;

  Ok(())
}

fn render_design(design: &DesignSystem, out_dir: &Path) -> Result<()> {
  let out_sub = out_dir.join("designs").join(&design.slug);
  fs::create_dir_all(&out_sub)
    .with_context(|| format!("Failed to create {}", out_sub.display()))?;

  let mut body = String::new();

  body.push_str(&format!(
    r##"<h1>{name}</h1>
        <p class="source">Source: {source}</p>
        <nav class="toc"><h3>Table of Contents</h3><ul>"##,
    name = design.name,
    source = design.source_inspiration,
  ));

  for section in &design.sections {
    body.push_str(&format!(
      r##"<li><a href="#section-{num}">{num}. {title}</a></li>"##,
      num = section.number,
      title = section.title,
    ));
  }

  body.push_str("</ul></nav>");

  for section in &design.sections {
    let parser = Parser::new(&section.content);
    let mut section_html = String::new();
    html::push_html(&mut section_html, parser);

    body.push_str(&format!(
      r##"<section id="section-{num}">
                <h2>{num}. {title}</h2>
                {content}
            </section>"##,
      num = section.number,
      title = section.title,
      content = section_html,
    ));
  }

  let html = format!(
    r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{name} — Design System Collection</title>
    <link rel="stylesheet" href="../../assets/style.css">
</head>
<body>
    <header>
        <a href="../../index.html" class="back-link">← Back to collection</a>
        <h1>{name}</h1>
    </header>
    <main>
        {body}
    </main>
    <script src="../../assets/script.js"></script>
</body>
</html>"##,
    name = design.name,
    body = body,
  );

  fs::write(out_sub.join("index.html"), &html)
    .with_context(|| format!("Failed to write {}/index.html", design.slug))?;

  Ok(())
}

fn render_search_index(designs: &[DesignSystem], out_dir: &Path) -> Result<()> {
  let search_data: Vec<serde_json::Value> = designs
    .iter()
    .map(|d| {
      serde_json::json!({
          "name": d.name,
          "slug": d.slug,
          "source": d.source_inspiration,
          "sections": d.sections.len(),
      })
    })
    .collect();

  let json = serde_json::to_string(&search_data)?;
  fs::write(out_dir.join("search.json"), &json).with_context(|| "Failed to write search.json")?;

  Ok(())
}
