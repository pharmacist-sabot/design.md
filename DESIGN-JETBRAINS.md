# Design System Inspired by JetBrains

## 1. Visual Theme & Atmosphere

JetBrains' web presence embodies the philosophy of its IDEs: **precision, clarity, and developer-first functionality**. The design system prioritizes legibility, structured information hierarchy, and a professional yet approachable aesthetic that appeals to technical audiences. The interface uses a **cool-neutral foundation** with carefully calibrated grays that feel engineered rather than decorative—like a well-organized code editor.

The canvas is predominantly white (`#ffffff`) or light gray (`#f7f8fa`), creating a clean backdrop that lets content and code take center stage. Text is rendered in a deep near-black (`#27282e`) that provides excellent contrast without the harshness of pure black. The color system is built around a **structured palette of semantic grays** (`#dfe1e5`, `#6f737a`, `#9a9da3`) with purposeful accent colors for interactive states and brand identity.

**JetBrains Sans**, the custom corporate typeface, anchors the visual language with geometric clarity and excellent screen readability. For code and technical content, **JetBrains Mono**—the open-source monospaced font designed specifically for developers—provides distinctive character shapes, true italics for select glyphs, and optimized letterforms for extended reading sessions.

What distinguishes JetBrains' visual language is its **functional minimalism**: borders are crisp 1px lines (`#dfe1e5` in light mode), shadows are subtle and purposeful rather than decorative, and interactive elements use clear visual affordances without excessive embellishment. The system embraces **dark mode as a first-class citizen**, with a parallel token system that maintains contrast ratios and visual hierarchy in low-light environments.

**Key Characteristics:**
- JetBrains Sans (corporate) + JetBrains Mono (code) typography pairing
- Cool-neutral palette: grays with blue undertones (`#f7f8fa`, `#27282e`, `#6f737a`)
- Primary accent: JetBrains Blue (`#3369d6` / `#366acf` dark mode) as the singular interactive color
- Crisp 1px borders: `1px solid #dfe1e5` (light) / `1px solid #6f737a` (dark)
- Subtle, layered shadows for elevation without visual noise
- Dual-mode design tokens: explicit Light/Dark mode color definitions
- 4px base spacing unit with systematic scale
- Corner radius scale: 2px → 4px → 8px → 12px → 16px → 20px → 24px

---

## 2. Color Palette & Roles

### Primary Brand
- **JetBrains Blue** (`#3369d6` Light / `#366acf` Dark): Primary CTA, active links, interactive accents—the only saturated color in core UI chrome
- **JetBrains Black** (`#27282e`): Primary text, headings, body copy. Optimized for readability on white backgrounds
- **Pure White** (`#ffffff`): Page background, card surfaces, button text on blue
- **Content Background** (`#f7f8fa` Light / `#2b2d30` Dark): Section backgrounds, sidebar surfaces, subtle elevation layers

### Semantic Grays (Light Mode)
| Token | Hex | Role |
|-------|-----|------|
| `ring-text-color` | `#27282e` | Primary text, headings |
| `ring-line-color` | `#dfe1e5` | Borders, dividers, subtle separators |
| `ring-tag-background-color` | `#ebecf0` | Tag backgrounds, subtle UI surfaces |
| `ring-disabled-background-color` | `#f7f8fa` | Disabled states, inactive surfaces |
| `ring-sidebar-background-color` | `#f7f8fa` | Navigation panels, secondary surfaces |

### Semantic Grays (Dark Mode)
| Token | Hex | Role |
|-------|-----|------|
| `ring-text-color` | `#e8e9ed` | Primary text (inverted) |
| `ring-borders-color` | `#6f737a` | Borders, dividers in dark mode |
| `ring-tag-background-color` | `#43454a` | Tag backgrounds, subtle surfaces |
| `ring-disabled-background-color` | `#393b40` | Disabled states, inactive surfaces |
| `ring-content-background-color` | `#1e1f22` | Main content area background |

