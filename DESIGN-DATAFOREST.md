# Design System Inspired by dataforest Cloud

## 1. Visual Theme & Atmosphere

dataforest Cloud's design philosophy embodies **German engineering precision**: clean, transparent, and purpose-driven. The interface prioritizes clarity over decoration, creating a professional atmosphere that instills trust in enterprise infrastructure. The visual language is rooted in **cool, technical neutrals** with strategic accents that guide attention to actionable elements without overwhelming the user.

The canvas is predominantly pure white (`#ffffff`) for maximum legibility of technical data, paired with a **deep charcoal text color** (`#1a1a1a` or `rgba(26, 26, 26, 0.98)`) that provides strong contrast without the harshness of pure black. The neutral scale leans toward **blue-gray undertones** (`#f8f9fa`, `#6c757d`, `#343a40`), evoking the reliability of enterprise hardware and data center infrastructure.

The typography backbone is a **modern geometric sans-serif** (likely Inter, Roboto, or system-ui stack), optimized for readability of dense technical information. Headlines use tight tracking and bold weights for scannability, while body text maintains generous line-height for comfortable reading of specifications and documentation.

What defines dataforest's visual identity is its **data-first hierarchy**. Rather than decorative elements, structure emerges from thoughtful spacing, clear typographic scale, and purposeful use of borders and subtle shadows. Tables, pricing cards, and specification lists are treated as primary content—not afterthoughts—with dedicated styling that makes complex information instantly digestible.

**Key Characteristics:**
- Geometric sans-serif font stack with optimized legibility for technical content
- Cool neutral palette: blue-gray undertones (`#f8f9fa`, `#6c757d`, `#343a40`) for professional, technical feel
- Deep charcoal text (`#1a1a1a`) for maximum readability of dense information
- Subtle borders: `1px solid #e9ecef` for structure without visual weight
- Minimal shadow system: single-layer, low-opacity shadows for subtle elevation
- Primary accent: dataforest Blue (`#0d6efd` or similar) for CTAs and interactive states
- Success/Status colors: Green (`#198754`), Warning (`#ffc107`), Error (`#dc3545`) for system feedback
- 4px base spacing unit with modular scale for technical precision

---

## 2. Color Palette & Roles

### Primary Brand
- **dataforest Black** (`#1a1a1a` / `rgba(26, 26, 26, 0.98)`): Primary headings, body copy, critical UI text. Deep charcoal for readability without eye strain.
- **Pure White** (`#ffffff`): Page background, card surfaces, table backgrounds, button text on blue.
- **dataforest Blue** (`#0d6efd` or `#0066cc`): Primary CTA, link color, interactive accent, focus states. The core brand color for actions.

### Secondary Brand
- **Deep Navy** (`#0a2540`): Secondary brand emphasis, dark mode surfaces, footer backgrounds.
- **Active Blue** (`#084298`): Button active/pressed state, darker variant for interaction feedback.

### Technical Neutral Scale
- **Cloud White** (`#f8f9fa`): Section backgrounds, table row alternation, subtle card fills. Slight blue tint for technical feel.
- **Steel Gray** (`#343a40`): Secondary headings, dark surface text, muted labels.
- **Medium Gray** (`#6c757d`): Body secondary text, descriptions, table cell content.
- **Light Gray** (`#adb5bd`): Placeholder text, disabled states, border fallbacks.
- **Border Gray** (`#e9ecef`): Standard division borders, table cell borders, card outlines.

### Semantic & Status Colors
- **Success Green** (`#198754`): Active states, completed tasks, positive indicators, healthy system status.
- **Warning Amber** (`#ffc107`): Pending states, caution indicators, attention-required alerts.
- **Error Red** (`#dc3545`): Failed actions, critical errors, destructive action confirmation.
- **Info Cyan** (`#0dcaf0`): Informational badges, neutral status indicators, documentation highlights.
- **Premium Purple** (`#6f42c1`): Premium features, advanced options, enterprise-tier indicators.

