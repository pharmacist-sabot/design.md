# Design System Inspired by pnpm

## 1. Visual Theme & Atmosphere

pnpm.io embodies a philosophy of **clarity, efficiency, and developer-first pragmatism**. The design system prioritizes legibility, technical precision, and a warm-but-professional aesthetic that reflects pnpm's core values: speed, disk-space efficiency, and monorepo excellence.

The page canvas uses a clean, high-contrast foundation: pure white (`#ffffff`) for light mode and deep charcoal (`#1b1b1d`) for dark mode. Text isn't pure black—it's a softened near-black (`#1c1e21`) in light mode and a crisp off-white (`#f5f6f7`) in dark mode—reducing eye strain during long documentation sessions.

The color palette is anchored by **pnpm Orange** (`#f69220`), a vibrant, energetic accent that signals primary actions, links, and interactive elements. Unlike saturated reds or blues, this orange conveys warmth, approachability, and forward momentum—perfect for a tool that makes package management feel faster and friendlier.

**Key Characteristics:**
- **System font stack**: `-apple-system, Segoe UI, Roboto, Ubuntu, Cantarell, Noto Sans, sans-serif` for maximum cross-platform legibility and performance
- **pnpm Orange** (`#f69220`) as the singular saturated accent for CTAs, links, and highlights
- **High-contrast neutrals**: Near-black text on white, off-white on dark, ensuring AAA-level readability
- **Subtle depth**: Minimal shadows (`0 0 0 1px hsla(0, 0%, 30%, .05)`) and soft borders for structure without visual noise
- **Responsive pragmatism**: Mobile-first layout with adaptive navigation, collapsible sections, and touch-friendly targets
- **Dark mode first-class**: Full theme parity with inverted color tokens and adjusted contrast ratios

---

## 2. Color Palette & Roles

### Primary Brand
- **pnpm Orange** (`#f69220`): Primary CTA, link color, interactive accent, focus states
- **Orange Light** (`#f79f3b`): Hover state for primary buttons and links
- **Orange Lighter** (`#f8a648`): Active/pressed state
- **Orange Lightest** (`#f9b970`): Subtle highlights, decorative elements
- **Orange Dark** (`#f0850a`): Disabled/low-emphasis interactive states
- **Orange Darkest** (`#bb6708`): Text on orange backgrounds, deep accents

### Core Neutrals (Light Mode)
- **Canvas White** (`#ffffff`): Page background, card surfaces, input backgrounds
- **Text Primary** (`#1c1e21`): Headings, body copy, primary UI text
- **Text Secondary** (`#444950`): Descriptions, metadata, secondary labels
- **Text Muted** (`#6e7681`): Placeholder text, disabled states, captions
- **Border Subtle** (`#d0d7de`): Dividers, input borders, card outlines
- **Border Focus** (`#0969da`): Focus ring for accessibility (blue for contrast against orange)

### Core Neutrals (Dark Mode)
- **Canvas Dark** (`#1b1b1d`): Page background
- **Surface Dark** (`#24292e`): Card backgrounds, input fields
- **Text Primary Dark** (`#f5f6f7`): Headings, body copy
- **Text Secondary Dark** (`#9198a1`): Descriptions, metadata
- **Text Muted Dark** (`#6e7681`): Placeholder, disabled states
- **Border Subtle Dark** (`#373e47`): Dividers, input borders

### Semantic & Utility Colors
- **Success Green** (`#2ea44f`): Completed actions, positive indicators, build success
- **Warning Yellow** (`#d29922`): Caution states, pending actions, deprecation notices
- **Error Red** (`#f85149`): Errors, destructive actions, validation failures
- **Info Blue** (`#58a6ff`): Informational badges, external links, documentation hints
- **Code Background** (`#f6f8fa` light / `#161b22` dark): Inline code blocks, terminal snippets

### Interactive States
- **Link Default**: `#f69220` with underline on hover
- **Link Hover**: `#f79f3b` with intensified underline
- **Focus Ring**: `2px solid #0969da` + `0 0 0 3px rgba(9, 105, 218, 0.3)` for high visibility
- **Button Hover**: Background `#f79f3b`, subtle scale transform `scale(1.02)`
- **Button Active**: Background `#f8a648`, `scale(0.98)` for tactile feedback
- **Disabled**: Text `#6e7681`, background `#f6f8fa`, pointer-events: none