### Semantic Accent Colors
| Color | Light Hex | Dark Hex | Role |
|-------|-----------|----------|------|
| **Success** | `#1f7536` | `#5fad65` | Confirmation, positive states, completion |
| **Warning** | `#ffaf0f` | `#e08957` | Attention indicators, caution states |
| **Error** | `#f2b6bb` (border) / `#fad4d8` (bg) | `#e37774` | Validation errors, destructive actions |
| **Info** | `#315fbd` | `#99bbff` | Informational messages, secondary links |
| **Highlight** | `#ffaf0f` | `#312a23` | Search highlights, focused content |

### Interactive States
- **Link Color**: `#315fbd` (Light) / `#99bbff` (Dark) with underline-on-hover
- **Hover Overlay**: `rgba(51, 105, 214, 0.08)` for subtle interactive feedback
- **Active/Pressed**: Darkened blue `#2a55b3` with slight scale transform
- **Focus Ring**: `2px solid #3369d6` with `box-shadow: 0 0 0 4px rgba(51, 105, 214, 0.2)`
- **Disabled Text**: `#9a9da3` with reduced opacity

### Shadows & Depth
```css
/* Level 1: Subtle elevation (cards, dropdowns) */
box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08), 0 1px 2px rgba(0, 0, 0, 0.04);

/* Level 2: Modal elevation */
box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12), 0 4px 8px rgba(0, 0, 0, 0.08);

/* Level 3: Floating elements (tooltips, popovers) */
box-shadow: 0 16px 40px rgba(0, 0, 0, 0.16), 0 8px 16px rgba(0, 0, 0, 0.1);

/* Dark mode shadows use higher opacity for contrast */
box-shadow: 0 2px 8px rgba(0, 0, 0, 0.32), 0 1px 2px rgba(0, 0, 0, 0.24);
```

---

## 3. Typography Rules

### Font Families
- **Primary UI**: `JetBrains Sans, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif`
- **Code/Technical**: `JetBrains Mono, 'Fira Code', 'Source Code Pro', monospace`
- **Fallback Stack**: System fonts optimized for cross-platform consistency

### OpenType Features (JetBrains Mono)
- `ss01`: Symmetrical lowercase `l` for improved character distinction
- `ss02`: Curly tail on `t` for enhanced readability
- `cv01-cv10`: Contextual alternates for punctuation and symbols
- `lnum`: Lining numerals for tabular data alignment

### Type Scale & Hierarchy

| Role | Font | Size (rem) | Weight | Line Height | Letter Spacing | Use Case |
|------|------|------------|--------|-------------|----------------|----------|
| **Display Hero** | JetBrains Sans | 3.5rem (56px) | 700 | 1.1 | -0.02em | Landing page headlines |
| **Section Title** | JetBrains Sans | 2.25rem (36px) | 600 | 1.2 | -0.01em | Major section headers |
| **Card Title** | JetBrains Sans | 1.5rem (24px) | 600 | 1.3 | normal | Feature cards, module titles |
| **Body Large** | JetBrains Sans | 1.125rem (18px) | 400 | 1.5 | normal | Introductory text, descriptions |
| **Body** | JetBrains Sans | 1rem (16px) | 400 | 1.5 | normal | Standard reading text |
| **UI Text** | JetBrains Sans | 0.875rem (14px) | 500 | 1.4 | normal | Navigation, buttons, labels |
| **Caption** | JetBrains Sans | 0.75rem (12px) | 400 | 1.33 | 0.01em | Metadata, helper text |
| **Code Inline** | JetBrains Mono | 0.875rem (14px) | 400 | 1.4 | normal | Inline code snippets |
| **Code Block** | JetBrains Mono | 0.875rem (14px) | 400 | 1.5 | normal | Multi-line code examples |

