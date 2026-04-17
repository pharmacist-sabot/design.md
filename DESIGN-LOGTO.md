# Design System Inspired by Logto

## 1. Visual Theme & Atmosphere

Logto's design language embodies the philosophy of modern developer infrastructure: **clarity, velocity, and trust**. The visual system is built on a vibrant, gradient-forward aesthetic that signals innovation while maintaining professional rigor. Unlike minimalist "blank canvas" approaches, Logto embraces dynamic color as a functional tool—using gradients not just for decoration, but to guide attention, denote interactivity, and reinforce brand identity.

The page canvas is pure white (`#ffffff`) or soft off-white (`#fafafa`) for content areas, with strategic use of deep navy (`#0f172a`) or rich purple (`#1e1b4b`) for hero sections and dark-mode surfaces. Text is never pure black; instead, Logto uses a layered gray scale with subtle blue undertones (`#1e293b`, `#334155`, `#64748b`) to reduce eye strain and create visual hierarchy.

What distinguishes Logto's visual language is its **gradient-first philosophy**. Primary actions, key highlights, and brand moments leverage multi-stop linear gradients that flow from vibrant blue (`#5d34f2`) through electric purple (`#7c5cff`) to soft cyan (`#55ccff`). These gradients are never arbitrary—they follow calculated HSL adjustments to ensure harmony and accessibility.

**Key Characteristics:**
- **Gradient-driven accents**: Primary CTAs and interactive elements use calculated gradients (`#5d34f2 → #7c5cff`, `#1fa2ff → #12d8fa`) generated via HSL manipulation
- **HSL-based color system**: All brand colors derive from a base hue with systematic lightness/saturation adjustments for consistency
- **CSS variable architecture**: Design tokens exposed as `--color-brand-*` variables for easy theming and customization
- **Inter font family**: Clean, highly legible system-ui fallback stack optimized for code-heavy interfaces
- **Soft depth model**: Subtle shadows (`0px 4px 20px rgba(0,0,0,0.08)`) and layered borders create hierarchy without visual weight
- **Pill-shaped interactive elements**: Buttons, badges, and inputs use generous border-radius (`9999px` or `12px`) for approachability
- **8px base spacing unit** with modular scale for rhythmic, predictable layouts

---

## 2. Color Palette & Roles

### Primary Brand Colors
| Token | Value | HSL | Usage |
|-------|-------|-----|-------|
| `--color-brand-primary` | `#5d34f2` | `hsl(253, 88%, 58%)` | Primary buttons, active states, key accents |
| `--color-brand-hover` | `#7c5cff` | `hsl(253, 88%, 68%)` | Button hover, link hover, interactive highlights |
| `--color-brand-active` | `#3c00c2` | `hsl(253, 88%, 48%)` | Button pressed, active navigation |
| `--color-brand-gradient-start` | `#5d34f2` | — | Gradient start for primary CTAs |
| `--color-brand-gradient-end` | `#7c5cff` | — | Gradient end for primary CTAs |

### Secondary & Accent Gradients
| Token | Value | Usage |
|-------|-------|-------|
| `--gradient-cyan` | `linear-gradient(135deg, #1fa2ff, #12d8fa)` | Secondary CTAs, feature highlights |
| `--gradient-purple-pink` | `linear-gradient(135deg, #5d34f2, #f07eff)` | Premium features, decorative accents |
| `--gradient-warm` | `linear-gradient(135deg, #fff480, #ffab40)` | Success states, celebratory moments |

### Neutral Scale (Light Mode)
| Token | Value | Usage |
|-------|-------|-------|
| `--color-text-primary` | `#0f172a` | Headings, primary body text |
| `--color-text-secondary` | `#334155` | Body copy, labels, descriptions |
| `--color-text-tertiary` | `#64748b` | Placeholder text, metadata, disabled states |
| `--color-bg-primary` | `#ffffff` | Page background, card surfaces |
| `--color-bg-secondary` | `#f8fafc` | Section alternation, subtle containers |
| `--color-bg-tertiary` | `#f1f5f9` | Input backgrounds, hover states |
| `--color-border-default` | `#e2e8f0` | Default borders, dividers |
| `--color-border-focus` | `#94a3b8` | Focus rings, active borders |

### Semantic Colors
| Token | Value | Usage |
|-------|-------|-------|
| `--color-success` | `#10b981` | Success states, completion badges |
| `--color-warning` | `#f59e0b` | Warning indicators, attention states |
| `--color-error` | `#ef4444` | Error messages, destructive actions |
| `--color-info` | `#3b82f6` | Informational badges, neutral accents |

