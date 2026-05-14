# Design System Inspired by Zed

## 1. Visual Theme & Atmosphere

Zed's design embodies the ethos of high-performance engineering: precise, minimal, and purposeful. The visual language prioritizes clarity and speed, stripping away decorative elements to focus attention on code and collaboration. The interface is built on a foundation of deep, cool neutrals rather than warm tones, creating a distinctly technical atmosphere that feels like a well-tuned instrument rather than a cozy notebook.

The page canvas defaults to near-black (`#0a0a0b` or `#121417`) in dark mode, with text rendered in high-contrast whites and grays (`#f1f2f4`, `#b8bdc7`) that maximize readability without eye strain. The cool gray scale (`#121417`, `#2c2e30`, `#606368`, `#9a9ea6`) carries subtle blue undertones, reinforcing the technical, precision-focused identity.

Typography is the backbone of Zed's system. The interface uses **I A Writer Quattro** for headings and UI text—a refined, highly-legible sans-serif with excellent character distinction—paired with **JetBrains Mono** or similar monospaced fonts for code. Headlines use tight letter-spacing and bold weights to create impact without ornamentation. OpenType features like `calt` (contextual alternates) and `liga` (ligatures) are enabled for code, enhancing the reading experience for developers.

What makes Zed's visual language distinctive is its **functional minimalism**. Borders are crisp `1px solid` lines in low-opacity grays (`rgba(255,255,255,0.1)`), creating clear separation without visual weight. Shadows are nearly absent—depth is communicated through subtle background shifts and layered surfaces rather than drop shadows. The result is an interface that feels fast, direct, and unobtrusive.

**Key Characteristics:**
- I A Writer Quattro for UI, JetBrains Mono for code—technical typography first
- Cool neutral palette: grays with blue undertones (`#121417` deep base, `#f1f2f4` light surface)
- High-contrast text via `#f1f2f4` on dark backgrounds—optimized for long coding sessions
- Crisp borders: `1px solid rgba(255,255,255,0.1)`—functional division without decoration
- Minimal to no shadows—depth through layering and background contrast, not elevation
- Zed Blue (`#1348DC` / `#0751cf`) as the singular accent for CTAs, links, and interactive states
- Sharp corners (4px radius) for buttons and inputs—precision over softness
- 4px base spacing unit with a tight, consistent scale for dense information layout

---

## 2. Color Palette & Roles

### Primary
- **Zed Black** (`#0a0a0b` / `#121417`): Primary background, editor surface, dark mode canvas. Deep, cool, and non-reflective.
- **Pure White** (`#ffffff`): Light mode background, button text on blue, high-contrast accents.
- **Zed Blue** (`#1348DC` / `#0751cf`): Primary CTA, link color, interactive accent—the only saturated color in core UI.

### Brand Secondary
- **Deep Blue** (`#022467`): Hover/active state for Zed Blue, secondary emphasis.
- **Active Blue** (`#043a99`): Pressed button state, focused interactive elements.

### Cool Neutral Scale (Dark Mode)
- **Deep Base** (`#121417`): Primary editor background, main surface.
- **Surface Dark** (`#1e2024`): Secondary surfaces, panels, dropdowns.
- **Surface Medium** (`#2c2e30`): Hover states, selected items, subtle elevation.
- **Border Gray** (`#45474c`): Dividers, borders, subtle separation.
- **Text Secondary** (`#606368`): Secondary labels, metadata, disabled text.
- **Text Muted** (`#9a9ea6`): Placeholder text, captions, low-emphasis content.

### Cool Neutral Scale (Light Mode)
- **Light Base** (`#ffffff`): Primary background in light mode.
- **Surface Light** (`#f1f2f4`): Secondary surfaces, cards, subtle backgrounds.
- **Border Light** (`#b8bdc7`): Dividers, borders in light mode.
- **Text Dark** (`#121417`): Primary text in light mode.
- **Text Secondary Light** (`#45474c`): Secondary text, labels.
- **Text Muted Light** (`#7c8087`): Placeholder, captions.

