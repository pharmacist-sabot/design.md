# Design System Inspired by Nuxt

## 1. Visual Theme & Atmosphere

Nuxt's design philosophy embodies **developer-first clarity**: a modern, technical aesthetic that prioritizes functionality without sacrificing warmth. The visual language is built on a crisp, high-contrast foundation with a distinctive emerald-green accent that signals innovation and performance. The interface feels like a well-organized code editor—clean, predictable, and empowering.

The page canvas uses pure white (`#ffffff`) in light mode and deep charcoal (`#09090b` to `#18181b`) in dark mode, creating a professional backdrop that reduces eye strain during extended development sessions. Text isn't pure black—it's a softened near-black (`#09090b` / `rgba(9,9,11,0.95)`) in light mode and warm off-white (`#f4f4f5`) in dark mode, ensuring optimal readability across themes.

The signature **Nuxt Green** (`#00DC82`) serves as the primary accent—a vibrant, energetic hue that conveys speed, growth, and modernity. Unlike Notion's warm neutrals, Nuxt's palette leans into cool, technical grays (`#f4f4f5`, `#71717a`, `#27272a`, `#09090b`) with subtle blue undertones, creating a distinctly digital, developer-oriented atmosphere.

Nuxt's typography relies on **Public Sans** (or system-ui fallbacks)—a geometric, highly legible sans-serif that mirrors the precision of code. Display text uses tighter line-heights (1.0–1.2) for impact, while body text maintains comfortable 1.5–1.6 line-heights for extended reading. Font weights follow a purposeful hierarchy: 400 for body, 500 for UI elements, 600 for emphasis, and 700 for headings.

**Border philosophy** emphasizes clarity over decoration: `1px solid` borders use semantic color variables (`ring-default`, `ring-accented`) with low opacity in dark mode, creating structure that adapts to theme. Shadows are subtle and purposeful—Tailwind's elevation scale (`shadow-sm` to `shadow-xl`) provides depth without visual noise.

**Key Characteristics:**
- Primary accent: Nuxt Green (`#00DC82`)—vibrant, performance-oriented
- Semantic color system: `primary`, `secondary`, `success`, `info`, `warning`, `error`, `neutral`
- CSS Variables as design tokens: `--ui-primary`, `--color-green-500`, etc. for flexible theming
- Built-in dark mode support via `@nuxtjs/color-mode` with auto-detection
- Tailwind CSS utility-first styling with CSS-first configuration via `@theme` directive
- Rounded corners scale: `rounded-md` (6px), `rounded-lg` (8px), `rounded-xl` (12px)
- 4px-based spacing system aligned with Tailwind's default scale
- Reka UI foundation for WAI-ARIA compliance and keyboard navigation

---

## 2. Color Palette & Roles

### Primary Brand Colors
- **Nuxt Green** (`#00DC82` / `--color-green-400`): Primary CTA, active states, brand accents, key interactive elements
- **Green Scale** (Tailwind-compatible):
  - `--color-green-50`: `#EFFDF5` (lightest tint)
  - `--color-green-100`: `#D9FBE8`
  - `--color-green-200`: `#B3F5D1`
  - `--color-green-300`: `#75EDAE`
  - `--color-green-400`: `#00DC82` (brand primary)
  - `--color-green-500`: `#00C16A` (hover)
  - `--color-green-600`: `#00A155` (active)
  - `--color-green-700`: `#007F45`
  - `--color-green-800`: `#016538`
  - `--color-green-900`: `#0A5331`
  - `--color-green-950`: `#052E16` (darkest)

### Core Neutrals (Light Mode)
- **Canvas White** (`#ffffff`): Page background, card surfaces, modal overlays
- **Text Primary** (`#09090b` / `rgba(9,9,11,0.95)`): Headings, body copy, primary labels
- **Text Secondary** (`#71717a` / `--color-zinc-500`): Descriptions, muted labels, placeholders
- **Border Default** (`#e4e4e7` / `--color-zinc-200`): Dividers, input borders, card outlines
- **Background Elevated** (`#f4f4f5` / `--color-zinc-100`): Hover states, subtle surfaces, section alternation

