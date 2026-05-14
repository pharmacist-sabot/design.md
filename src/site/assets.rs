use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

pub fn write_assets(out_dir: &Path) -> Result<()> {
  write_css(out_dir)?;
  write_js(out_dir)?;
  Ok(())
}

fn write_css(out_dir: &Path) -> Result<()> {
  let css = r##":root {
  --bg: #ffffff;
  --text: #1a1a2e;
  --card-bg: #f8f9fa;
  --border: #e0e0e0;
  --accent: #4361ee;
  --accent-light: #eef0ff;
}

@media (prefers-color-scheme: dark) {
  :root {
    --bg: #0f0f1a;
    --text: #e0e0e0;
    --card-bg: #1a1a2e;
    --border: #2a2a3e;
    --accent: #6c8cff;
    --accent-light: #1a1a3e;
  }
}

* { margin: 0; padding: 0; box-sizing: border-box; }

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: var(--bg);
  color: var(--text);
  line-height: 1.6;
  max-width: 960px;
  margin: 0 auto;
  padding: 2rem 1.5rem;
}

header { margin-bottom: 2rem; }
header h1 { font-size: 2rem; margin-bottom: 0.5rem; }
header p { color: var(--text); opacity: 0.7; }
.back-link { display: inline-block; margin-bottom: 1rem; color: var(--accent); text-decoration: none; font-size: 0.9rem; }
.back-link:hover { text-decoration: underline; }

#search {
  width: 100%;
  padding: 0.75rem 1rem;
  border: 2px solid var(--border);
  border-radius: 8px;
  font-size: 1rem;
  background: var(--card-bg);
  color: var(--text);
  margin-top: 1rem;
  outline: none;
  transition: border-color 0.2s;
}
#search:focus { border-color: var(--accent); }

.design-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1rem;
  margin-top: 1.5rem;
}

.design-card {
  display: block;
  padding: 1.25rem;
  background: var(--card-bg);
  border: 1px solid var(--border);
  border-radius: 10px;
  text-decoration: none;
  color: var(--text);
  transition: transform 0.2s, box-shadow 0.2s;
}
.design-card:hover { transform: translateY(-2px); box-shadow: 0 4px 12px rgba(0,0,0,0.1); }
.design-card h2 { font-size: 1.2rem; margin-bottom: 0.3rem; }
.design-card .source { font-size: 0.85rem; opacity: 0.6; }
.design-card .sections { font-size: 0.8rem; margin-top: 0.5rem; color: var(--accent); }

footer { margin-top: 3rem; padding-top: 1.5rem; border-top: 1px solid var(--border); font-size: 0.85rem; opacity: 0.6; }
footer a { color: var(--accent); text-decoration: none; }

nav.toc {
  background: var(--card-bg);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 1rem;
  margin-bottom: 2rem;
}
nav.toc h3 { margin-bottom: 0.5rem; font-size: 0.95rem; }
nav.toc ul { list-style: none; }
nav.toc li { margin: 0.25rem 0; }
nav.toc a { color: var(--accent); text-decoration: none; font-size: 0.9rem; }
nav.toc a:hover { text-decoration: underline; }

section { margin-bottom: 2rem; }
section h2 { font-size: 1.4rem; margin-bottom: 0.75rem; padding-bottom: 0.3rem; border-bottom: 2px solid var(--accent); }
section p { margin-bottom: 0.75rem; }
section pre {
  background: var(--card-bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 1rem;
  overflow-x: auto;
  font-size: 0.85rem;
  margin-bottom: 1rem;
}
section code {
  background: var(--card-bg);
  padding: 0.15rem 0.35rem;
  border-radius: 3px;
  font-size: 0.85rem;
}
section pre code { background: none; padding: 0; }
section ul, section ol { margin-left: 1.5rem; margin-bottom: 0.75rem; }
section table { width: 100%; border-collapse: collapse; margin-bottom: 1rem; }
section th, section td { padding: 0.5rem; border: 1px solid var(--border); text-align: left; }
section th { background: var(--card-bg); font-weight: 600; }
"##;

  fs::write(out_dir.join("assets/style.css"), css).with_context(|| "Failed to write style.css")?;

  Ok(())
}

fn write_js(out_dir: &Path) -> Result<()> {
  let js = r##"async function filterDesigns() {
  const query = document.getElementById('search').value.toLowerCase();
  const cards = document.querySelectorAll('.design-card');

  cards.forEach(card => {
    const text = card.textContent.toLowerCase();
    card.style.display = text.includes(query) ? '' : 'none';
  });
}
"##;

  fs::write(out_dir.join("assets/script.js"), js).with_context(|| "Failed to write script.js")?;

  Ok(())
}