### Dark Mode Tokens
| Token | Value | Usage |
|-------|-------|-------|
| `--color-bg-dark-primary` | `#0f172a` | Dark page background |
| `--color-bg-dark-secondary` | `#1e293b` | Dark card surfaces |
| `--color-text-dark-primary` | `#f8fafc` | Dark mode primary text |
| `--color-text-dark-secondary` | `#cbd5e1` | Dark mode secondary text |
| `--color-border-dark` | `#334155` | Dark mode borders |

### Interactive States
| State | Background | Text | Border | Shadow |
|-------|-----------|------|--------|--------|
| Default | `--color-brand-primary` | `#ffffff` | `1px solid transparent` | `0px 2px 8px rgba(93,52,242,0.24)` |
| Hover | `--color-brand-hover` | `#ffffff` | `1px solid transparent` | `0px 4px 16px rgba(124,92,255,0.32)` |
| Active | `--color-brand-active` | `#ffffff` | `1px solid transparent` | `0px 1px 4px rgba(60,0,194,0.4)` |
| Focus | `--color-brand-primary` | `#ffffff` | `2px solid #94a3b8` | `0px 0px 0px 4px rgba(93,52,242,0.16)` |
| Disabled | `#cbd5e1` | `#64748b` | `1px solid #e2e8f0` | `none` |

---

## 3. Typography Rules

### Font Family Stack
```css
font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', 
             'Roboto', 'Helvetica Neue', Arial, 'Noto Sans', 
             sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji';
```
- **Primary**: Inter (variable font preferred) for optimal screen readability
- **Fallbacks**: System-ui stack for performance and consistency across platforms
- **Monospace**: `ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace` for code snippets

### Type Scale & Hierarchy

| Role | Font Size | Weight | Line Height | Letter Spacing | Usage |
|------|-----------|--------|-------------|----------------|-------|
| Display Hero | 48px (3rem) | 700 | 1.1 | -0.02em | Main headlines, value props |
| Display Secondary | 40px (2.5rem) | 700 | 1.15 | -0.015em | Section headers, feature titles |
| Heading 1 | 32px (2rem) | 600 | 1.2 | -0.01em | Page titles, major sections |
| Heading 2 | 24px (1.5rem) | 600 | 1.3 | normal | Card titles, subsection headers |
| Heading 3 | 20px (1.25rem) | 600 | 1.4 | normal | Form labels, list headers |
| Body Large | 18px (1.125rem) | 400 | 1.5 | normal | Intro paragraphs, feature descriptions |
| Body | 16px (1rem) | 400 | 1.5 | normal | Standard reading text |
| Body Small | 14px (0.875rem) | 400 | 1.43 | normal | Captions, helper text |
| Label | 14px (0.875rem) | 500 | 1.43 | 0.01em | Form labels, button text, navigation |
| Code | 14px (0.875rem) | 400 | 1.4 | normal | Inline code, terminal snippets |
| Badge | 12px (0.75rem) | 600 | 1.33 | 0.05em | Status tags, version labels |

### Typography Principles
- **Progressive weight scaling**: Headings use 600-700 weight for scannability; body text stays at 400 for readability
- **Tight leading for display**: Line-height compresses as size increases (1.1 at 48px → 1.5 at 16px) for visual density
- **Code-friendly monospace**: Dedicated monospace stack with consistent sizing for technical content
- **Responsive scaling**: All type sizes use `clamp()` or fluid typography for seamless mobile-to-desktop adaptation
- **Semantic token naming**: Typography tokens follow `--text-{role}-{property}` pattern for maintainability

---

## 4. Component Stylings

### Buttons

**Primary Gradient Button**
```css
background: linear-gradient(135deg, var(--color-brand-gradient-start), var(--color-brand-gradient-end));
color: #ffffff;
padding: 12px 24px;
border-radius: 9999px;
border: 1px solid transparent;
font-weight: 600;
font-size: 16px;
box-shadow: 0px 2px 8px rgba(93, 52, 242, 0.24);
transition: all 0.2s ease;
```
- **Hover**: Gradient shifts to lighter stop (`#7c5cff` dominant), shadow intensifies
- **Active**: Scale transform `scale(0.98)`, gradient darkens
- **Focus**: 2px outer ring `#94a3b8` + subtle glow shadow

**Secondary Outline Button**
```css
background: transparent;
color: var(--color-brand-primary);
border: 1px solid var(--color-brand-primary);
padding: 12px 24px;
border-radius: 9999px;
font-weight: 600;
```
- **Hover**: Background fills with `rgba(93,52,242,0.08)`, border darkens
- **Active**: Background `rgba(93,52,242,0.16)`, scale `0.98`