### Shadows & Depth
- **Level 0 (Flat)**: No shadow, no border — text blocks, section backgrounds
- **Level 1 (Subtle)**: `0 1px 2px rgba(0,0,0,0.05)` — cards, inputs, dropdowns
- **Level 2 (Elevated)**: `0 4px 12px rgba(0,0,0,0.08)` — modals, sticky headers, floating buttons
- **Focus Glow**: `0 0 0 3px rgba(9, 105, 218, 0.3)` — keyboard navigation indicator
- **Dark Mode Adjustments**: Shadows use `rgba(0,0,0,0.3)` base with reduced blur for depth perception

---

## 3. Typography Rules

### Font Family
- **Primary Stack**: `-apple-system, Segoe UI, Roboto, Ubuntu, Cantarell, Noto Sans, sans-serif, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol"`
- **Monospace**: `SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace` for code blocks, CLI snippets
- **No custom web fonts**: Prioritizes system fonts for instant rendering and performance

### Hierarchy

| Role | Size | Weight | Line Height | Letter Spacing | Use Case |
|------|------|--------|-------------|----------------|----------|
| Hero Title | 3.5rem (56px) | 700 | 1.1 | -0.02em | Homepage headline, major section headers |
| Section Heading | 2rem (32px) | 700 | 1.2 | -0.01em | Documentation section titles, feature headers |
| Subheading | 1.5rem (24px) | 600 | 1.3 | normal | Card titles, subsection headers |
| Body Large | 1.125rem (18px) | 400 | 1.6 | normal | Intro paragraphs, feature descriptions |
| Body | 1rem (16px) | 400 | 1.5 | normal | Standard documentation text, UI labels |
| Body Small | 0.875rem (14px) | 400 | 1.4 | normal | Metadata, captions, helper text |
| Code Inline | 0.9em | 400 | 1.4 | normal | Inline code snippets, command references |
| Code Block | 0.9rem (14.4px) | 400 | 1.5 | normal | Multi-line CLI examples, config snippets |
| Button / Nav | 1rem (16px) | 500 | 1.2 | normal | Navigation links, CTA buttons, tabs |
| Badge / Tag | 0.75rem (12px) | 600 | 1.3 | 0.02em | Status indicators, version tags, feature labels |

### Principles
- **Legibility first**: System fonts ensure instant rendering; no font-loading delays
- **Consistent rhythm**: Line heights scale proportionally (1.1 → 1.6) as text size decreases
- **Weight hierarchy**: 400 (reading), 500 (UI), 600 (emphasis), 700 (headings) for clear visual scanning
- **Code clarity**: Monospace fonts with generous line-height for multi-line terminal output
- **Responsive scaling**: Font sizes use `clamp()` or fluid typography for seamless mobile-to-desktop adaptation

---

## 4. Component Stylings

### Buttons

**Primary (Orange)**
```css
background: #f69220;
color: #ffffff;
padding: 0.75rem 1.5rem;
border-radius: 6px;
border: 1px solid transparent;
font-weight: 500;
transition: background 0.2s, transform 0.1s;
```
- Hover: `background: #f79f3b; transform: scale(1.02);`
- Active: `background: #f8a648; transform: scale(0.98);`
- Focus: `outline: 2px solid #0969da; box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.3);`
- Use: "Install pnpm", "Get Started", primary CTAs

**Secondary (Outline)**
```css
background: transparent;
color: #1c1e21;
border: 1px solid #d0d7de;
padding: 0.75rem 1.5rem;
border-radius: 6px;
font-weight: 500;
```
- Hover: `border-color: #f69220; color: #f69220;`
- Dark mode: `color: #f5f6f7; border-color: #373e47;`
- Use: Secondary actions, cancel buttons, ghost actions

**Text / Link Button**
```css
background: transparent;
color: #f69220;
padding: 0.5rem 0;
text-decoration: none;
border-bottom: 1px solid transparent;
```
- Hover: `border-bottom-color: #f69220;`
- Use: Inline links, navigation items, tertiary actions

**Badge / Pill**
```css
background: #f6f8fa; /* or #161b22 in dark mode */
color: #1c1e21;
padding: 0.25rem 0.75rem;
border-radius: 999px;
font-size: 0.75rem;
font-weight: 600;
```
- Variant (orange): `background: #fff5e6; color: #f69220;`
- Use: Version tags, status indicators, feature labels