### Core Neutrals (Dark Mode)
- **Canvas Dark** (`#09090b` to `#18181b`): Page background, deep surfaces
- **Text Primary** (`#f4f4f5` / `rgba(244,244,245,0.95)`): Headings, body copy
- **Text Secondary** (`#a1a1aa` / `--color-zinc-400`): Descriptions, muted content
- **Border Default** (`#27272a` / `--color-zinc-800`): Dividers, subtle outlines
- **Background Elevated** (`#27272a` / `--color-zinc-800`): Hover states, card surfaces

### Semantic Color System
Nuxt UI uses semantic naming for maintainability and theme flexibility:

| Semantic Role | Default Color | Light Mode Usage | Dark Mode Usage |
|--------------|--------------|-----------------|-----------------|
| `primary` | `green` | CTAs, active nav, key links | Same, with adjusted contrast |
| `secondary` | `blue` | Secondary actions, complementary UI | `--color-blue-400` variants |
| `success` | `green` | Confirmations, completed states | `--color-green-400` |
| `info` | `blue` | Tooltips, help text, neutral alerts | `--color-blue-400` |
| `warning` | `yellow` | Pending states, attention indicators | `--color-amber-400` |
| `error` | `red` | Validation errors, destructive actions | `--color-red-400` |
| `neutral` | `slate` | Text, borders, disabled states | `--color-zinc-*` scale |

### Interactive States
- **Link Primary** (`--color-primary`): Underline on hover, smooth transition
- **Focus Ring** (`--ui-focus-ring`): `2px solid` outline with `--color-primary` + subtle shadow
- **Hover Overlay**: `bg-primary/10` for soft variants, `bg-primary/75` for solid buttons
- **Active State**: Scale transform (`scale(0.98)`) + darker color variant
- **Disabled**: `opacity-75` + `cursor-not-allowed`, no hover effects

### Shadows & Depth (Tailwind-aligned)
```css
/* Card elevation - 4-layer subtle stack */
--ui-shadow-card: 
  rgba(0,0,0,0.04) 0px 4px 18px,
  rgba(0,0,0,0.027) 0px 2.025px 7.85px,
  rgba(0,0,0,0.02) 0px 0.8px 2.93px,
  rgba(0,0,0,0.01) 0px 0.175px 1.04px;

/* Deep elevation for modals/overlays */
--ui-shadow-deep: 
  rgba(0,0,0,0.1) 0px 10px 15px -3px,
  rgba(0,0,0,0.1) 0px 4px 6px -2px;

/* Focus indicator */
--ui-focus-ring: 0 0 0 2px var(--color-primary), 0 0 0 4px rgba(0,220,130,0.2);
```

---

## 3. Typography Rules

### Font Family Stack
```css
--font-sans: 'Public Sans', system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif;
--font-mono: 'JetBrains Mono', 'Fira Code', Consolas, Monaco, 'Andale Mono', monospace;
```
Fonts are automatically optimized and loaded via `@nuxt/fonts` module.

### Type Scale & Hierarchy

| Role | Font | Size (rem) | Size (px) | Weight | Line Height | Letter Spacing | Use Case |
|------|------|-----------|-----------|--------|-------------|----------------|----------|
| Display XL | Public Sans | 3.5 | 56px | 700 | 1.0 | -0.02em | Hero headlines, landing page titles |
| Display LG | Public Sans | 2.5 | 40px | 700 | 1.1 | -0.015em | Section headers, feature titles |
| Heading 1 | Public Sans | 2 | 32px | 700 | 1.2 | -0.01em | Page titles, major section headers |
| Heading 2 | Public Sans | 1.5 | 24px | 700 | 1.3 | normal | Sub-section headers, card titles |
| Heading 3 | Public Sans | 1.25 | 20px | 600 | 1.4 | normal | Content headers, list titles |
| Body LG | Public Sans | 1.125 | 18px | 400 | 1.6 | normal | Introductions, feature descriptions |
| Body | Public Sans | 1 | 16px | 400 | 1.5–1.6 | normal | Standard reading text, documentation |
| Body SM | Public Sans | 0.875 | 14px | 400 | 1.5 | normal | Captions, metadata, helper text |
| UI Label | Public Sans | 0.875 | 14px | 500 | 1.4 | normal | Form labels, navigation, buttons |
| Code / Mono | JetBrains Mono | 0.875 | 14px | 400 | 1.5 | normal | Code blocks, terminal output, technical content |
| Badge | Public Sans | 0.75 | 12px | 600 | 1.33 | +0.025em | Status tags, version labels, small indicators |