**Ghost / Text Button**
```css
background: transparent;
color: var(--color-text-secondary);
padding: 8px 16px;
border-radius: 8px;
font-weight: 500;
```
- **Hover**: Text color shifts to `--color-brand-primary`, subtle background tint

**Icon Button**
```css
width: 40px; height: 40px;
border-radius: 12px;
display: flex; align-items: center; justify-content: center;
background: var(--color-bg-secondary);
color: var(--color-text-secondary);
```
- **Hover**: Background `--color-bg-tertiary`, color `--color-brand-primary`

### Input Fields
```css
background: #ffffff;
border: 1px solid var(--color-border-default);
border-radius: 12px;
padding: 12px 16px;
font-size: 16px;
color: var(--color-text-primary);
transition: border-color 0.2s, box-shadow 0.2s;
```
- **Focus**: Border `--color-brand-primary`, box-shadow `0px 0px 0px 4px rgba(93,52,242,0.16)`
- **Error**: Border `--color-error`, subtle red glow
- **Placeholder**: `--color-text-tertiary`, `font-style: italic`

### Cards & Containers
```css
background: var(--color-bg-primary);
border: 1px solid var(--color-border-default);
border-radius: 16px;
padding: 24px;
box-shadow: 0px 4px 20px rgba(0, 0, 0, 0.08);
transition: box-shadow 0.2s, transform 0.2s;
```
- **Hover**: Shadow intensifies to `0px 8px 32px rgba(0,0,0,0.12)`, subtle `translateY(-2px)`
- **Interactive cards**: Cursor `pointer`, border highlights on hover
- **Featured cards**: Gradient border `1px solid transparent` with `background: linear-gradient(...) border-box`

### Navigation
- **Desktop**: Horizontal nav with logo left, links center-aligned, CTA right
- **Links**: `--color-text-secondary`, weight 500, hover → `--color-brand-primary`
- **Active state**: Underline `2px solid --color-brand-primary` or background tint
- **Mobile**: Hamburger menu with slide-in panel, large touch targets (44px min)

### Badges & Tags
```css
background: rgba(93, 52, 242, 0.12);
color: var(--color-brand-primary);
padding: 4px 12px;
border-radius: 9999px;
font-size: 12px;
font-weight: 600;
letter-spacing: 0.05em;
```
- **Variants**: Success (`rgba(16,185,129,0.12)`), Warning (`rgba(245,158,11,0.12)`), Error (`rgba(239,68,68,0.12)`)
- **Pill style**: Full rounded corners for status indicators

### Code Blocks & Terminal
```css
background: #1e1b4b;
color: #e2e8f0;
border-radius: 12px;
padding: 16px;
font-family: var(--font-mono);
font-size: 14px;
line-height: 1.5;
border: 1px solid #334155;
```
- **Syntax highlighting**: Semantic tokens for keywords, strings, comments
- **Copy button**: Top-right corner, subtle hover effect

---

## 5. Layout Principles

### Spacing System
- **Base unit**: 8px
- **Scale**: `2, 4, 8, 12, 16, 20, 24, 32, 40, 48, 64, 80, 96, 128px`
- **CSS variables**: `--space-{size}` tokens for consistent application
- **Fluid spacing**: `clamp()` used for responsive padding/margin on key sections

### Grid & Container
- **Max content width**: 1200px centered with `padding: 0 24px`
- **Hero sections**: Full-bleed gradient backgrounds with centered content column
- **Feature grids**: 3-column desktop → 2-column tablet → 1-column mobile
- **Card layouts**: Consistent 24px internal padding, 16px gap between items

### Whitespace Philosophy
- **Generous section padding**: 80-120px vertical spacing between major content blocks
- **Content density balance**: Tight line-height (1.5) for body text surrounded by ample margins
- **Visual breathing room**: Cards and components never touch edges; minimum 24px container padding
- **Gradient transitions**: Section background shifts use soft gradient fades, not hard color breaks

### Border Radius Scale
| Size | Value | Usage |
|------|-------|-------|
| Micro | 4px | Checkbox, radio, small toggles |
| Small | 8px | Buttons (secondary), inputs, menu items |
| Medium | 12px | Cards, modals, dropdowns |
| Large | 16px | Featured cards, hero containers |
| Pill | 9999px | Primary buttons, badges, tags |
| Circle | 50% | Avatars, toggle knobs, loading spinners |

---

## 6. Depth & Elevation