### Cards & Containers
```css
background: #ffffff; /* or #24292e in dark mode */
border: 1px solid #d0d7de; /* or #373e47 */
border-radius: 8px;
padding: 1.5rem;
box-shadow: 0 1px 2px rgba(0,0,0,0.05);
transition: box-shadow 0.2s, border-color 0.2s;
```
- Hover: `box-shadow: 0 4px 12px rgba(0,0,0,0.08); border-color: #f69220;`
- Featured card: Add `border-left: 4px solid #f69220;` for emphasis
- Use: Feature blocks, testimonial cards, documentation previews

### Inputs & Forms
```css
background: #ffffff; /* or #161b22 */
color: #1c1e21; /* or #f5f6f7 */
border: 1px solid #d0d7de; /* or #373e47 */
border-radius: 6px;
padding: 0.625rem 0.875rem;
font-size: 1rem;
```
- Focus: `border-color: #0969da; box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.3);`
- Placeholder: `color: #6e7681;`
- Error state: `border-color: #f85149;`
- Use: Search bars, configuration inputs, newsletter signups

### Navigation
- **Desktop**: Horizontal nav with logo left, links center, CTA right
- **Mobile**: Hamburger menu with slide-in panel, sticky header
- **Links**: `font-weight: 500`, `color: #1c1e21`, hover `color: #f69220`
- **Active state**: Bottom border `2px solid #f69220` or background tint
- **Dropdowns**: White background, subtle shadow, `border-radius: 8px`, keyboard navigable

### Code Blocks & Terminal Snippets
```css
background: #f6f8fa; /* light */ or #161b22 /* dark */
border: 1px solid #d0d7de; /* or #373e47 */
border-radius: 8px;
padding: 1rem 1.25rem;
font-family: SFMono-Regular, Consolas, monospace;
font-size: 0.9rem;
line-height: 1.5;
overflow-x: auto;
```
- Syntax highlighting: Prism.js with GitHub (light) / Dracula (dark) themes
- Copy button: Top-right absolute position, orange icon on hover
- Line numbers: Optional, muted gray, non-selectable

### Distinctive Components

**Feature Grid Cards**
- 3-column grid on desktop → 2-column tablet → 1-column mobile
- Icon (SVG, 48x48px) + title (24px) + description (16px)
- Hover: Subtle lift (`transform: translateY(-2px)`) + shadow intensification
- Background alternation: White / `#f6f8fa` for visual rhythm

**Sponsor / Logo Grid**
- Grayscale logos by default, color on hover
- Uniform sizing (`height: 40px`), centered alignment
- Tier labels: "Platinum", "Gold", "Silver" with orange accent badges
- Responsive: Horizontal scroll on mobile, grid on desktop

**Testimonial Blocks**
- Quote icon (orange, decorative) + testimonial text (18px italic) + author attribution
- Avatar (48px circle) + name + title/company
- Background: Subtle tint `#fff5e6` (light) or `#24292e` (dark)

**Version / Language Dropdowns**
- Pill-shaped trigger with chevron icon
- Dropdown: White background, shadow, `border-radius: 8px`, max-height with scroll
- Active item: Orange background, white text

---

## 5. Layout Principles

### Spacing System
- **Base unit**: 4px (for fine-grained control)
- **Scale**: `4, 8, 12, 16, 24, 32, 48, 64, 96, 128px`
- **Section padding**: `64px` vertical on desktop, `32px` on mobile
- **Card padding**: `24px` internal, `16px` between grid items
- **Inline spacing**: `4px` (micro), `8px` (small), `16px` (medium), `24px` (large)

### Grid & Container
- **Max content width**: `1200px` centered with `padding: 0 24px`
- **Hero section**: Full-width background, centered content column (`max-width: 800px`)
- **Feature sections**: 3-column grid (`gap: 24px`), auto-fit with `minmax(280px, 1fr)`
- **Documentation layout**: Two-column (sidebar + content) on desktop, stacked on mobile
- **Full-bleed sections**: Warm orange tint (`#fff5e6`) or dark charcoal (`#1b1b1d`) for visual separation

### Whitespace Philosophy
- **Generous vertical rhythm**: 64–96px between major sections to reduce cognitive load
- **Content islands**: Text blocks surrounded by ample margin, creating focused reading zones
- **Progressive disclosure**: Collapsible sections, expandable code snippets, lazy-loaded content
- **Breathing room**: No element touches container edges; minimum `24px` padding internally