### Interactive States
- **Link Blue** (`#0d6efd`): Primary link color with subtle underline on hover.
- **Link Hover** (`#0a58ca`): Darkened blue for hover/active link states.
- **Focus Ring** (`#86b7fe`): Visible focus indicator for accessibility (blue with outer glow).
- **Badge Background** (`#e7f1ff`): Light blue tint for status badges and tags.
- **Badge Text** (`#084298`): Darker blue for badge text readability.

### Shadows & Depth
- **Card Shadow** (`0 2px 4px rgba(0, 0, 0, 0.08), 0 1px 2px rgba(0, 0, 0, 0.04)`): Subtle elevation for cards and interactive surfaces.
- **Modal Shadow** (`0 8px 24px rgba(0, 0, 0, 0.12), 0 4px 8px rgba(0, 0, 0, 0.08)`): Deeper elevation for modals and overlays.
- **Whisper Border** (`1px solid #e9ecef`): Standard division border for tables, cards, and sections.

---

## 3. Typography Rules

### Font Family
- **Primary**: `Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif`
- **Monospace (Code/Technical)**: `"SF Mono", Monaco, "Cascadia Code", "Roboto Mono", Consolas, "Courier New", monospace`
- **OpenType Features**: `"tnum"` (tabular numerals) enabled for pricing tables and technical specs to ensure alignment.

### Hierarchy

| Role | Font | Size | Weight | Line Height | Letter Spacing | Notes |
|------|------|------|--------|-------------|----------------|-------|
| Display Hero | Inter | 48px (3.00rem) | 700 | 1.10 | -0.02em | Maximum impact for primary headlines |
| Section Heading | Inter | 32px (2.00rem) | 700 | 1.20 | -0.01em | Feature section titles, page headers |
| Card Title | Inter | 24px (1.50rem) | 600 | 1.25 | normal | Product tier titles, feature cards |
| Sub-heading | Inter | 20px (1.25rem) | 600 | 1.30 | normal | Content section headers, table titles |
| Body Large | Inter | 18px (1.125rem) | 400 | 1.50 | normal | Introductions, feature descriptions |
| Body | Inter | 16px (1.00rem) | 400 | 1.50 | normal | Standard reading text, documentation |
| Body Technical | Inter | 16px (1.00rem) | 500 | 1.40 | normal | Specifications, API parameters, technical labels |
| UI Label | Inter | 14px (0.875rem) | 500 | 1.43 | 0.01em | Form labels, navigation, button text |
| Caption | Inter | 13px (0.8125rem) | 400 | 1.40 | normal | Metadata, helper text, footnotes |
| Micro / Badge | Inter | 12px (0.75rem) | 600 | 1.33 | 0.02em | Status badges, tags, inline labels |
| Code / Monospace | SF Mono | 14px (0.875rem) | 400 | 1.40 | normal | API examples, CLI commands, technical output |

### Principles
- **Tabular numerals for data**: `"tnum"` OpenType feature ensures prices, specs, and metrics align vertically in tables.
- **Four-weight system**: 400 (body/reading), 500 (UI/technical labels), 600 (emphasis/headings), 700 (display/hero). Enables clear hierarchy without visual clutter.
- **Technical readability**: Line-height tightens slightly for technical content (1.40) to keep related specs visually grouped, while body text maintains 1.50 for comfortable reading.
- **Monospace for code**: Dedicated monospace stack for all technical content, ensuring consistent character width for alignment of code samples and CLI output.
- **Responsive scaling**: Font sizes scale proportionally across breakpoints, with display headlines reducing to 32px → 24px on mobile while maintaining weight hierarchy.

---

## 4. Component Stylings

### Buttons

