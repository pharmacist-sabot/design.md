# Design System Collection

![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)
![Status](https://img.shields.io/badge/status-active-brightgreen.svg?style=flat-square)
![Contributions](https://img.shields.io/badge/contributions-welcome-orange.svg?style=flat-square)

> A curated collection of production-ready design system documentation, inspired by world-class products and engineered for developer experience.

---

## Overview

This repository houses comprehensive, implementation-ready design system specifications. Each design document is crafted to mirror the visual language, interaction patterns, and engineering principles of leading software products—translated into actionable guidelines for designers and developers.

**Why this exists:**
- Reference-quality design tokens, components, and patterns
- Developer-friendly specifications with CSS-ready values
- Systematic approach to spacing, typography, color, and elevation
- Accessibility-first: WCAG-compliant contrast and focus management
- Dual-mode support: Light and Dark mode token definitions

---

## Available Design Systems

| Design System | Source Inspiration | Status |
|--------------|------------------|--------|
| [**Dataforest-Inspired**](./DESIGN-DATAFOREST.md) | [dataforest.net](https://cloud.dataforest.net) | ✅ Complete |
| [**JetBrains-Inspired**](./DESIGN-JETBRAINS.md) | [jetbrains.com](https://www.jetbrains.com) | ✅ Complete |
| [**Logto-Inspired**](./DESIGN-LOGTO.md) | [logto.io](https://logto.io) | ✅ Complete |
| [**Neon-Inspired**](./DESIGN-NEON.md) | [neon.com](https://neon.com) | ✅ Complete |
| [**Node.js-Inspired**](./DESIGN-NODEJS.md) | [nodejs.org](https://nodejs.org) | ✅ Complete |
| [**Nuxt-Inspired**](./DESIGN-NUXT.md) | [nuxt.com](https://nuxt.com) | ✅ Complete |

---

## Quick Start

### Using a Design System

1. **Browse** the design specification file (e.g., `DESIGN-JETBRAINS.md`)
2. **Extract tokens** from the Color Palette or Typography sections
3. **Implement components** using the provided CSS snippets and guidelines
4. **Adapt** spacing, radius, and elevation scales to your project needs

---

## Design System Structure

Each design specification follows a consistent, navigable structure:

```
DESIGN-[PRODUCT].md
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

## Key Principles

### For Designers
- **Consistency over creativity**: Reuse tokens, don't invent new values
- **Hierarchy through scale**: Let typography and spacing communicate importance
- **Subtlety in depth**: Shadows and borders should support, not distract
- **Accessibility by default**: Contrast, focus, and motion preferences are non-negotiable

### For Developers
- **Tokens first**: Use CSS custom properties for all design values
- **Responsive by design**: Implement fluid type and spacing with `clamp()`
- **Dark mode ready**: Structure styles to support `prefers-color-scheme`
- **Performance conscious**: Prefer CSS transitions over JS animations

---

## Contributing

We welcome contributions! Whether you're fixing a typo, adding a new component, or proposing an entirely new design system:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/add-figma-system`)
3. **Follow** the existing structure and formatting conventions
4. **Test** your changes locally (preview the Markdown)
5. **Submit** a Pull Request with a clear description

### Adding a New Design System

1. Create a new file: `DESIGN-[PRODUCT].md`
2. Use the [template structure](#-design-system-structure) above
3. Include all 9 sections with detailed, implementation-ready content
4. Update the [Available Design Systems](#-available-design-systems) table
5. Add relevant badges to the top of your file (optional)

### Contribution Guidelines

- ✅ Use English for all documentation
- ✅ Include hex codes, CSS snippets, and token names
- ✅ Specify both Light and Dark mode values where applicable
- ✅ Reference WCAG contrast ratios for color pairs
- ❌ Avoid subjective language ("beautiful", "elegant")—focus on measurable specs

---

## License

This project is licensed under the **MIT License**—see the [LICENSE](./LICENSE) file for details.

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
- **Report an issue**: Use the [Issues tab](https://github.com/pharmacist-sabot/design.md/issues)
- **Discuss ideas**: Start a [Discussion](https://github.com/pharmacist-sabot/design.md/discussions)