| Level | Shadow Definition | Use Case |
|-------|------------------|----------|
| Flat (0) | `none` | Text blocks, static content |
| Subtle (1) | `0px 1px 2px rgba(0,0,0,0.04)` | Input borders, dividers, list items |
| Card (2) | `0px 4px 20px rgba(0,0,0,0.08)` | Standard cards, dropdowns, tooltips |
| Elevated (3) | `0px 8px 32px rgba(0,0,0,0.12)` | Modals, popovers, active cards |
| Floating (4) | `0px 16px 48px rgba(93,52,242,0.24)` | Primary CTAs, featured elements, drag handles |

**Shadow Philosophy**: Logto's elevation system uses **color-aware shadows**—brand-colored glows for interactive elements (`rgba(93,52,242,0.24)`) and neutral shadows for structural depth. Multi-layer shadows are avoided in favor of single, well-tuned blurs for performance and clarity.

### Decorative Elements
- **Gradient orbs**: Soft, blurred gradient circles (`filter: blur(60px)`) as background accents
- **Grid patterns**: Subtle dotted or lined grids (`1px dashed rgba(148,163,184,0.2)`) for technical sections
- **Code-themed illustrations**: Abstract terminal windows, connection diagrams, flowcharts in brand colors

---

## 7. Responsive Behavior

### Breakpoints
| Name | Width | Key Adaptations |
|------|-------|----------------|
| Mobile | `< 640px` | Single-column layout, stacked navigation, reduced padding |
| Tablet | `640px – 1024px` | 2-column grids, condensed navigation, adjusted typography |
| Desktop | `1024px – 1440px` | Full 3-column layouts, horizontal nav, standard spacing |
| Large Desktop | `> 1440px` | Centered content with generous margins, enhanced visuals |

### Touch & Interaction Targets
- **Minimum tap target**: 44×44px for all interactive elements
- **Button padding**: Minimum 12px vertical, 24px horizontal on mobile
- **Form inputs**: 48px minimum height for comfortable mobile entry
- **Navigation links**: 44px minimum height with adequate spacing

### Collapsing Strategy
- **Hero headlines**: 48px → 40px → 32px with proportional line-height adjustment
- **Navigation**: Horizontal links → hamburger menu with slide-in panel
- **Feature cards**: 3-column → 2-column → stacked vertical
- **Code snippets**: Horizontal scroll containers with copy-to-clipboard button
- **Footer**: Multi-column grid → stacked sections with clear visual separation

### Image & Media Behavior
- **Product screenshots**: Maintain 16:9 or 4:3 aspect ratios with responsive `max-width: 100%`
- **Gradient backgrounds**: Use `background-size: cover` with `background-position: center`
- **Iconography**: SVG sprites with `currentColor` for theme adaptability
- **Dark mode images**: Provide `prefers-color-scheme: dark` variants where critical

---

## 8. Accessibility & States

### Focus Management
- **Visible focus indicators**: 2px outline `--color-border-focus` + subtle glow shadow
- **Keyboard navigation**: Logical tab order, skip-to-content link, focus trapping in modals
- **Reduced motion**: Respect `prefers-reduced-motion` for animations and transitions

### Interactive States Matrix
| Element | Default | Hover | Active | Focus | Disabled |
|---------|---------|-------|--------|-------|----------|
| Primary Button | Gradient + shadow | Lighter gradient + intensified shadow | Scale 0.98 + darker gradient | Outline ring + glow | Opacity 0.6, no shadow |
| Secondary Button | Outline + text color | Background tint + darker border | Scale 0.98 + background fill | Outline ring | Grayed outline + text |
| Input Field | Default border | Border `--color-border-focus` | — | Brand border + glow | Gray background + border |
| Link | `--color-text-secondary` | `--color-brand-primary` + underline | — | Outline ring | `--color-text-tertiary` |
| Card | Default shadow | Intensified shadow + slight lift | — | — | Opacity 0.7 |

### Color Contrast Compliance
- **Primary text on white**: `#0f172a` on `#ffffff` = 16.5:1 (WCAG AAA)
- **Secondary text on white**: `#334155` on `#ffffff` = 8.2:1 (WCAG AAA)
- **Brand button text**: `#ffffff` on `#5d34f2` = 4.8:1 (WCAG AA for large text)
- **Gradient button text**: Tested at both gradient endpoints for minimum 4.5:1 ratio
- **Dark mode text**: `#f8fafc` on `#0f172a` = 15.1:1 (WCAG AAA)

### ARIA & Semantic HTML
- All interactive elements include appropriate `role`, `aria-*`, and keyboard handlers
- Form inputs paired with `<label>` elements or `aria-labelledby`
- Dynamic content updates announced via `aria-live` regions
- Skip links and landmark roles for screen reader navigation