### Typography Principles
- **Developer-first legibility**: JetBrains Mono's character shapes prioritize distinction between similar glyphs (`0`/`O`, `1`/`l`/`I`)
- **Weight hierarchy**: 400 (body), 500 (UI), 600 (titles), 700 (headlines)—avoiding excessive weight variation
- **Line height scaling**: Tighter leading for headlines (1.1–1.2), relaxed for body (1.5) to improve scanability
- **Code integration**: Monospaced fonts used contextually for technical content, never for UI chrome
- **Responsive scaling**: Type sizes use `clamp()` functions for fluid scaling between breakpoints

---

## 4. Component Stylings

### Buttons

**Primary Button**
```css
background: #3369d6;
color: #ffffff;
padding: 10px 20px;
border-radius: 4px; /* Corner Radius/S */
border: 1px solid transparent;
font: 500 14px/1.4 JetBrains Sans;
transition: background 0.2s ease, transform 0.1s ease;

&:hover {
  background: #2a55b3;
}
&:active {
  transform: scale(0.98);
  background: #224799;
}
&:focus-visible {
  outline: 2px solid #3369d6;
  outline-offset: 2px;
  box-shadow: 0 0 0 4px rgba(51, 105, 214, 0.2);
}
&:disabled {
  background: #ebecf0;
  color: #9a9da3;
  cursor: not-allowed;
}
```

**Secondary Button**
```css
background: transparent;
color: #27282e;
padding: 10px 20px;
border-radius: 4px;
border: 1px solid #dfe1e5;
font: 500 14px/1.4 JetBrains Sans;

&:hover {
  background: rgba(51, 105, 214, 0.08);
  border-color: #3369d6;
}
```

**Ghost / Text Button**
```css
background: transparent;
color: #315fbd;
padding: 8px 12px;
border-radius: 4px;
text-decoration: none;

&:hover {
  text-decoration: underline;
  background: rgba(51, 105, 214, 0.08);
}
```

**Icon Button**
```css
width: 32px;
height: 32px;
padding: 0;
border-radius: 4px;
display: inline-flex;
align-items: center;
justify-content: center;
background: transparent;
border: 1px solid transparent;

&:hover {
  background: rgba(39, 40, 46, 0.08);
}
```

### Cards & Containers
```css
background: #ffffff;
border: 1px solid #dfe1e5;
border-radius: 8px; /* Corner Radius/M */
padding: 24px;
box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08), 0 1px 2px rgba(0, 0, 0, 0.04);

/* Hover state for interactive cards */
&:hover {
  border-color: #3369d6;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12), 0 2px 4px rgba(0, 0, 0, 0.08);
}

/* Dark mode variant */
@media (prefers-color-scheme: dark) {
  background: #1e1f22;
  border-color: #6f737a;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.32), 0 1px 2px rgba(0, 0, 0, 0.24);
}
```

### Inputs & Forms
```css
background: #ffffff;
color: #27282e;
border: 1px solid #dfe1e5;
border-radius: 4px;
padding: 8px 12px;
font: 400 14px/1.4 JetBrains Sans;
transition: border-color 0.2s ease, box-shadow 0.2s ease;

&::placeholder {
  color: #9a9da3;
}

&:focus {
  border-color: #3369d6;
  outline: none;
  box-shadow: 0 0 0 3px rgba(51, 105, 214, 0.15);
}

&:disabled {
  background: #f7f8fa;
  color: #9a9da3;
  cursor: not-allowed;
}
```

### Navigation
- **Horizontal Nav**: Clean white background, non-sticky by default
- **Logo**: JetBrains wordmark + icon (32px height), left-aligned
- **Links**: JetBrains Sans 14px weight 500, `#27282e` text, hover → `#3369d6`
- **Active State**: Bottom border `2px solid #3369d6` or subtle background tint
- **Mobile**: Collapsible hamburger menu with slide-in panel
- **Dropdowns**: White background, 8px radius, Level 2 shadow, keyboard-navigable

### Code Blocks & Technical Content
```css
background: #f7f8fa; /* or #2b2d30 in dark mode */
border: 1px solid #dfe1e5;
border-radius: 8px;
padding: 16px;
font-family: JetBrains Mono, monospace;
font-size: 14px;
line-height: 1.5;
overflow-x: auto;

/* Syntax highlighting tokens use semantic colors */
.keyword { color: #af1df5; }
.string { color: #21d789; }
.comment { color: #9a9da3; font-style: italic; }
.function { color: #087cfa; }
```