### Semantic Accent Colors
- **Success Green** (`#22c55e`): Success states, completed actions, positive indicators.
- **Warning Orange** (`#f97316`): Warning states, attention indicators.
- **Error Red** (`#ef4444`): Errors, destructive actions, critical alerts.
- **Info Cyan** (`#06b6d4`): Informational messages, neutral highlights.
- **Purple Accent** (`#8b5cf6`): Premium features, experimental tags.

### Interactive States
- **Link Blue** (`#0751cf`): Primary link color, underline on hover.
- **Link Hover** (`#043a99`): Darker blue for hover/active link states.
- **Focus Ring** (`#0751cf` with `2px` outline): Keyboard focus indicator.
- **Selection Blue** (`rgba(7, 81, 207, 0.2)`): Text selection background.
- **Hover Surface** (`rgba(255,255,255,0.05)` dark / `rgba(0,0,0,0.05)` light): Subtle hover background.

### Borders & Dividers
- **Primary Border** (`1px solid rgba(255,255,255,0.1)` dark / `1px solid rgba(0,0,0,0.1)` light): Standard division.
- **Strong Border** (`1px solid rgba(255,255,255,0.2)`): Emphasized separation, active states.
- **Subtle Border** (`1px solid rgba(255,255,255,0.05)`): Very light division for nested content.

---

## 3. Typography Rules

### Font Family
- **UI / Headings**: `I A Writer Quattro V`, fallbacks: `Inter, -apple-system, system-ui, Segoe UI, Helvetica, Arial, sans-serif`
- **Code / Editor**: `JetBrains Mono, Fira Code, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace`
- **OpenType Features**: `"calt"`, `"liga"` enabled for code fonts; `"lnum"` for UI numerals

### Hierarchy

| Role | Font | Size | Weight | Line Height | Letter Spacing | Notes |
|------|------|------|--------|-------------|----------------|-------|
| Display Hero | I A Writer Quattro | 56px (3.5rem) | 700 | 1.10 | -0.5px | Maximum impact, landing page headlines |
| Section Heading | I A Writer Quattro | 40px (2.5rem) | 700 | 1.20 | -0.3px | Feature sections, major dividers |
| Sub-heading | I A Writer Quattro | 28px (1.75rem) | 600 | 1.30 | -0.2px | Card titles, content headers |
| Body Large | I A Writer Quattro | 20px (1.25rem) | 500 | 1.40 | normal | Introductions, feature descriptions |
| Body | I A Writer Quattro | 16px (1.00rem) | 400 | 1.50 | normal | Standard UI text, documentation |
| UI Label | I A Writer Quattro | 14px (0.875rem) | 500 | 1.43 | 0.1px | Buttons, navigation, form labels |
| Caption | I A Writer Quattro | 13px (0.8125rem) | 400 | 1.40 | 0.15px | Metadata, timestamps, helper text |
| Code Block | JetBrains Mono | 14px (0.875rem) | 400 | 1.60 | normal | Editor text, inline code, terminals |
| Code Small | JetBrains Mono | 12px (0.75rem) | 400 | 1.50 | normal | Inline snippets, annotations |
| Badge | I A Writer Quattro | 12px (0.75rem) | 600 | 1.33 | 0.2px | Status indicators, tags, version labels |

### Principles
- **Precision spacing**: Tight line-heights at large sizes (1.10–1.30) create dense, impactful headlines; relaxed spacing at code sizes (1.60) enhances readability.
- **Weight hierarchy**: 400 (body/reading), 500 (UI/labels), 600 (emphasis), 700 (headings). Clear, functional distinction without excess.
- **Monospace prominence**: Code fonts are first-class citizens—sized and spaced for long-duration reading, with ligatures enabled for common operator sequences.
- **Micro-tracking for small text**: 12–13px text uses slight positive letter-spacing (0.1–0.2px) to improve legibility at small sizes.
- **No decorative flourishes**: No italics for emphasis in UI (reserved for code comments), no all-caps headings—clarity over style.

---

## 4. Component Stylings

### Buttons

**Primary Blue**
- Background: `#1348DC` (Zed Blue)
- Text: `#ffffff`
- Padding: `10px 20px`
- Radius: `4px` (sharp, technical)
- Border: `1px solid transparent`
- Hover: background `#043a99`, subtle scale `1.02`
- Active: background `#022467`, scale `0.98`
- Focus: `2px solid #0751cf` outline + subtle glow
- Use: Primary CTAs ("Download", "Get Started")

