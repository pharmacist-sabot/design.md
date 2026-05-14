# Design System Collection

![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)
![Status](https://img.shields.io/badge/status-active-brightgreen.svg?style=flat-square)
![Rust](https://img.shields.io/badge/rust-1.80+-orange.svg?style=flat-square)

> A curated collection of production-ready design system documentation, inspired by world-class products — with a Rust CLI to manage, lint, and generate a static site.

---

## Overview

This repository houses comprehensive, implementation-ready design system specifications organized under `designs/`. Each design is in its own directory (`designs/<name>/DESIGN.md`) and follows a consistent 9-section structure.

The [`design-md`](./) CLI tool (written in Rust) helps you:
- **List** all available design systems
- **Lint** them for structural issues and WCAG contrast compliance
- **Build** a beautiful static documentation site
- **Scaffold** new design systems from a template

---

## Quick Start

```bash
# List all designs
cargo run -- list

# Lint all designs
cargo run -- lint

# Generate static site
cargo run -- build

# Scaffold a new design
cargo run -- new <name> --source <url>
```

Open `out/index.html` in your browser after running `build`.

---

## Available Design Systems

| Design System | Path | Status |
|--------------|------|--------|
| [Codemod](./designs/codemod/DESIGN.md) | `designs/codemod/` | ✅ Complete |
| [Dataforest](./designs/dataforest/DESIGN.md) | `designs/dataforest/` | ✅ Complete |
| [El Patita](./designs/elpatita/DESIGN.md) | `designs/elpatita/` | ✅ Complete |
| [JetBrains](./designs/jetbrains/DESIGN.md) | `designs/jetbrains/` | ✅ Complete |
| [Logto](./designs/logto/DESIGN.md) | `designs/logto/` | ✅ Complete |
| [Neon](./designs/neon/DESIGN.md) | `designs/neon/` | ✅ Complete |
| [Node.js](./designs/nodejs/DESIGN.md) | `designs/nodejs/` | ✅ Complete |
| [Nuxt](./designs/nuxt/DESIGN.md) | `designs/nuxt/` | ✅ Complete |
| [Pnpm](./designs/pnpm/DESIGN.md) | `designs/pnpm/` | ✅ Complete |
| [Zed](./designs/zed/DESIGN.md) | `designs/zed/` | ✅ Complete |

---

## CLI Reference

### `design-md list`
Lists all design systems found under `designs/`, showing name, source, and section count.

### `design-md show <name>`
Display a summary of a specific design system with its sections.

### `design-md lint [--fix]`
Validates all designs against:
- **Structure**: All 9 required sections present
- **Contrast**: WCAG AA (4.5:1) ratio compliance for color pairs

### `design-md build [--out-dir <path>]`
Generates a static HTML site from all designs:
- Responsive grid index with search/filter
- Individual design pages with rendered Markdown
- Dark/light mode support
- Full-text search index (`search.json`)

### `design-md new <name> --source <url>`
Scaffolds a new design system:
- Creates `designs/<name>/DESIGN.md` from template
- Pre-fills source URL and name

---

## Design System Structure

Each design follows a consistent structure with 9 sections:

```
designs/<product>/DESIGN.md
├── 1. Visual Theme & Atmosphere    # Philosophy, key characteristics
├── 2. Color Palette & Roles        # Tokens, semantic colors, states
├── 3. Typography Rules             # Fonts, scale, hierarchy, OpenType
├── 4. Component Stylings           # Buttons, cards, inputs, nav, code
├── 5. Layout Principles            # Spacing, grid, whitespace, radius
├── 6. Depth & Elevation            # Shadow system, decorative depth
├── 7. Responsive Behavior          # Breakpoints, touch targets, collapsing
├── 8. Accessibility & States       # Focus, contrast, motion preferences
└── 9. Agent Prompt Guide           # Quick refs, example prompts, iteration rules
```

---

## Project Structure

```
.
├── Cargo.toml              # Rust project config
├── src/
│   ├── main.rs             # CLI entry point
│   ├── commands/           # CLI command implementations
│   ├── lint/               # Lint validators (structure, contrast)
│   ├── site/               # Static site generator (renderer, assets)
│   ├── models.rs           # Core data structures
│   └── utils.rs            # Design discovery, markdown parsing
├── designs/                # All design system files
│   └── <name>/DESIGN.md
├── template/               # Template for new designs
├── out/                    # Generated site output (gitignored)
└── README.md
```

---

## Contributing

We welcome contributions! Whether you're fixing a typo, adding a new component, or proposing an entirely new design system:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/add-figma-system`)
3. **Use** `cargo run -- new <name> --source <url>` to scaffold
4. **Fill in** the 9 sections with implementation-ready content
5. **Run** `cargo run -- lint` to validate
6. **Submit** a Pull Request with a clear description

### Contribution Guidelines
- ✅ Use English for all documentation
- ✅ Include hex codes, CSS snippets, and token names
- ✅ Specify both Light and Dark mode values where applicable
- ✅ Reference WCAG contrast ratios for color pairs
- ❌ Avoid subjective language ("beautiful", "elegant")—focus on measurable specs

---

## License

This project is licensed under the **MIT License**.

You are free to:
- ✅ Use these design specifications in personal or commercial projects
- ✅ Modify and adapt tokens to fit your brand
- ✅ Share and distribute with attribution

You must:
- ℹ️ Include the original license and copyright notice
- 🚫 Not hold the authors liable for any damages

---

## Get in Touch

- **Suggest a design system**: Open an Issue with the product name and URL
- **Report an issue**: Use the [Issues tab](https://github.com/suradet-ps/design.md/issues)
- **Discuss ideas**: Start a [Discussion](https://github.com/suradet-ps/design.md/discussions)