### Typography Principles
- **Progressive line-height**: Tighter (1.0–1.2) for large display text, relaxed (1.5–1.6) for body reading
- **Weight hierarchy**: 400 (read), 500 (interact), 600 (emphasize), 700 (announce)
- **Monospace integration**: `--font-mono` for all code-related content with syntax highlighting support
- **Responsive scaling**: Type sizes use `clamp()` for fluid typography across breakpoints
- **OpenType features**: `font-variant-numeric: lining-nums` for consistent numerical display in tables/metrics

---

## 4. Component Stylings

### Buttons

**Primary Solid**
```css
base: rounded-md font-medium inline-flex items-center gap-1.5 px-2.5 py-1.5 text-sm
color: text-white bg-primary hover:bg-primary/90 active:bg-primary/75
focus: focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary
disabled: disabled:opacity-75 disabled:cursor-not-allowed
transition: transition-colors duration-200
```

**Outline Variant**
```css
base: rounded-md font-medium inline-flex items-center gap-1.5 px-2.5 py-1.5 text-sm
color: text-primary ring-1 ring-inset ring-primary/50 hover:bg-primary/10 active:bg-primary/15
focus: focus-visible:ring-2 focus-visible:ring-primary
```

**Soft / Subtle Variants**
```css
soft: text-primary bg-primary/10 hover:bg-primary/15
subtle: text-primary ring-1 ring-primary/25 bg-primary/10 hover:bg-primary/15
```

**Ghost & Link**
```css
ghost: text-primary hover:bg-primary/10 (transparent background)
link: text-primary hover:text-primary/75 underline-offset-4 hover:underline
```

**Size Scale**
| Size | Padding | Text | Icon Size | Use Case |
|------|---------|------|-----------|----------|
| `xs` | `px-2 py-1` | `text-xs` | `size-4` (16px) | Inline actions, dense UI |
| `sm` | `px-2.5 py-1.5` | `text-xs` | `size-4` | Secondary actions, form buttons |
| `md` | `px-2.5 py-1.5` | `text-sm` | `size-5` (20px) | **Default** - primary CTAs |
| `lg` | `px-3 py-2` | `text-sm` | `size-5` | Prominent actions, hero CTAs |
| `xl` | `px-3 py-2` | `text-base` | `size-6` (24px) | Major conversions, landing page |

### Cards & Containers

**Base Card Structure**
```css
root: rounded-lg overflow-hidden bg-default ring-1 ring-default
header: p-4 sm:px-6 border-b border-default
body: p-4 sm:p-6
footer: p-4 sm:px-6 border-t border-default
```

**Card Variants**
| Variant | Background | Border | Use Case |
|---------|-----------|--------|----------|
| `outline` (default) | `bg-default` | `ring-1 ring-default` | Standard content cards |
| `solid` | `bg-inverted text-inverted` | None | High-emphasis promotions |
| `soft` | `bg-elevated/50` | None | Subtle content grouping |
| `subtle` | `bg-elevated/50` | `ring-1 ring-default` | Distinguishable sections |

**Hover & Interactive States**
- Default: Subtle shadow intensification (`shadow-sm` → `shadow-md`)
- Clickable cards: `cursor-pointer hover:ring-2 hover:ring-primary/50`
- Selected state: `ring-2 ring-primary bg-primary/5`

### Inputs & Forms