---

## 9. Agent Prompt Guide

### Quick Color Reference
- **Primary CTA gradient**: `linear-gradient(135deg, #5d34f2, #7c5cff)`
- **Brand primary**: `#5d34f2` (HSL: 253, 88%, 58%)
- **Brand hover**: `#7c5cff` (HSL: 253, 88%, 68%)
- **Background primary**: `#ffffff` (light) / `#0f172a` (dark)
- **Text primary**: `#0f172a` (light) / `#f8fafc` (dark)
- **Border default**: `#e2e8f0` (light) / `#334155` (dark)
- **Focus ring**: `2px solid #94a3b8` + glow shadow
- **Success**: `#10b981` | **Warning**: `#f59e0b` | **Error**: `#ef4444`

### Example Component Prompts
- **Hero Section**: "Create a hero section with full-bleed gradient background (`linear-gradient(135deg, #5d34f2, #7c5cff)`). Headline at 48px Inter weight 700, line-height 1.1, color `#ffffff`. Subheading at 20px weight 400, color `rgba(255,255,255,0.9)`. Primary CTA button with gradient fill, white text, 9999px radius, 12px 24px padding. Secondary ghost button with white border, white text. Add subtle gradient orb decoration with `filter: blur(60px)`."

- **Feature Card**: "Design a feature card: white background, 1px solid `#e2e8f0` border, 16px radius. Shadow: `0px 4px 20px rgba(0,0,0,0.08)`. Icon container with gradient background (`#5d34f2 → #7c5cff`), 48px size, 12px radius. Title at 20px weight 600, `#0f172a`. Description at 16px weight 400, `#334155`. Hover: shadow intensifies to `0px 8px 32px rgba(0,0,0,0.12)`, subtle `translateY(-2px)`."

- **Input Field with Validation**: "Create an input field: white background, 1px solid `#e2e8f0` border, 12px radius, 12px 16px padding, 16px Inter. Placeholder color `#64748b`. Focus state: border `#5d34f2`, box-shadow `0px 0px 0px 4px rgba(93,52,242,0.16)`. Error state: border `#ef4444`, subtle red glow. Add helper text below at 14px `#64748b`."

- **Navigation Bar**: "Build a desktop navigation: white background, 1px bottom border `#e2e8f0`. Logo left (32px height). Center-aligned links: Inter 16px weight 500, `#334155`, hover → `#5d34f2`. Right-aligned primary CTA button with gradient fill. Mobile: hamburger icon right, slide-in panel with stacked links and CTA."

- **Code Snippet Block**: "Design a code block: background `#1e1b4b`, text `#e2e8f0`, 12px radius, 16px padding, 1px border `#334155`. Font: monospace stack, 14px, line-height 1.5. Add copy button top-right: 32px circle, `#334155` background, hover `#475569`. Include syntax highlighting tokens for keywords, strings, comments."

### Iteration Guide
1. **Always use HSL-based color generation**: When extending the palette, adjust lightness/saturation of the base hue (`253°`) rather than picking arbitrary hex values
2. **Gradients are functional, not decorative**: Use gradients to denote interactivity, hierarchy, or brand moments—not randomly. Test contrast at both gradient endpoints
3. **CSS variables are mandatory**: All colors, spacing, and typography must reference `--color-*`, `--space-*`, `--text-*` tokens for theming support
4. **Inter font with system fallbacks**: Never use decorative or serif fonts for UI text; prioritize readability for developer audiences
5. **Pill radius for primary actions**: Buttons, badges, and key interactive elements use `9999px` radius; structural elements use `12px` or `16px`
6. **Shadow color awareness**: Use brand-colored glows (`rgba(93,52,242,0.24)`) for interactive elevation, neutral shadows for structural depth
7. **Dark mode is first-class**: Every component must have explicit dark mode tokens; test contrast in both modes
8. **Accessibility is non-negotiable**: All interactive states must have visible focus indicators; color contrasts must meet WCAG AA minimum

### Theming Strategy
Logto's design system is built for **brand adaptation**. When customizing for a client:
1. Accept a single base color (hex or HSL)
2. Generate the full palette using HSL lightness adjustments: `±10%` for hover/active states
3. Apply the new values to CSS variables: `--color-brand-primary`, `--color-brand-hover`, etc.
4. Test all interactive states and dark mode variants
5. Provide a `data-theme` or `class`-based toggle for light/dark mode support

This approach ensures brand consistency while maintaining accessibility, performance, and developer experience—the core pillars of Logto's design philosophy.