**Secondary / Outline**
- Background: transparent
- Text: `#f1f2f4` (dark) / `#121417` (light)
- Border: `1px solid rgba(255,255,255,0.2)` (dark) / `1px solid rgba(0,0,0,0.2)` (light)
- Padding: `10px 20px`
- Radius: `4px`
- Hover: border brightens to `rgba(255,255,255,0.4)`, text shifts to white
- Use: Secondary actions, cancel buttons

**Ghost / Text Button**
- Background: transparent
- Text: `#9a9ea6` (muted) → `#f1f2f4` on hover
- No border
- Padding: `6px 12px`
- Use: Tertiary actions, inline links, menu items

**Pill Badge**
- Background: `rgba(7, 81, 207, 0.15)` (tinted blue)
- Text: `#60a5fa`
- Padding: `4px 10px`
- Radius: `999px` (full pill)
- Font: 12px weight 600
- Use: Status labels ("New", "Beta"), version tags

### Cards & Containers
- Background: `#1e2024` (dark) / `#ffffff` (light)
- Border: `1px solid rgba(255,255,255,0.1)` (dark) / `1px solid rgba(0,0,0,0.1)` (light)
- Radius: `8px` (standard), `12px` (featured)
- Shadow: **None or minimal**—depth via background contrast only
- Hover: border brightens to `rgba(255,255,255,0.2)`, subtle background shift
- Code cards: monospace font, syntax highlighting, copy button top-right

### Inputs & Forms
- Background: `#1e2024` (dark) / `#ffffff` (light)
- Text: `#f1f2f4` (dark) / `#121417` (light)
- Border: `1px solid rgba(255,255,255,0.15)` (dark) / `1px solid rgba(0,0,0,0.2)` (light)
- Padding: `10px 14px`
- Radius: `4px`
- Focus: `2px solid #0751cf` outline, border brightens
- Placeholder: `#606368` (dark) / `#7c8087` (light)

### Navigation
- Clean horizontal nav on transparent/dark background
- Brand logo left-aligned (Zed logomark + wordmark in white/blue)
- Links: I A Writer Quattro 14–16px weight 500, `#b8bdc7` text
- Hover: text shifts to `#f1f2f4`, subtle underline
- CTA: blue pill button ("Download") right-aligned
- Mobile: hamburger collapse with slide-in panel
- Dropdowns: sharp corners, subtle border, no shadows

### Code & Terminal Treatment
- Monospace font with ligatures enabled
- Syntax highlighting via semantic tokens (not just colors)
- Line numbers in muted gray, right-aligned
- Cursor: `2px` solid block in Zed Blue, subtle blink
- Selection: `rgba(7, 81, 207, 0.2)` background, no border
- Terminal: deep black background (`#0a0a0b`), green prompt, monospace text

### Distinctive Components

**Feature Cards with Code Previews**
- Large code snippet headers (syntax-highlighted TypeScript/Rust)
- 8px radius card with crisp border
- Title at 20px weight 600, description at 14px weight 400
- Optional "Try in Zed" CTA button bottom-right

**Testimonial / Quote Cards**
- Minimal layout: avatar (40px circle), name/title, quote text
- No decorative quotes—clean typography only
- Optional company logo in muted gray

**Metric / Stat Cards**
- Large number display (e.g., "79,248") in 40px weight 700
- Label below in 14px muted gray
- Optional trend indicator (↑ green / ↓ red)
- Crisp border container, no shadow

**Extension / Theme Gallery Grid**
- Card grid with extension icon (48px), name, description, install count
- Hover: subtle border brighten + scale `1.01`
- "Install" button appears on hover (blue pill)

---

## 5. Layout Principles

### Spacing System
- Base unit: `4px`
- Scale: `4px, 8px, 12px, 16px, 24px, 32px, 48px, 64px, 96px, 128px`
- Tight, consistent increments—no fractional values, no organic scaling

### Grid & Container
- Max content width: `1200px` (centered)
- Hero: centered single-column, generous vertical padding (`96–128px`)
- Feature sections: 2–3 column responsive grids
- Full-width dark sections for visual rhythm
- Code snippets contained with crisp border, no shadow