**Text Input / Textarea**
```css
base: rounded-md border border-default bg-default text-default
padding: px-3 py-2 text-sm
placeholder: placeholder:text-muted
focus: focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent
disabled: disabled:opacity-50 disabled:cursor-not-allowed
error: border-error focus:ring-error
```

**Select / Dropdown**
- Same base styles as input + trailing chevron icon
- Options: `bg-default hover:bg-elevated` with clear focus indicator

**Checkbox / Radio**
- Custom styled with Nuxt Green checkmark
- Focus ring matches button focus pattern
- Label spacing: `gap-2` with aligned baseline

### Navigation

**Header Navigation**
- Layout: Flex row with logo left, links center, CTA right
- Links: `text-sm font-medium text-muted hover:text-default transition-colors`
- Active state: `text-primary font-semibold` + subtle bottom border
- Mobile: Hamburger toggle with slide-in drawer

**Sidebar Navigation** (for docs/apps)
- Collapsible sections with chevron indicators
- Active item: `bg-primary/10 text-primary font-medium rounded-md`
- Nested items: Indented with `border-l-2 border-default` guide line

### Code Blocks & Technical Content

**Code Block Container**
```css
root: rounded-lg bg-[#1e1e2e] text-[#cdd6f4] font-mono text-sm
header: flex items-center gap-2 px-4 py-2 border-b border-[#313244]
copy-button: p-1.5 rounded hover:bg-[#313244] transition
line-numbers: text-[#6c7086] select-none pr-4
```

**Inline Code**
```css
base: rounded px-1.5 py-0.5 bg-elevated text-primary font-mono text-[0.875em]
border: ring-1 ring-inset ring-primary/20
```

**Syntax Highlighting**
- Powered by Shiki with Nuxt-themed theme
- Tokens: `keyword` (purple), `string` (green), `function` (blue), `comment` (gray)

### Distinctive Components

**Feature Grid Cards**
- Icon: `size-8` rounded-lg with `bg-primary/10 text-primary`
- Title: `text-lg font-semibold` at 20px
- Description: `text-muted` at 16px with comfortable line-height
- Layout: Responsive grid (1→2→3 columns) with `gap-6`

**Module Badges**
- Pill shape: `rounded-full px-3 py-1`
- Background: `bg-primary/10 text-primary` or semantic variants
- Text: `text-xs font-semibold tracking-wide`
- Use: Module status ("New", "Official", "Community")

**Trust / Metrics Display**
- Large number: `text-4xl font-bold text-primary` with `tabular-nums`
- Label: `text-muted text-sm` below
- Container: Subtle border with `p-6` padding

---

## 5. Layout Principles

### Spacing System (Tailwind-aligned)
Base unit: **4px** with scale:
```
0, 1(4px), 2(8px), 3(12px), 4(16px), 5(20px), 6(24px), 
8(32px), 10(40px), 12(48px), 16(64px), 20(80px), 24(96px)
```

**Usage Guidelines:**
- `space-2` (8px): Icon-to-text spacing, inline element gaps
- `space-4` (16px): Component internal padding, form field spacing
- `space-6` (24px): Card padding, section internal spacing
- `space-8` (32px): Section-to-section vertical rhythm
- `space-12+` (48px+): Major layout divisions, hero spacing

### Grid & Container System
- **Max content width**: `max-w-7xl` (~1280px) centered with `mx-auto`
- **Page padding**: `px-4 sm:px-6 lg:px-8` for responsive horizontal padding
- **Section padding**: `py-12 sm:py-16 lg:py-20` for vertical rhythm
- **Grid layouts**: CSS Grid with `grid-cols-1 md:grid-cols-2 lg:grid-cols-3` patterns
- **Flex utilities**: `flex`, `items-center`, `justify-between` for header/footer patterns