### Border Radius Scale
- **Micro (4px)**: Inputs, small buttons, tags
- **Standard (6px)**: Buttons, cards, dropdowns
- **Comfortable (8px)**: Feature cards, modals, code blocks
- **Large (12px)**: Hero containers, promotional banners
- **Full pill (999px)**: Badges, status indicators, toggle switches

---

## 6. Depth & Elevation

| Level | Treatment | Use Case |
|-------|-----------|----------|
| **Flat (0)** | No shadow, no border | Page background, text blocks, section dividers |
| **Subtle (1)** | `0 1px 2px rgba(0,0,0,0.05)` + `1px solid #d0d7de` | Cards, inputs, dropdown menus |
| **Elevated (2)** | `0 4px 12px rgba(0,0,0,0.08)` + `1px solid #d0d7de` | Modals, sticky headers, floating action buttons |
| **Focus (A11y)** | `0 0 0 3px rgba(9, 105, 218, 0.3)` + `2px solid #0969da` | Keyboard navigation on interactive elements |
| **Dark Mode Adjust** | Shadows use `rgba(0,0,0,0.3)` base; borders `#373e47` | Maintain depth perception in low-light themes |

**Shadow Philosophy**: pnpm.io uses minimal, purposeful elevation. Shadows are subtle and functional—never decorative. The multi-layer approach (blur + spread + color) creates natural depth without visual clutter. In dark mode, shadows are slightly stronger to compensate for reduced contrast.

### Decorative Elements
- **Abstract illustrations**: Geometric shapes, subtle gradients, or terminal-style animations in hero sections
- **Code-inspired patterns**: Faint grid backgrounds, monospace decorative text, or ASCII-art accents
- **Orange accent lines**: Thin `2px solid #f69220` dividers for section separation or emphasis
- **No heavy imagery**: Prioritizes typography, code snippets, and vector graphics for performance

---

## 7. Responsive Behavior

### Breakpoints
| Name | Width | Key Adaptations |
|------|-------|----------------|
| **Mobile** | `< 640px` | Single-column layout, collapsed nav, touch-optimized targets |
| **Tablet** | `640–1024px` | 2-column grids, condensed sidebar, adaptive typography |
| **Desktop** | `1024–1440px` | Full 3-column grids, persistent sidebar, expanded content width |
| **Large Desktop** | `> 1440px` | Centered layout with generous margins, max-width constraints |

### Touch Targets
- **Buttons**: Minimum `44x44px` tap area (padding ensures compliance)
- **Links**: Adequate spacing (`8px` vertical) to prevent mis-taps
- **Navigation**: Hamburger menu with large toggle (`48x48px`), slide-in panel with scroll
- **Code copy buttons**: Minimum `32x32px` with clear iconography

### Collapsing Strategy
- **Hero headline**: `clamp(2rem, 5vw, 3.5rem)` for fluid scaling
- **Navigation**: Horizontal links → hamburger menu with accessible ARIA labels
- **Feature grids**: `grid-template-columns: repeat(auto-fit, minmax(280px, 1fr))`
- **Sidebar**: Persistent on desktop, off-canvas drawer on mobile
- **Code blocks**: Horizontal scroll with visible scrollbar, copy button always accessible
- **Footer**: Multi-column → stacked single column with grouped links

### Image & Media Behavior
- **Logos / Icons**: SVG format with `width: 100%; height: auto;` for crisp scaling
- **Screenshots**: Max-width constrained, `border-radius: 8px`, subtle shadow
- **Illustrations**: Decorative elements scale proportionally; hidden on smallest viewports if non-essential
- **Dark mode**: SVGs with `currentColor` or dual-theme assets for seamless inversion

---

## 8. Accessibility & States

### Focus System
- **Visible focus**: `2px solid #0969da` outline + `0 0 0 3px rgba(9, 105, 218, 0.3)` glow
- **Keyboard navigation**: Logical tab order, skip-to-content link, ARIA landmarks
- **Reduced motion**: Respect `prefers-reduced-motion` for animations/transitions
- **Screen reader support**: Semantic HTML, ARIA labels for icons, descriptive link text

### Interactive States
| State | Visual Treatment |
|-------|-----------------|
| **Default** | Standard appearance per component spec |
| **Hover** | Color shift (`#f69220` → `#f79f3b`), subtle scale (`1.02`), underline for links |
| **Active** | Darker orange (`#f8a648`), scale down (`0.98`), shadow reduction |
| **Focus** | Blue focus ring + glow for high visibility against orange accents |
| **Disabled** | `opacity: 0.6; pointer-events: none;` text color `#6e7681` |
| **Loading** | Spinner icon + `opacity: 0.7;` on button, disabled state |