**Primary Blue**
- Background: `#0d6efd` (dataforest Blue)
- Text: `#ffffff`
- Padding: `10px 20px` (vertical × horizontal)
- Radius: `6px` (subtle rounding for technical precision)
- Border: `1px solid transparent`
- Hover: background `#0b5ed7`, subtle scale `transform: scale(1.02)`
- Active: background `#0a58ca`, `transform: scale(0.98)`
- Focus: `2px solid #86b7fe` outline + `0 0 0 4px rgba(13, 110, 253, 0.25)` shadow
- Use: Primary CTAs ("Create a seed now", "Configure Seed", "Deploy")

**Secondary / Outline**
- Background: `transparent`
- Text: `#0d6efd`
- Border: `1px solid #0d6efd`
- Padding: `10px 20px`
- Radius: `6px`
- Hover: background `rgba(13, 110, 253, 0.08)`, text `#0a58ca`
- Active: background `rgba(13, 110, 253, 0.15)`
- Use: Secondary actions, cancel buttons, alternative flows

**Ghost / Text Button**
- Background: `transparent`
- Text: `#6c757d` (Medium Gray)
- Padding: `8px 12px`
- Hover: text `#343a40`, subtle background `#f8f9fa`
- Use: Tertiary actions, inline links, "View all FAQs", "Contact an expert"

**Pill Badge**
- Background: `#e7f1ff` (tinted blue)
- Text: `#084298` (Active Blue)
- Padding: `4px 10px`
- Radius: `9999px` (full pill)
- Font: `12px weight 600`
- Use: Status indicators ("Active", "New", "Beta"), feature tags, plan labels

### Cards & Containers

**Product Tier Card**
- Background: `#ffffff`
- Border: `1px solid #e9ecef`
- Radius: `12px`
- Shadow: `0 2px 4px rgba(0, 0, 0, 0.08), 0 1px 2px rgba(0, 0, 0, 0.04)`
- Padding: `24px`
- Hover: border `#0d6efd`, shadow intensifies slightly
- Header: Plan name at 24px weight 600, tagline at 14px weight 400 color `#6c757d`
- Divider: `1px solid #e9ecef` with `24px` vertical spacing
- Price: Large numeric display at 32px weight 700, currency/unit at 14px weight 400
- CTA: Full-width primary button at card bottom

**Specification Table Card**
- Background: `#ffffff`
- Border: `1px solid #e9ecef`
- Radius: `8px` (slightly tighter for data-dense content)
- Shadow: None or `0 1px 2px rgba(0, 0, 0, 0.04)` for subtle lift
- Header row: Background `#f8f9fa`, text `#343a40` weight 600
- Body rows: Alternating `#ffffff` / `#f8f9fa` for scanability
- Cell padding: `12px 16px`
- Border between rows: `1px solid #e9ecef`
- Numeric columns: Right-aligned, tabular numerals enabled

**Feature / Value Card**
- Background: `#f8f9fa` (Cloud White) for section alternation
- Border: None or `1px solid #e9ecef` for contained cards
- Radius: `12px`
- Icon/illustration: Top-aligned, 48×48px or 64×64px
- Title: `20px weight 600`, color `#1a1a1a`
- Description: `16px weight 400`, color `#6c757d`, line-height 1.50
- Use: "Why dataforest Cloud" sections, trust indicators, value propositions

### Inputs & Forms

**Text Input / Textarea**
- Background: `#ffffff`
- Text: `#1a1a1a`
- Border: `1px solid #ced4da` (slightly darker than standard border)
- Padding: `10px 14px`
- Radius: `6px`
- Placeholder: `#6c757d` at 400 weight
- Focus: Border `#0d6efd`, outline `2px solid #86b7fe`, shadow `0 0 0 4px rgba(13, 110, 253, 0.25)`
- Error state: Border `#dc3545`, helper text color `#dc3545`

**Select Dropdown**
- Same base styles as text input
- Chevron icon: `#6c757d`, right-aligned
- Options: White background, hover `#f8f9fa`, selected `#e7f1ff`