### Whitespace Philosophy
- **Content breathing room**: Generous vertical spacing (`py-12+`) between major sections
- **Visual hierarchy through spacing**: Larger gaps (`space-8+`) separate conceptual groups; smaller gaps (`space-2–4`) connect related elements
- **Card isolation**: Cards use `ring-1` borders + subtle shadows instead of heavy spacing for separation
- **Code/content balance**: Technical content gets tighter spacing (`space-3`) for scannability; marketing content uses relaxed spacing (`space-6+`) for storytelling

### Border Radius Scale
| Token | Value | Use Case |
|-------|-------|----------|
| `rounded` | 0.25rem (4px) | Inputs, small interactive elements |
| `rounded-md` | 0.375rem (6px) | **Default** - buttons, cards, modals |
| `rounded-lg` | 0.5rem (8px) | Featured cards, hero containers |
| `rounded-xl` | 0.75rem (12px) | Modal dialogs, elevated surfaces |
| `rounded-2xl` | 1rem (16px) | Marketing sections, promotional blocks |
| `rounded-full` | 9999px | Badges, avatars, pill buttons |

---

## 6. Depth & Elevation

### Elevation Scale (Tailwind Shadow Tokens)
| Level | Token | CSS Value | Use Case |
|-------|-------|-----------|----------|
| 0 | `shadow-none` | None | Flat surfaces, text blocks |
| 50 | `shadow-sm` | `0 1px 2px 0 rgb(0 0 0 / 0.05)` | Subtle card borders, input focus |
| 100 | `shadow` | `0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)` | Default cards, dropdowns |
| 200 | `shadow-md` | `0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)` | Hover states, active menus |
| 300 | `shadow-lg` | `0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)` | Modals, popovers, sticky headers |
| 400 | `shadow-xl` | `0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)` | Featured overlays, critical dialogs |

**Shadow Philosophy**: Nuxt uses Tailwind's multi-layer shadow system where each layer has low individual opacity (0.05–0.1) that combines for natural, diffused depth. Shadows are **theme-aware**—dark mode shadows use lighter values (`rgba(255,255,255,0.1)`) to maintain perceived depth on dark backgrounds.

### Layering Strategy
1. **Base Layer** (`z-0`): Page background, static content
2. **Content Layer** (`z-10`): Cards, sections, interactive elements
3. **Overlay Layer** (`z-20`): Dropdowns, tooltips, popovers
4. **Modal Layer** (`z-30`): Dialogs, drawers, full-screen overlays
5. **Toast Layer** (`z-40`): Notifications, transient messages
6. **Loading Layer** (`z-50`): Full-page loaders, skeleton screens

### Decorative Depth
- **Gradient accents**: Subtle linear gradients (`bg-gradient-to-br from-primary/5 to-transparent`) for section headers
- **Blob illustrations**: Abstract, soft-edged shapes in `primary/5` opacity for visual interest without distraction
- **Code block depth**: Slightly darker background (`bg-[#1e1e2e]`) + inner ring for terminal-like appearance

---

## 7. Responsive Behavior

### Breakpoint System (Tailwind Defaults)
| Name | Min Width | Key Adjustments |
|------|-----------|----------------|
| `sm` | 640px | Horizontal padding increase, 2-column grids enable |
| `md` | 768px | Navigation expands, card grids shift to 2–3 columns |
| `lg` | 1024px | Full desktop layout, sidebar navigation visible |
| `xl` | 1280px | Max-width content container, enhanced typography scale |
| `2xl` | 1536px | Additional whitespace, optional feature enhancements |

### Mobile-First Adaptations
- **Typography**: `text-base` → `text-lg` for body on mobile for touch readability
- **Buttons**: Minimum touch target `44px` height via `py-2.5` + `text-sm`
- **Navigation**: Horizontal nav collapses to hamburger menu at `md` breakpoint
- **Cards**: Stack vertically on mobile (`grid-cols-1`), expand to grid on desktop
- **Code blocks**: Horizontal scroll enabled with `overflow-x-auto` + visual indicator

### Touch & Interaction Enhancements
- **Tap targets**: All interactive elements meet 44×44px minimum via padding
- **Hover fallbacks**: `@media (hover: hover)` queries ensure touch devices get appropriate states
- **Focus visibility**: `focus-visible` utilities provide keyboard-only focus indicators
- **Reduced motion**: `@media (prefers-reduced-motion)` support for animations