### Distinctive Components

**Product Cards**
- 8px radius, whisper border (`1px solid #dfe1e5`)
- Product icon (48px) + title (24px/600) + description (16px/400)
- Hover: border color shift to JetBrains Blue, subtle shadow elevation
- "Free for non-commercial use" badge: `#ebecf0` background, 12px/600 text

**Trust Badges / Logos Grid**
- Company logos displayed in grayscale by default, color on hover
- Grid layout with consistent spacing (24px gap)
- Optional metric display: large number (36px/700) + descriptor (14px/400)

**Feature Toggles / Tags**
```css
background: #ebecf0;
color: #27282e;
padding: 4px 8px;
border-radius: 999px; /* Pill shape */
font: 600 12px/1.33 JetBrains Sans;
letter-spacing: 0.01em;

/* Semantic variants */
&.success { background: #e6f7e9; color: #1f7536; }
&.warning { background: #fff4e6; color: #dd5b00; }
&.error { background: #fad4d8; color: #b32635; }
```

---

## 5. Layout Principles

### Spacing System
- **Base unit**: 4px
- **Scale**: 4, 8, 12, 16, 20, 24, 32, 40, 48, 64, 80, 96, 128px
- **Micro-adjustments**: 2px, 6px, 10px for fine-tuning component internals
- **CSS Custom Properties**: `--ring-space-2: 2px; --ring-space-4: 4px;` etc.

### Grid & Container
- **Max content width**: 1200px centered with responsive padding
- **Breakpoint containers**: 
  - Mobile: 16px horizontal padding
  - Tablet: 24px padding
  - Desktop: 32px padding + centered max-width
- **Grid system**: CSS Grid with `minmax()` for flexible card layouts
- **Feature sections**: 2–3 column grids on desktop, stacked on mobile

### Whitespace Philosophy
- **Functional breathing room**: 32–64px vertical spacing between major sections
- **Content grouping**: Related elements use 8–16px internal spacing; unrelated use 24px+
- **Asymmetric balance**: Slightly more bottom margin than top on headings for optical alignment
- **Code-friendly density**: Technical content blocks use tighter line-height (1.4–1.5) but generous surrounding margins

### Border Radius Scale
| Token | Value | Use Case |
|-------|-------|----------|
| `Corner Radius/XS` | 2px | Checkbox borders, tiny UI details |
| `Corner Radius/S` | 4px | Buttons, inputs, small interactive elements |
| `Corner Radius/M` | 8px | Standard cards, containers, dropdowns |
| `Corner Radius/L` | 12px | Featured cards, modal containers |
| `Corner Radius/XL` | 16px | Hero sections, large promotional blocks |
| `Corner Radius/2XL` | 20px | Full-bleed sections with rounded corners |
| `Corner Radius/3XL` | 24px | Decorative background shapes |
| `Pill` | 999px | Tags, badges, toggle switches |

---

## 6. Depth & Elevation

| Level | Shadow Definition | Use Case |
|-------|------------------|----------|
| **Flat (0)** | `none` | Page background, text blocks, static content |
| **Subtle (1)** | `0 1px 2px rgba(0,0,0,0.04), 0 2px 8px rgba(0,0,0,0.08)` | Cards, dropdowns, hover states |
| **Elevated (2)** | `0 4px 8px rgba(0,0,0,0.08), 0 8px 24px rgba(0,0,0,0.12)` | Modals, popovers, active menus |
| **Floating (3)** | `0 8px 16px rgba(0,0,0,0.1), 0 16px 40px rgba(0,0,0,0.16)` | Tooltips, drag-and-drop previews |
| **Focus** | `0 0 0 3px rgba(51, 105, 214, 0.15)` + `2px solid #3369d6` outline | Keyboard focus indicator |