### Whitespace Philosophy
- **Dense but breathable**: Content blocks are compact (line-height 1.4–1.5) but surrounded by generous margins.
- **Functional alternation**: Dark sections alternate with slightly lighter surfaces (`#1e2024` → `#121417`), creating rhythm without color shifts.
- **Code-first density**: Editor UI is intentionally dense to maximize visible code; surrounding chrome is minimal to reduce distraction.

### Border Radius Scale
- Sharp (0px): Decorative elements, full-bleed sections
- Micro (2px): Inline elements, tags
- Standard (4px): Buttons, inputs, functional interactive elements
- Comfortable (8px): Cards, containers, panels
- Large (12px): Hero cards, featured content
- Full Pill (999px): Badges, status indicators

---

## 6. Depth & Elevation

| Level | Treatment | Use |
|-------|-----------|-----|
| Flat (Level 0) | No shadow, no border | Page background, text blocks |
| Division (Level 1) | `1px solid rgba(255,255,255,0.1)` | Standard borders, card outlines, dividers |
| Surface (Level 2) | Background shift (`#1e2024` → `#2c2e30`) | Hover states, active panels |
| Modal (Level 3) | Background overlay `rgba(0,0,0,0.5)` + border | Modals, dropdowns, context menus |
| Focus (Accessibility) | `2px solid #0751cf` outline | Keyboard focus on interactive elements |

**Depth Philosophy**: Zed rejects traditional shadow-based elevation. Depth is communicated through:
1. **Background contrast**: Slightly lighter surfaces appear "closer"
2. **Border hierarchy**: Brighter borders indicate interactive or focused states
3. **Overlay modals**: Dark semi-transparent overlays create clear modal hierarchy
4. **No drop shadows**: Avoids visual noise and GPU cost—performance-first design

### Decorative Elements
- Minimal illustrations: abstract geometric shapes, subtle gradients
- No decorative icons in chrome—icons are functional only
- Brand elements (Zed logomark) used sparingly for recognition

---

## 7. Responsive Behavior

### Breakpoints
| Name | Width | Key Changes |
|------|-------|-------------|
| Mobile Small | `<400px` | Single column, minimal padding, condensed nav |
| Mobile | `400–768px` | Stacked layout, hamburger menu, touch targets ≥44px |
| Tablet | `768–1024px` | 2-column grids, expanded padding, persistent nav |
| Desktop | `1024–1440px` | Full layout, 3-column grids where applicable |
| Large Desktop | `>1440px` | Centered content, generous margins, max-width enforced |

### Touch Targets
- Buttons: minimum `44px` height, `16px` horizontal padding
- Navigation links: `44px` tap area with adequate spacing
- Form inputs: `44px` height, clear focus states
- Code copy buttons: `32px` square, visible on hover/focus

### Collapsing Strategy
- Hero: 56px headline → 40px → 28px on mobile, maintains weight hierarchy
- Navigation: horizontal links + CTA → hamburger menu with slide-in panel
- Feature cards: 3-column → 2-column → single column stacked
- Code snippets: horizontal scroll on mobile, maintain syntax highlighting
- Testimonials: grid → carousel on mobile
- Footer: multi-column → stacked single column
- Section spacing: `96px+` → `48px` on mobile

### Image & Code Behavior
- Code blocks maintain crisp border and syntax highlighting at all sizes
- Hero illustrations scale proportionally, maintain aspect ratio
- Product screenshots use responsive images with consistent border treatment
- Full-width sections maintain edge-to-edge treatment on all viewports

---

## 8. Accessibility & States

### Focus System
- All interactive elements receive visible `2px solid #0751cf` focus outline
- Focus indicator appears on keyboard navigation only (not mouse)
- Sufficient color contrast: primary text on background exceeds WCAG AAA (`>14:1`)
- Reduced motion preference respected: disable animations, transitions

### Interactive States
- **Default**: Standard appearance with crisp borders
- **Hover**: Border brightens (`rgba(255,255,255,0.1)` → `0.2`), text shifts to white, subtle scale `1.01`
- **Active/Pressed**: Background darkens, scale `0.98`, border brightens further
- **Focus**: Blue outline ring + subtle background shift
- **Disabled**: Text muted to `#606368`, opacity `0.6`, cursor `not-allowed`