**Checkbox / Radio**
- Size: `18×18px`
- Border: `1px solid #ced4da`, radius `4px` (checkbox) / `50%` (radio)
- Checked: Background `#0d6efd`, checkmark/icon `#ffffff`
- Focus: Outer ring `2px solid #86b7fe`
- Label: `14px weight 500`, color `#1a1a1a`, left-aligned with 8px gap

### Navigation

**Primary Header Nav**
- Background: `#ffffff` with subtle bottom border `1px solid #e9ecef`
- Logo: Left-aligned, 32×32px icon + wordmark
- Links: `14px weight 500`, color `#1a1a1a`, padding `12px 16px`
- Hover: Color `#0d6efd`, subtle background `#f8f9fa`
- Active: Color `#0d6efd`, bottom border `2px solid #0d6efd`
- CTA Button: Primary blue pill button right-aligned ("Create a seed now")
- Mobile: Hamburger menu collapses to vertical list with same spacing

**Sidebar Navigation (Dashboard)**
- Background: `#f8f9fa` or `#ffffff` with right border
- Width: `240px` desktop, collapsible to icon-only
- Items: `14px weight 500`, padding `10px 16px`, radius `6px`
- Active: Background `#e7f1ff`, text `#084298`, left accent bar `3px solid #0d6efd`
- Icons: 16×16px, left of label, color `#6c757d` (active: `#0d6efd`)
- Section headers: `12px weight 600`, color `#6c757d`, uppercase, letter-spacing `0.05em`

### Tables & Data Display

**Pricing Table**
- Container: `1px solid #e9ecef` border, `12px` radius, white background
- Header: `#f8f9fa` background, `14px weight 600`, padding `14px 16px`
- Body rows: `16px weight 400`, padding `12px 16px`, alternating row backgrounds
- Numeric cells: Right-aligned, tabular numerals, `16px weight 500` for prices
- Action column: Right-aligned, contains primary/ghost buttons
- Footer: `#f8f9fa` background, `14px weight 400` color `#6c757d`, "All prices incl. 19% VAT"

**Specification List**
- Layout: Two-column grid (label / value) on desktop, stacked on mobile
- Label: `14px weight 500`, color `#343a40`
- Value: `16px weight 400`, color `#1a1a1a`, tabular numerals for specs
- Divider: `1px dashed #e9ecef` between items
- Use: Product specs, plan details, technical requirements

### Badges & Status Indicators

**Status Badge**
- Background: Semantic color at 15% opacity (e.g., `rgba(25, 135, 84, 0.15)` for success)
- Text: Semantic color at 100% (e.g., `#198754`)
- Padding: `4px 10px`
- Radius: `9999px`
- Font: `12px weight 600`, uppercase optional
- Use: Seed status ("running", "stopped", "provisioning"), plan tags ("CPU-optimized")

**Plan Badge**
- Background: `#e7f1ff` (tinted blue)
- Text: `#084298` (Active Blue)
- Padding: `6px 12px`
- Radius: `6px` (slightly squared for technical feel)
- Font: `13px weight 600`
- Use: Plan tier labels ("Entry", "Standard", "Performance")

---

## 5. Layout Principles

### Spacing System
- Base unit: `4px` (technical precision)
- Scale: `4, 8, 12, 16, 20, 24, 32, 40, 48, 64, 80, 96, 128px`
- Micro-adjustments: `2px, 6px, 10px, 14px` for fine-tuning component internals
- Section padding: `64px` desktop, `40px` tablet, `24px` mobile
- Card internal padding: `24px` standard, `16px` compact for data-dense cards

### Grid & Container
- Max content width: `1200px` centered with `24px` horizontal padding on mobile
- Hero section: Centered single-column, `80-120px` top padding for visual breathing room
- Feature sections: `1-3 column` responsive grid (1 mobile → 2 tablet → 3 desktop)
- Pricing tables: Full-width within container, horizontal scroll on small mobile
- Dashboard: Two-column layout (sidebar + main content) with flexible main area