**Shadow Philosophy**: JetBrains uses **purpose-driven elevation**—shadows exist to communicate interactivity and hierarchy, not decoration. Each shadow layer uses low-opacity blacks (`0.04`–`0.16`) to create soft, natural depth that doesn't compete with content. Dark mode shadows increase opacity (`0.24`–`0.32`) to maintain contrast against darker backgrounds.

### Decorative Elements
- **Product illustrations**: Clean, isometric-style graphics with subtle gradients
- **Code screenshot treatment**: 1px border + 8px radius + Level 1 shadow
- **Section alternation**: White ↔ `#f7f8fa` backgrounds for visual rhythm without harsh dividers
- **Gradient accents**: Subtle linear gradients (`#3369d6` → `#087cfa`) for hero CTAs only

---

## 7. Responsive Behavior

### Breakpoints
| Name | Width | Key Adaptations |
|------|-------|----------------|
| **Mobile** | `< 640px` | Single-column layout, condensed navigation, 16px base padding |
| **Tablet** | `640–1024px` | 2-column grids where appropriate, expanded touch targets |
| **Desktop** | `1024–1440px` | Full multi-column layouts, hover states enabled |
| **Large Desktop** | `> 1440px` | Centered content with max-width, generous margins |

### Touch Targets & Accessibility
- **Minimum tap target**: 44×44px for all interactive elements
- **Button padding**: Minimum 10px vertical, 20px horizontal
- **Link spacing**: 8px minimum between adjacent links
- **Focus indicators**: Visible `2px` outline + subtle shadow for keyboard navigation
- **Reduced motion**: Respect `prefers-reduced-motion` for transforms and transitions

### Collapsing Strategy
- **Hero section**: Display text scales from 56px → 36px → 28px; letter-spacing relaxes proportionally
- **Navigation**: Horizontal links → hamburger menu with slide-in panel
- **Feature grids**: 3-column → 2-column → stacked layout
- **Code examples**: Horizontal scroll enabled on small screens; syntax highlighting preserved
- **Product cards**: Image top → image left → stacked layout based on viewport
- **Footer**: Multi-column → 2-column → single-column stacked

### Image & Media Behavior
- **Product screenshots**: Maintain aspect ratio with `object-fit: contain`; whisper border at all sizes
- **Illustrations**: SVG-based for crisp rendering; scale proportionally with container
- **Lazy loading**: Intersection Observer for offscreen images; blurred placeholder while loading
- **Dark mode images**: Use `prefers-color-scheme` to serve optimized assets

---

## 8. Accessibility & States

### Focus Management
- **Visible focus**: `2px solid #3369d6` outline + `box-shadow` reinforcement on all interactive elements
- **Skip links**: Hidden until focused; appears at top of page for keyboard users
- **Logical tab order**: DOM order matches visual layout; modal traps focus appropriately
- **ARIA labels**: Descriptive labels for icon-only buttons; `aria-expanded` for collapsible sections

### Interactive States
| State | Visual Treatment | Example |
|-------|-----------------|---------|
| **Default** | Standard styling per component spec | Resting button |
| **Hover** | Background tint (`rgba(51,105,214,0.08)`) + border color shift | Mouse over link |
| **Active** | Scale `0.98` + darker background | Button press |
| **Focus** | Outline + shadow ring (see above) | Keyboard navigation |
| **Disabled** | `#9a9da3` text + `#f7f8fa` background + `cursor: not-allowed` | Unavailable action |
| **Loading** | Spinner overlay + reduced opacity + `aria-busy="true"` | Async operation |

### Color Contrast Compliance
- **Primary text** (`#27282e` on `#ffffff`): **15.8:1** (WCAG AAA)
- **Secondary text** (`#6f737a` on `#ffffff`): **4.6:1** (WCAG AA for large text)
- **JetBrains Blue** (`#3369d6` on `#ffffff`): **4.5:1** (WCAG AA for normal text)
- **Success text** (`#1f7536` on `#ffffff`): **6.2:1** (WCAG AAA)
- **Dark mode text** (`#e8e9ed` on `#1e1f22`): **14.1:1** (WCAG AAA)