### Color Contrast
- Primary text (`#f1f2f4`) on dark background (`#121417`): ~18:1 ratio (WCAG AAA)
- Secondary text (`#606368`) on dark background: ~5.5:1 ratio (WCAG AA)
- Zed Blue (`#0751cf`) on dark background: ~4.8:1 ratio (WCAG AA for large text)
- Badge text (`#60a5fa`) on tinted background (`rgba(7,81,207,0.15)`): ~4.6:1 ratio

### Motion & Animation
- Transitions: `150ms ease-out` for hover/focus states
- Micro-interactions: subtle scale (`1.01–0.98`), no bounce or elastic easing
- Page transitions: none—immediate load for perceived performance
- Respect `prefers-reduced-motion`: disable all non-essential animations

---

## 9. Agent Prompt Guide

### Quick Color Reference
- Primary CTA: Zed Blue (`#1348DC` / `#0751cf`)
- Dark Background: Deep Base (`#121417`)
- Light Background: Pure White (`#ffffff`)
- Heading Text: Pure White (`#ffffff`)
- Body Text: Surface Light (`#f1f2f4`)
- Secondary Text: Text Secondary (`#606368`)
- Muted Text: Text Muted (`#9a9ea6`)
- Border: `1px solid rgba(255,255,255,0.1)` (dark) / `rgba(0,0,0,0.1)` (light)
- Link: Zed Blue (`#0751cf`)
- Focus Ring: `2px solid #0751cf`

### Example Component Prompts
- "Create a hero section on deep base background (`#121417`). Headline at 56px I A Writer Quattro weight 700, line-height 1.10, letter-spacing -0.5px, color `#ffffff`. Subtitle at 20px weight 500, line-height 1.40, color `#b8bdc7`. Blue CTA button (`#1348DC`, 4px radius, 10px 20px padding, white text) and outline button (transparent bg, white text, 1px solid `rgba(255,255,255,0.2)` border)."
- "Design a code card: `#1e2024` background, 1px solid `rgba(255,255,255,0.1)` border, 8px radius. Title at 20px I A Writer Quattro weight 600. Code block below in JetBrains Mono 14px, syntax-highlighted, with copy button top-right. No shadow."
- "Build a pill badge: `rgba(7, 81, 207, 0.15)` background, `#60a5fa` text, 999px radius, 4px 10px padding, 12px I A Writer Quattro weight 600, letter-spacing 0.2px."
- "Create navigation: transparent/dark header. I A Writer Quattro 14px weight 500 for links, `#b8bdc7` text. Blue CTA 'Download' right-aligned (`#1348DC` bg, white text, 4px radius). Hover: text shifts to `#f1f2f4`."
- "Design an alternating section layout: deep base (`#121417`) sections alternate with surface dark (`#1e2024`) sections. Each section has 64–96px vertical padding, max-width 1200px centered. Section heading at 40px weight 700, line-height 1.20."

### Iteration Guide
1. **Prioritize function over form**: Every visual element should serve a clear purpose—remove decoration that doesn't aid comprehension or interaction.
2. **Use cool neutrals**: Zed's grays have blue undertones (`#121417`, `#2c2e30`, `#606368`), never warm/yellow tones.
3. **Sharp corners**: 4px radius for interactive elements—precision over softness.
4. **Minimal depth**: Avoid shadows; use background contrast and border hierarchy for layering.
5. **Monospace first**: Code is the primary content—typography, spacing, and highlighting should optimize for reading code.
6. **Single accent color**: Zed Blue (`#1348DC`) is the only saturated color in core UI—use sparingly for CTAs, links, and focus states.
7. **High contrast always**: Text on background must meet WCAG AAA where possible—readability is non-negotiable.
8. **Performance-conscious**: Avoid heavy animations, complex gradients, or GPU-intensive effects—design for speed.

---

> *"Code at the speed of thought."*  
> Zed's design system exists to disappear—so developers can focus on what matters: building, collaborating, and creating. Every color, type choice, and spacing decision serves that goal. When in doubt, remove. When uncertain, simplify.