### Image & Media Behavior
- **Responsive images**: `w-full h-auto` + `object-cover` for consistent aspect ratios
- **Lazy loading**: Native `loading="lazy"` + blur-up placeholders for performance
- **Dark mode images**: `dark:invert` or separate assets for optimal contrast
- **Screenshot borders**: `ring-1 ring-default rounded-lg` for product previews

---

## 8. Accessibility & States

### Focus Management
- **Visible focus rings**: `focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary`
- **Skip links**: Hidden until focused, positioned at top of page for keyboard users
- **Logical tab order**: Components follow DOM order; modals trap focus appropriately
- **ARIA attributes**: Auto-applied via Reka UI foundation (roles, labels, states)

### Interactive State System
| State | Visual Treatment | Implementation |
|-------|-----------------|----------------|
| `default` | Base styles per variant | Component defaults |
| `hover` | Color shift + subtle scale/overlay | `hover:*` utilities |
| `active` | Darker color + `scale(0.98)` transform | `active:*` + transition |
| `focus` | Outline ring + shadow reinforcement | `focus-visible:*` |
| `disabled` | `opacity-75` + `cursor-not-allowed` | `disabled:*` utilities |
| `loading` | Spinner + reduced opacity + `pointer-events-none` | Conditional rendering |

### Color Contrast Compliance
- **Primary text on white**: `#09090b` on `#ffffff` = 18.5:1 (WCAG AAA) ✅
- **Secondary text on white**: `#71717a` on `#ffffff` = 5.8:1 (WCAG AA) ✅
- **Primary green on white**: `#00DC82` on `#ffffff` = 2.9:1 (requires large text) ⚠️
  - *Solution*: Use white text on green backgrounds for buttons (`#ffffff` on `#00DC82` = 4.7:1) ✅
- **Dark mode text**: `#f4f4f5` on `#09090b` = 17.2:1 (WCAG AAA) ✅

### Theme Awareness
- **CSS Variables**: All colors reference `--color-*` tokens that auto-adapt to light/dark
- **Prefers-color-scheme**: Auto-detection via `@nuxtjs/color-mode` with manual override
- **High contrast mode**: Semantic colors maintain distinction in Windows High Contrast
- **Reduced transparency**: Respect `prefers-reduced-transparency` for motion-sensitive users

---

## 9. Agent Prompt Guide

### Quick Color Reference (CSS Variables)
```css
/* Primary brand */
--color-primary: var(--color-green-400); /* #00DC82 */

/* Backgrounds */
--ui-bg-default: var(--color-white); /* Light */
--ui-bg-default: var(--color-zinc-900); /* Dark */
--ui-bg-elevated: var(--color-zinc-50); /* Light */
--ui-bg-elevated: var(--color-zinc-800); /* Dark */

/* Text */
--ui-text-default: var(--color-zinc-900); /* Light */
--ui-text-default: var(--color-zinc-100); /* Dark */
--ui-text-muted: var(--color-zinc-500); /* Light */
--ui-text-muted: var(--color-zinc-400); /* Dark */

/* Borders */
--ui-border-default: var(--color-zinc-200); /* Light */
--ui-border-default: var(--color-zinc-700); /* Dark */

/* Focus */
--ui-focus-ring: 0 0 0 2px var(--color-primary), 0 0 0 4px rgba(0,220,130,0.2);
```

### Example Component Prompts
- **Hero Section**:
  ```
  Create a hero section on white/dark canvas. Headline at 3.5rem (56px) Public Sans weight 700, line-height 1.0, color --ui-text-default. Subtitle at 1.25rem (20px) weight 400, line-height 1.6, color --ui-text-muted. Primary CTA button: bg-primary text-white rounded-md px-4 py-2.5 text-sm font-medium hover:bg-primary/90 transition. Secondary ghost button: text-primary hover:bg-primary/10. Layout: max-w-4xl mx-auto px-4 py-16 sm:py-24.
  ```