### Whitespace Philosophy
- **Generous vertical rhythm**: `64px` between major sections creates clear content separation and reduces cognitive load for technical decision-making.
- **Functional alternation**: White (`#ffffff`) sections for primary content alternate with Cloud White (`#f8f9fa`) for secondary/feature sections, creating visual rhythm without decorative elements.
- **Data density with clarity**: Tables and specification lists use compact row spacing (`12px` vertical padding) but maintain clear column separation and alternating row backgrounds for scanability.
- **Focused content islands**: Body text blocks are surrounded by ample margin (`24-32px`) to create readable "islands" of content within the technical interface.

### Border Radius Scale
- Micro (`4px`): Form inputs, small interactive elements, badge corners
- Subtle (`6px`): Buttons, cards, dropdowns, modal corners
- Standard (`8px`): Data tables, specification cards, contained sections
- Comfortable (`12px`): Primary product cards, feature containers, modal dialogs
- Full Pill (`9999px`): Status badges, tags, inline labels
- Circle (`50%`): Avatars, toggle switches, loading indicators

---

## 6. Depth & Elevation

| Level | Treatment | Use |
|-------|-----------|-----|
| Flat (Level 0) | No shadow, no border | Page background, static content blocks, section backgrounds |
| Subtle Border (Level 1) | `1px solid #e9ecef` | Table borders, card outlines, form field defaults, section dividers |
| Soft Lift (Level 2) | `0 2px 4px rgba(0,0,0,0.08), 0 1px 2px rgba(0,0,0,0.04)` | Interactive cards, product tiers, hover states, dropdown menus |
| Modal Depth (Level 3) | `0 8px 24px rgba(0,0,0,0.12), 0 4px 8px rgba(0,0,0,0.08)` | Modals, overlays, popovers, contextual menus |
| Focus (Accessibility) | `0 0 0 4px rgba(13, 110, 253, 0.25)` + `2px solid #86b7fe` outline | Keyboard focus on all interactive elements |

**Shadow Philosophy**: dataforest's elevation system prioritizes **clarity over decoration**. Shadows are minimal, single- or double-layer, with low opacity to suggest interactivity without visual noise. The goal is to guide attention to actionable elements (cards, buttons, modals) while keeping the interface feeling "flat" and technical—appropriate for infrastructure management where cognitive load should be minimized.

### Decorative Elements
- **Minimal illustration**: Simple line icons or abstract geometric shapes for feature sections (e.g., cloud, server, shield icons)
- **No decorative gradients**: Backgrounds are solid colors; visual interest comes from content hierarchy and spacing
- **Trust indicators**: Badges, certifications, and metrics displayed in clean, scannable formats (e.g., "100% Made in Germany", "16 years expertise")
- **Product screenshots**: Dashboard/workspace previews with `1px solid #e9ecef` border and `8px` radius, displayed at consistent aspect ratios

---

## 7. Responsive Behavior

### Breakpoints
| Name | Width | Key Changes |
|------|-------|-------------|
| Mobile Small | <480px | Single column, minimal padding, stacked pricing cards |
| Mobile | 480-768px | Standard mobile layout, horizontal scroll for wide tables |
| Tablet | 768-1024px | Two-column grids, expanded padding, sidebar collapsible |
| Desktop Small | 1024-1280px | Standard desktop layout, full navigation visible |
| Desktop | 1280-1440px | Full layout, maximum content width `1200px` |
| Large Desktop | >1440px | Centered content, generous margins, optional sidebar expansion |

### Touch Targets
- Buttons: Minimum `44×44px` tap target (padding ensures this even with smaller visual size)
- Navigation links: `14px` text with `12px` vertical padding for comfortable tap area
- Form inputs: `44px` minimum height for easy selection
- Table actions: Action buttons in dedicated column with adequate spacing
- Mobile menu toggle: Standard hamburger icon with `48×48px` touch target