### Motion & Reduced Preferences
```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}
```

---

## 9. Agent Prompt Guide

### Quick Color Reference
```css
/* Primary */
--ring-main-color: #3369d6;        /* Light mode primary */
--ring-main-color-dark: #366acf;   /* Dark mode primary */
--ring-text-color: #27282e;        /* Primary text */
--ring-content-bg: #ffffff;        /* Content background */
--ring-line-color: #dfe1e5;        /* Borders/dividers */

/* Semantic */
--ring-success-color: #1f7536;
--ring-warning-color: #ffaf0f;
--ring-error-color: #b32635;

/* Dark mode overrides */
@media (prefers-color-scheme: dark) {
  --ring-text-color: #e8e9ed;
  --ring-content-bg: #1e1f22;
  --ring-line-color: #6f737a;
}
```

### Example Component Prompts
- **Hero Section**: "Create a hero on white background. Headline: JetBrains Sans 56px/700, line-height 1.1, color `#27282e`. Subheading: 18px/400, `#6f737a`. Primary CTA: `#3369d6` background, white text, 4px radius, 10px 20px padding. Secondary CTA: transparent bg, `#3369d6` border, same padding."

- **Product Card**: "Design a card: white bg, `1px solid #dfe1e5` border, 8px radius. Shadow: `0 2px 8px rgba(0,0,0,0.08), 0 1px 2px rgba(0,0,0,0.04)`. Title: 24px/600 JetBrains Sans. Description: 16px/400 `#6f737a`. Badge: `#ebecf0` bg, 12px/600, pill shape."

- **Code Block**: "Create a code snippet container: `#f7f8fa` bg, `1px solid #dfe1e5` border, 8px radius, 16px padding. Font: JetBrains Mono 14px/1.5. Syntax tokens: keyword `#af1df5`, string `#21d789`, comment `#9a9da3 italic`."

- **Navigation Bar**: "Build a header: white bg, 1px bottom border `#dfe1e5`. Logo left (32px height). Links: JetBrains Sans 14px/500, `#27282e`, hover → `#3369d6`. Primary CTA right: blue pill button. Mobile: hamburger toggle at 640px breakpoint."

- **Dark Mode Toggle**: "Implement theme switch: respect `prefers-color-scheme` default. Provide manual toggle with sun/moon icons. Store preference in `localStorage`. Transition colors with `0.2s ease` on `background-color`, `color`, `border-color`."

### Iteration Guide
1. **Always use the dual-mode token system**: Every color has explicit Light/Dark definitions—never hardcode hex values without checking dark mode equivalents
2. **Corner radii follow the 2→4→8→12→16→20→24 scale**: Use semantic tokens (`Corner Radius/M`) not hardcoded pixels
3. **JetBrains Blue is the only saturated accent**: Reserve `#3369d6` for primary actions and active states; use semantic colors (success/warning/error) for system feedback only
4. **Typography pairs Sans + Mono intentionally**: Use JetBrains Sans for UI chrome, JetBrains Mono exclusively for code/technical content
5. **Shadows are functional, not decorative**: Use elevation levels purposefully—Level 1 for cards, Level 2 for modals, never for static content
6. **Spacing is systematic**: Use the 4px base unit scale; avoid arbitrary values like 13px or 19px
7. **Accessibility is non-negotiable**: All interactive elements must have visible focus states; contrast ratios must meet WCAG AA minimum
8. **Code examples are first-class content**: Treat code blocks with the same design care as marketing copy—syntax highlighting, readable fonts, proper scrolling

---

> **Design Philosophy Summary**: JetBrains' system prioritizes *clarity over cleverness*, *function over flourish*. Every visual decision serves developer productivity: legible typography, predictable interactions, and a calm canvas that lets code—and the work it enables—take center stage. The system is engineered, not embellished; precise, not precious.