### Color Contrast
- **Primary text** (`#1c1e21` on `#ffffff`): ~16:1 ratio (WCAG AAA)
- **Secondary text** (`#444950` on `#ffffff`): ~7:1 ratio (WCAG AA)
- **Orange CTA** (`#f69220` on `#ffffff`): ~3.5:1 (WCAG AA for large text ≥18px)
- **White text on orange** (`#ffffff` on `#f69220`): ~4.8:1 (WCAG AA)
- **Dark mode**: All ratios re-validated; focus ring uses blue for contrast against orange

### Motion & Animation
- **Transitions**: `0.2s ease` for color, `0.1s ease` for transform (buttons)
- **Entrance animations**: Subtle fade-in + slide-up (`opacity 0.3s, transform 0.3s`)
- **Hover effects**: Scale + shadow, never position shifts that cause layout reflow
- **Respect preferences**: Disable animations if `prefers-reduced-motion: reduce`

---

## 9. Agent Prompt Guide

### Quick Color Reference
```css
--ifm-color-primary: #f69220;        /* Primary orange */
--ifm-color-primary-light: #f79f3b;  /* Hover */
--ifm-color-primary-dark: #f0850a;   /* Disabled */
--ifm-background-color: #ffffff;     /* Light mode canvas */
--ifm-background-surface-color: #f6f8fa; /* Card background */
--ifm-font-color-base: #1c1e21;      /* Primary text */
--ifm-font-color-secondary: #444950; /* Secondary text */
--ifm-color-emphasis-300: #d0d7de;   /* Subtle border */
--ifm-color-emphasis-200: #0969da;   /* Focus ring */
```

### Example Component Prompts
- **Hero Section**: "Create a hero section on white background. Headline at 56px weight 700, line-height 1.1, color #1c1e21. Subtitle at 20px weight 400, color #444950. Primary CTA button (#f69220 bg, white text, 6px radius, 0.75rem 1.5rem padding) and secondary outline button (transparent bg, #1c1e21 text, #d0d7de border). Add subtle bottom border #d0d7de to separate from next section."

- **Documentation Card**: "Design a card: white background, 1px solid #d0d7de border, 8px radius. Use shadow: 0 1px 2px rgba(0,0,0,0.05). Title at 24px weight 600, color #1c1e21. Body at 16px weight 400, color #444950. Add orange left border (4px solid #f69220) for featured items."

- **Code Block**: "Build a code snippet container: background #f6f8fa, 1px solid #d0d7de border, 8px radius, 1rem 1.25rem padding. Font: SFMono-Regular, Consolas, monospace, 0.9rem size, 1.5 line-height. Include copy button (top-right, orange icon on hover, 32x32px tap target)."

- **Navigation Bar**: "Create responsive nav: white header, logo left (48px height), links center (16px weight 500, #1c1e21 text), orange CTA right. Hover: link color #f69220. Mobile: hamburger menu (48x48px) with slide-in panel, ARIA labels, focus management."

- **Dark Mode Toggle**: "Design theme switcher: pill container (24x44px), toggle circle (20x20px), orange accent when active. Transition: 0.2s ease. Respect prefers-color-scheme, persist user choice in localStorage."

### Iteration Guide
1. **Always use system fonts** — no custom web fonts; prioritize performance and instant rendering
2. **pnpm Orange is the only saturated color** in core UI — use it sparingly for CTAs, links, and highlights
3. **Contrast is non-negotiable** — verify all text/background pairs meet WCAG AA minimum (AAA preferred)
4. **Shadows are functional, not decorative** — use minimal elevation for hierarchy, never for visual flair
5. **Dark mode is first-class** — design both themes simultaneously; test inversion of all components
6. **Mobile-first spacing** — start with 16px base padding, scale up for larger viewports
7. **Code blocks are content, not decoration** — ensure syntax highlighting is legible, copy buttons are accessible
8. **Focus states must be visible** — blue focus ring (#0969da) ensures keyboard navigation works against orange accents

---

> **Design Philosophy Summary**: pnpm.io's design system reflects the tool itself—efficient, pragmatic, and developer-centric. Every visual decision serves clarity, performance, and accessibility. The warm orange accent conveys energy without sacrificing professionalism, while the strict typographic hierarchy and minimal depth system ensure content remains the hero. This is a design system built for people who value speed, precision, and getting work done.