### Collapsing Strategy
- **Hero section**: Display headline `48px → 32px → 24px` on mobile; subtitle and CTA stack vertically
- **Navigation**: Horizontal nav + CTA → hamburger menu with vertical list; sidebar collapses to icon-only
- **Pricing cards**: 3-column grid → 2-column → single column stacked with horizontal scroll option
- **Specification tables**: Full table → horizontal scroll container with sticky first column on mobile
- **Feature grids**: 3-column → 2-column → single column; icons scale proportionally
- **Footer**: Multi-column → stacked single column with clear section headers
- **Section spacing**: `64px → 40px → 24px` vertical padding on mobile for content density

### Image & Media Behavior
- Product screenshots: Maintain aspect ratio with `max-width: 100%`, `height: auto`
- Icons: SVG with `currentColor` for theme consistency, scale with `em` units
- Dashboard previews: Lazy-loaded with placeholder skeleton, consistent border/radius treatment
- Logo/brand assets: High-DPI variants for retina displays, fallback to SVG

---

## 8. Accessibility & States

### Focus System
- All interactive elements receive visible, high-contrast focus indicators
- Focus outline: `2px solid #86b7fe` + `0 0 0 4px rgba(13, 110, 253, 0.25)` shadow for depth
- Tab navigation: Logical order matching visual layout; skip links for main content
- Screen reader support: ARIA labels for icon-only buttons, proper heading hierarchy, table headers with `scope`

### Interactive States
- **Default**: Standard appearance with subtle borders and neutral colors
- **Hover**: Color shift on text (`#1a1a1a → #0d6efd` for links), subtle background tint (`#f8f9fa`) for cards/buttons, scale `1.02` for primary buttons
- **Active/Pressed**: Darker background variant, scale `0.98` for tactile feedback
- **Focus**: Blue outline ring with shadow reinforcement (as above)
- **Disabled**: Opacity `0.6`, background `#f8f9fa`, text `#6c757d`, cursor `not-allowed`, no hover effects
- **Loading**: Skeleton screens for async content, spinner icons with `aria-live="polite"`

### Color Contrast
- Primary text (`#1a1a1a`) on white: **~16:1** ratio (exceeds WCAG AAA)
- Secondary text (`#6c757d`) on white: **~4.5:1** ratio (WCAG AA for normal text)
- Primary CTA (`#0d6efd`) on white: **~4.6:1** ratio (WCAG AA for large text/buttons)
- Badge text (`#084298`) on badge bg (`#e7f1ff`): **~5.2:1** ratio (WCAG AA)
- Error text (`#dc3545`) on white: **~5.8:1** ratio (WCAG AA)

### Motion & Preference
- Respect `prefers-reduced-motion`: Disable scale transforms, use opacity transitions only
- Default transitions: `150ms ease-in-out` for hover/focus states
- Async feedback: Skeleton loaders with subtle pulse animation (`opacity 0.6 → 1.0`)
- No auto-playing media or disruptive animations in core UI

---

## 9. Agent Prompt Guide

### Quick Color Reference
- Primary CTA: dataforest Blue (`#0d6efd`)
- Background: Pure White (`#ffffff`)
- Alt Background: Cloud White (`#f8f9fa`)
- Heading text: dataforest Black (`#1a1a1a`)
- Body text: dataforest Black (`#1a1a1a`)
- Secondary text: Medium Gray (`#6c757d`)
- Muted text: Light Gray (`#adb5bd`)
- Border: `1px solid #e9ecef`
- Link: dataforest Blue (`#0d6efd`)
- Focus ring: Focus Blue (`#86b7fe`) + shadow
- Success: Green (`#198754`)
- Warning: Amber (`#ffc107`)
- Error: Red (`#dc3545`)