- **Feature Card**:
  ```
  Design a feature card: rounded-lg bg-default ring-1 ring-default overflow-hidden. Icon container: size-12 rounded-lg bg-primary/10 text-primary flex items-center justify-center mb-4. Title: text-lg font-semibold text-default mb-2. Description: text-muted text-sm leading-relaxed. Hover state: shadow-md transition-shadow. Layout: p-6 flex flex-col h-full.
  ```

- **Code Block**:
  ```
  Build a code block: rounded-lg bg-[#1e1e2e] text-[#cdd6f4] font-mono text-sm p-4 overflow-x-auto. Header: flex items-center gap-2 px-4 py-2 border-b border-[#313244] with copy button (p-1.5 rounded hover:bg-[#313244]). Line numbers: text-[#6c7086] pr-4 select-none. Syntax tokens: .keyword { color: #cba6f7 }, .string { color: #a6e3a1 }, .function { color: #89b4fa }.
  ```

- **Navigation Header**:
  ```
  Create sticky header: bg-default/80 backdrop-blur border-b border-default. Logo left (32px height). Nav links center: text-sm font-medium text-muted hover:text-default transition px-3 py-2. Active link: text-primary font-semibold. CTA button right: primary solid variant. Mobile: hamburger button at md breakpoint, slide-in drawer with same link styles.
  ```

- **Alternating Section Layout**:
  ```
  Design section alternation: odd sections bg-default, even sections bg-elevated. Each section: py-12 sm:py-16 lg:py-20 max-w-7xl mx-auto px-4. Section heading: text-2xl sm:text-3xl font-bold text-default mb-6. Content grid: grid grid-cols-1 md:grid-cols-2 gap-8 lg:gap-12.
  ```

### Iteration Guide
1. **Always use semantic colors**: Reference `primary`, `success`, `error` instead of hex values for theme flexibility
2. **Leverage CSS Variables**: Use `--ui-*` and `--color-*` tokens for all colors, spacing, and radii
3. **Respect the 4px grid**: All spacing, sizing, and positioning should align to 4px increments
4. **Dark mode first**: Design components with both themes in mind; test contrast in both modes
5. **Variant system**: Use Tailwind Variants pattern for component styling—define `base`, `variants`, `compoundVariants`
6. **Accessibility by default**: Include `focus-visible`, `aria-*`, and keyboard navigation in all interactive components
7. **Performance conscious**: Use `will-change`, `contain`, and lazy loading for complex animations/media
8. **Nuxt Green sparingly**: Use `#00DC82` for primary actions only; rely on semantic neutrals for structure

### Theming Configuration Example
```ts
// app.config.ts
export default defineAppConfig({
  ui: {
    colors: {
      primary: 'green',
      secondary: 'blue',
      neutral: 'zinc'
    }
  },
  theme: {
    colors: ['primary', 'secondary', 'success', 'info', 'warning', 'error', 'neutral']
  }
})
```

```css
/* assets/css/main.css */
@import "tailwindcss";
@import "@nuxt/ui";

@theme {
  /* Extend color palette */
  --color-brand-50: #f0fdfa;
  --color-brand-400: #00DC82; /* Nuxt Green */
  --color-brand-900: #022c22;
  
  /* Typography */
  --font-sans: 'Public Sans', system-ui, sans-serif;
  --font-mono: 'JetBrains Mono', monospace;
  
  /* Custom spacing */
  --spacing-section: 3rem; /* 48px */
}
```

---

> **Design Philosophy Summary**: Nuxt's design system prioritizes **developer experience**, **technical clarity**, and **theme flexibility**. Every decision—from the vibrant green accent to the CSS-variable-driven theming—serves developers building modern web applications. The system is intentionally opinionated about structure (Tailwind utilities, semantic colors) while remaining deeply customizable (CSS variables, runtime config). It balances aesthetic polish with performance pragmatism, ensuring beautiful interfaces that load fast and work everywhere.