### Example Component Prompts
- **Hero Section**: "Create a hero section on white background. Headline at 48px Inter weight 700, line-height 1.10, letter-spacing -0.02em, color #1a1a1a. Subtitle at 20px weight 600, line-height 1.30, color #6c757d. Primary CTA button (#0d6efd bg, white text, 6px radius, 10px 20px padding) and secondary ghost button (transparent bg, #0d6efd text, 1px solid #0d6efd border). Generous vertical padding: 80px top, 64px bottom."

- **Product Tier Card**: "Design a pricing card: white background, 1px solid #e9ecef border, 12px radius, subtle shadow (0 2px 4px rgba(0,0,0,0.08)). Plan title at 24px Inter weight 600, tagline at 14px weight 400 color #6c757d. Divider: 1px solid #e9ecef with 24px spacing. Price display: 32px weight 700 for amount, 14px weight 400 for '/Month' unit. Feature list: 16px weight 400, 12px line spacing. Full-width primary button at bottom."

- **Specification Table**: "Build a data table: white background, 1px solid #e9ecef border, 8px radius. Header row: #f8f9fa background, 14px weight 600, #343a40 text, 14px 16px padding. Body rows: alternating #ffffff/#f8f9fa, 16px weight 400, 12px 16px padding. Numeric columns: right-aligned, tabular numerals enabled. Action column: right-aligned with ghost button. Footer: #f8f9fa background, 14px weight 400 color #6c757d."

- **Status Badge**: "Create a status badge: semantic color background at 15% opacity (e.g., rgba(25,135,84,0.15) for success), text at 100% semantic color (#198754), 9999px radius, 4px 10px padding, 12px Inter weight 600. Use for seed status indicators like 'running', 'stopped', 'provisioning'."

- **Navigation Bar**: "Design a header nav: white background with 1px solid #e9ecef bottom border. Logo left-aligned (32×32px icon + wordmark). Links: Inter 14px weight 500, #1a1a1a text, 12px 16px padding, hover color #0d6efd with #f8f9fa background. Primary CTA button right-aligned: #0d6efd bg, white text, 6px radius. Mobile: hamburger menu collapses to vertical list."

### Iteration Guide
1. **Prioritize clarity over decoration**: Every visual element should serve a functional purpose—guide attention, communicate status, or improve scannability of technical data.
2. **Use cool neutrals**: dataforest's grays have blue undertones (#f8f9fa, #6c757d, #343a40), never warm/yellow tones. This reinforces the technical, enterprise brand.
3. **Tabular numerals for data**: Enable `"tnum"` OpenType feature for all pricing, specs, and metrics to ensure vertical alignment in tables.
4. **Four-weight typography**: 400 (body), 500 (UI labels), 600 (headings/emphasis), 700 (display). Avoid using weight 300 or lighter for body text.
5. **Subtle borders only**: Use `1px solid #e9ecef` for structure—never heavier borders or decorative dividers.
6. **Minimal shadows**: Single- or double-layer shadows with max opacity 0.12. Elevation should suggest interactivity, not create visual noise.
7. **Cloud White for alternation**: Use `#f8f9fa` for section backgrounds to create rhythm without decorative elements.
8. **6px radius for interactive elements**: Buttons, inputs, and cards use 6px radius for a precise, technical feel; reserve 12px for primary product cards.
9. **dataforest Blue is the only saturated accent**: Use `#0d6efd` sparingly for CTAs, links, and focus states. Semantic colors (green/amber/red) are reserved for system status.
10. **Design for data density**: Tables, specs, and pricing should be scannable first, beautiful second. Use alternating row backgrounds, clear column alignment, and adequate padding.

---

> **Brand Voice Alignment**: This design system supports dataforest's brand promise of *Simplified, secured, sovereign* cloud infrastructure. Every visual decision—from the cool neutral palette to the data-first typography—reinforces transparency, precision, and enterprise-grade reliability. The interface should feel like a well-engineered tool: powerful when you need it, invisible when you don't.