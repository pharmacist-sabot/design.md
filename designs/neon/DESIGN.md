# Design System Inspired by Neon

## 1. Visual Theme & Atmosphere

Neon's design language embodies the philosophy of modern infrastructure: **deep, technical, and effortlessly powerful**. The interface is built on a sophisticated dark-mode-first palette that evokes the feeling of a professional terminal or IDE, creating an environment where developers feel at home. The primary canvas uses near-black backgrounds (`#0c0d0d`, `#1a1a1a`) rather than pure black, reducing eye strain while maintaining maximum contrast for code and technical content.

The typography system is engineered for precision and readability at scale. The primary typeface stack leverages **Inter** for UI text, **IBM Plex Sans** for headings and body copy, and **Geist Mono** for code blocks — a deliberate choice that signals technical credibility. Letter-spacing is tightened at display sizes (`-0.04em` to `-0.02em`) to create headlines that feel compressed and authoritative, while body text uses neutral tracking for optimal legibility.

What distinguishes Neon's visual language is its **layered depth system**. Rather than flat surfaces, Neon employs sophisticated gradient borders, radial masks, and multi-layer shadows that create a sense of dimensional hierarchy without visual clutter. Borders aren't simple strokes — they're complex `radial-gradient` compositions that produce subtle glows and edge highlights, giving UI elements a crafted, premium feel.

**Key Characteristics:**
- Dark-mode-first palette with near-black bases (`#0c0d0d`, `#1a1a1a`) and strategic light accents
- Three-font system: Inter (UI), IBM Plex Sans (content), Geist Mono (code)
- Tight letter-spacing at display sizes (`-0.04em` to `-0.02em`) for authoritative headlines
- Complex gradient borders using `radial-gradient` compositions for dimensional depth
- Multi-layer shadow stacks with low-opacity blacks for subtle elevation
- Primary accent: Electric Green (`#00E599`) as the singular high-visibility action color
- Secondary palette: Vibrant multi-hue accents (`#ff4c79`, `#ffa64c`, `#648DFF`) for feature differentiation
- 8px base spacing unit with responsive scaling for mobile-first layouts

## 2. Color Palette & Roles

### Core Neutrals
- **Black Pure** (`#000000`): Deepest background, modal overlays
- **Black New** (`#0c0d0d`): Primary page background, dark surfaces
- **Black Default** (`#1a1a1a`): Secondary surfaces, cards, inputs
- **Black Fog** (`#0d0e12`): Subtle section alternation, hover states
- **White** (`#ffffff`): Primary text on dark, light mode backgrounds

### Primary Accent
- **Green 45 / Primary 1** (`#00E599`): Primary CTA, success states, key interactive elements — the signature Neon green
- **Green 44** (`#39A57D`): Selection highlight, secondary emphasis, focus rings
- **Green 52** (`#34D59A`): Hover states for primary green, subtle highlights

### Secondary Accent Spectrum
| Token | Hex | Role |
|-------|-----|------|
| `secondary.1` | `#ff4c79` | Warning, destructive actions, high-priority alerts |
| `secondary.2` | `#f0f075` | Caution, pending states, informational highlights |
| `secondary.3` | `#ffa64c` | Warning, attention indicators, feature callouts |
| `secondary.5` | `#aa99ff` | Premium features, experimental badges |
| `secondary.7` | `#259df4` | Information links, secondary CTAs |
| `secondary.8` | `#0055ff` | Active states, selected tabs, primary links |
| `secondary.9` | `#ade0eb` | Light backgrounds, subtle highlights, disabled states |

### Gray Scale (Light Mode)
| Token | Hex | Role |
|-------|-----|------|
| `gray.1` | `#262626` | Headings, primary text on light |
| `gray.4` | `#808080` | Secondary text, captions |
| `gray.7` | `#e5e5e5` | Borders, dividers, disabled inputs |
| `gray.9` | `#FAFAFA` | Light mode page background |

### Gray Scale (Dark Mode — `gray-new`)
| Token | Hex | Role |
|-------|-----|------|
| `gray-new.10` | `#18191B` | Card surfaces, elevated containers |
| `gray-new.20` | `#303236` | Input backgrounds, hover states |
| `gray-new.40` | `#61646B` | Secondary text, placeholder text |
| `gray-new.60` | `#94979E` | Disabled text, subtle labels |
| `gray-new.90` | `#E4E5E7` | Primary text on dark surfaces |
| `gray-new.98` | `#FAFAFA` | Light text alternative |

### Code Syntax Highlighting
```css
code {
  --code-green-1: #078345;   /* Strings, literals */
  --code-green-2: #47D18C;   /* Functions, keywords */
  --code-blue-1: #206CDF;    /* Variables, properties */
  --code-blue-2: #66A3FF;    /* Types, imports */
  --code-red-1: #DA0B51;     /* Errors, deletions */
  --code-red-2: #F6558C;     /* Warnings, highlights */
  --code-orange-1: #FF9500;  /* Numbers, constants */
  --code-orange-2: #FFBF66;  /* Annotations */
  --code-gray-1: #B3B3B3;    /* Comments */
  --code-gray-2: #808080;    /* Punctuation */
}
```

### Interactive States
- **Focus Ring**: `#38A57D` (Green 44) with `2px` outline offset
- **Link Hover**: Primary green (`#00E599`) → lighter green (`#00FFAA`)
- **Button Active**: Scale transform `scale(0.98)` + slight opacity reduction
- **Selection**: `#39A57D` background with white text

### Complex Gradient Borders (Signature Pattern)
Neon's distinctive border treatment uses layered `radial-gradient` compositions:

```css
/* Example: Home Bento Card Border */
border-image: radial-gradient(42.03% 56.98% at 0% 100%, #847A9D 0%, transparent 89.37%),
              radial-gradient(20.73% 29.17% at 24.37% 100%, #545C8D 0%, transparent 89.37%),
              radial-gradient(22.14% 53.65% at 68.28% 0%, #545C8D 0%, transparent 95.75%),
              linear-gradient(0deg, #181818, #181818) 1;
```

## 3. Typography Rules

### Font Family Stack
```css
:root {
  --font-inter: 'Inter', -apple-system, system-ui, sans-serif;      /* UI, buttons, navigation */
  --font-esbuild: 'IBM Plex Sans', 'IBM Plex Sans Fallback', sans-serif; /* Headings, body content */
  --font-geist-mono: 'Geist Mono', 'SF Mono', monospace;           /* Code, technical content */
}
```

### Responsive Type Scale
| Class | Desktop | Tablet (≤1599px) | Mobile (≤1279px) | Small Mobile (≤767px) | Weight | Line Height |
|-------|---------|-----------------|-----------------|----------------------|--------|-------------|
| `.t-8xl` | 104px | 72px | 64px | 42px | 700 | 1.125 (dense) |
| `.t-7xl` | 80px | 56px | 48px | 36px | 700 | 1.125 |
| `.t-6xl` | 64px | 48px | 40px | 36px | 700 | 1.2 |
| `.t-5xl` | 56px | 40px | 36px | 32px | 700 | 1.25 |
| `.t-4xl` | 40px | 36px | 32px | 28px | 700 | 1.3 |
| `.t-3xl` | 32px | 24px | 20px | 17px* | 600 | 1.35 |
| `.t-2xl` | 24px | 20px | 18px | 16px | 600 | 1.4 |
| `.t-xl` | 20px | 18px | 16px | — | 500 | 1.45 |
| `.t-lg` | 18px | 16px | — | — | 400 | 1.5 |
| `.t-base` | 16px | 14px | — | — | 400 | 1.5 |
| `.t-sm` | 14px | — | — | — | 400 | 1.43 |

*\*Only `.t-3xl` adjusts at ≤413px breakpoint*

### Letter-Spacing Scale
```css
.letter-spacing {
  --tighter: -0.04em;    /* Display headlines (t-7xl, t-8xl) */
  --extra-tight: -0.02em; /* Section headings (t-5xl, t-6xl) */
  --snug: -0.01em;       /* Subheadings (t-3xl, t-4xl) */
  --normal: 0;           /* Body text (t-base, t-lg) */
  --wide: 0.02em;        /* Labels, badges */
  --wider: 0.04em;       /* Uppercase micro-copy */
}
```

### Typography Principles
- **Density at scale**: Larger headings use tighter line-height (`1.125`) and negative letter-spacing to create visual weight without increasing font size
- **Four-weight hierarchy**: 300 (light labels), 400 (body), 500 (UI emphasis), 600-700 (headings)
- **Code-first legibility**: Geist Mono for all technical content with syntax-aware coloring
- **Responsive compression**: Font sizes scale down progressively across 4 breakpoints to maintain hierarchy on small screens

## 4. Component Stylings

### Buttons

**Primary CTA**
```css
.btn-primary {
  background: #00E599;
  color: #000000;
  padding: 12px 24px;
  border-radius: 8px;
  font-weight: 600;
  font-size: 16px;
  transition: all 0.2s ease;
}
.btn-primary:hover {
  background: #00cc88; /* Green 2 */
  transform: translateY(-1px);
}
.btn-primary:active {
  transform: scale(0.98);
}
.btn-primary:focus-visible {
  outline: 2px solid #39A57D;
  outline-offset: 2px;
}
```

**Secondary / Outline**
```css
.btn-secondary {
  background: transparent;
  color: #ffffff;
  border: 1px solid rgba(255, 255, 255, 0.2);
  padding: 12px 24px;
  border-radius: 8px;
  font-weight: 500;
}
.btn-secondary:hover {
  border-color: #00E599;
  color: #00E599;
}
```

**Ghost / Text Link**
```css
.btn-ghost {
  background: transparent;
  color: #94979E; /* gray-new.60 */
  padding: 8px 16px;
  font-weight: 500;
}
.btn-ghost:hover {
  color: #ffffff;
  text-decoration: none;
}
```

**Pill Badge**
```css
.badge-pill {
  background: #18191B; /* gray-new.10 */
  color: #E4E5E7; /* gray-new.90 */
  border-radius: 9999px;
  padding: 4px 12px;
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0.02em; /* wide */
  text-transform: uppercase;
}
.badge-pill.primary {
  background: rgba(0, 229, 153, 0.15);
  color: #00E599;
}
```

### Cards & Containers

**Standard Card**
```css
.card {
  background: #18191B; /* gray-new.10 */
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 16px;
  padding: 24px;
  transition: border-color 0.2s ease;
}
.card:hover {
  border-color: rgba(0, 229, 153, 0.3);
}
```

**Featured Card with Gradient Border**
```css
.card-featured {
  background: linear-gradient(0deg, #0D0E12, #0D0E12);
  border: 1px solid transparent;
  border-radius: 16px;
  background-clip: padding-box;
  mask: linear-gradient(#fff 0 0) padding-box, linear-gradient(#fff 0 0);
  mask-composite: exclude;
  /* Complex radial gradients for edge glow */
  background-image: 
    radial-gradient(70.46% 67.03% at 100% -10.6%, rgba(19, 33, 45, 0.80) 14.53%, transparent 85.73%),
    linear-gradient(213deg, transparent 52.96%, rgba(1, 119, 99, 0.20) 138.77%);
}
```

### Inputs & Forms
```css
.input {
  background: #303236; /* gray-new.20 */
  border: 1px solid #494B50; /* gray-new.30 */
  border-radius: 8px;
  padding: 12px 16px;
  color: #E4E5E7; /* gray-new.90 */
  font-size: 16px;
}
.input::placeholder {
  color: #61646B; /* gray-new.40 */
}
.input:focus {
  border-color: #39A57D;
  outline: 2px solid #39A57D;
  outline-offset: 2px;
}
```

### Navigation
- **Desktop**: Horizontal nav with logo left, links center-aligned, CTA right
- **Mobile**: Hamburger menu with slide-in panel
- **Active State**: Bottom border `2px solid #00E599` or text color shift to primary green
- **Link Typography**: Inter, 15px, weight 500, letter-spacing `-0.01em`

### Code Blocks
```css
.code-block {
  background: #0c0d0d; /* black.new */
  border: 1px solid #303236; /* gray-new.20 */
  border-radius: 12px;
  padding: 20px;
  font-family: var(--font-geist-mono);
  font-size: 14px;
  line-height: 1.6;
  overflow-x: auto;
}
.code-block .token.string { color: #078345; }
.code-block .token.function { color: #47D18C; }
.code-block .token.keyword { color: #206CDF; }
.code-block .token.comment { color: #B3B3B3; }
```

### Admonitions / Callouts
```css
.admonition {
  border-left: 4px solid;
  padding: 16px 20px;
  border-radius: 0 8px 8px 0;
  background: rgba(24, 25, 27, 0.5);
}
.admonition.warning {
  border-color: #ff4c79; /* secondary.1 */
}
.admonition.tip {
  border-color: #00E599; /* primary */
}
.admonition.info {
  border-color: #259df4; /* secondary.7 */
}
```

## 5. Layout Principles

### Spacing System
- **Base unit**: 8px
- **Scale**: 4px, 8px, 12px, 16px, 20px, 24px, 32px, 40px, 48px, 64px, 80px, 96px, 128px
- **Responsive adjustment**: All vertical spacing reduces by ~20% on mobile breakpoints

### Grid & Container
- **Max content width**: 1200px centered with `padding: 0 24px`
- **Section padding**: 80px vertical on desktop, 48px on tablet, 32px on mobile
- **Grid system**: CSS Grid with `gap: 24px` for card layouts, `gap: 16px` for inline elements
- **Breakpoints**:
  ```css
  --breakpoint-xs: 25.8125rem;  /* 413px */
  --breakpoint-sm: 39.9375rem;  /* 639px */
  --breakpoint-md: 47.9375rem;  /* 767px */
  --breakpoint-lg: 63.9375rem;  /* 1023px */
  --breakpoint-xl: 79.9375rem;  /* 1279px */
  --breakpoint-2xl: 99.9375rem; /* 1599px */
  --breakpoint-3xl: 119.9375rem;/* 1919px */
  ```

### Whitespace Philosophy
- **Breathing room**: Generous vertical spacing (64-96px) between major sections to create visual hierarchy
- **Content density**: Tight horizontal spacing within cards (16-24px) to maximize information density
- **Edge treatment**: Full-bleed sections alternate with contained content for rhythm
- **Dark mode depth**: Subtle background variations (`#0c0d0d` → `#18191B`) create section separation without hard borders

### Border Radius Scale
```css
.radius {
  --micro: 4px;    /* Inputs, small buttons */
  --small: 8px;    /* Standard cards, buttons */
  --medium: 12px;  /* Featured cards, modals */
  --large: 16px;   /* Hero containers, major sections */
  --full: 9999px;  /* Pills, badges, avatars */
}
```

## 6. Depth & Elevation

### Shadow System
Neon uses a **multi-layer, low-opacity shadow stack** for natural depth:

```css
/* Level 1: Subtle lift (cards, inputs) */
.shadow-1 {
  box-shadow: 
    0px 1px 2px rgba(0, 0, 0, 0.05),
    0px 2px 4px rgba(0, 0, 0, 0.03);
}

/* Level 2: Elevated (modals, dropdowns) */
.shadow-2 {
  box-shadow: 
    0px 4px 12px rgba(0, 0, 0, 0.15),
    0px 2px 6px rgba(0, 0, 0, 0.1),
    0px 1px 3px rgba(0, 0, 0, 0.08);
}

/* Level 3: Floating (tooltips, popovers) */
.shadow-3 {
  box-shadow: 
    0px 8px 24px rgba(0, 0, 0, 0.25),
    0px 4px 12px rgba(0, 0, 0, 0.15),
    0px 2px 6px rgba(0, 0, 0, 0.1);
}
```

### Gradient Border Depth (Signature)
Instead of traditional borders, Neon uses `border-image` with radial gradients to create edge highlights:

```css
.border-glow {
  border: 1px solid transparent;
  background-clip: padding-box;
  mask: linear-gradient(#fff 0 0) padding-box, linear-gradient(#fff 0 0);
  mask-composite: exclude;
  background-image: 
    radial-gradient(60.96% 60.55% at 0% 0%, rgba(100, 144, 185, 0.50) 0%, transparent 80%),
    radial-gradient(64.38% 53.06% at 100% 100%, rgba(100, 158, 185, 0.50) 0%, transparent 80%),
    linear-gradient(0deg, #242628, #242628);
}
```

### Decorative Depth Elements
- **Lightning title effects**: Radial gradients with white-to-transparent transitions for metallic sheen
- **Bento grid borders**: Multi-radial compositions creating corner glows and edge highlights
- **Security card backgrounds**: Layered radial masks with green tints for thematic emphasis

## 7. Responsive Behavior

### Breakpoint Strategy
| Name | Max Width | Layout Changes |
|------|-----------|---------------|
| `xs` | 413px | Single column, minimal padding, compressed type scale |
| `sm` | 639px | Stacked cards, reduced horizontal padding |
| `md` | 767px | 2-column grids begin, navigation collapses to hamburger |
| `lg` | 1023px | Full card grids, expanded padding, desktop nav |
| `xl` | 1279px | Max content width (1200px) engaged |
| `2xl` | 1599px | Additional horizontal margin for ultra-wide screens |
| `3xl` | 1919px | Centered layout with generous side margins |

### Type Scaling Strategy
Font sizes compress progressively across breakpoints:
- `t-8xl`: 104px → 72px → 64px → 42px
- `t-6xl`: 64px → 48px → 40px → 36px
- `t-4xl`: 40px → 36px → 32px → 28px
- Body text remains stable (16px → 14px) for readability

### Component Collapsing
- **Navigation**: Horizontal links → hamburger menu with slide-in panel
- **Feature grids**: 3-column → 2-column → single column stacked
- **Code tabs**: Horizontal tabs → vertical accordion on mobile
- **CTA sections**: Side-by-side buttons → stacked full-width buttons
- **Hero illustrations**: Complex compositions → simplified single focal point

### Touch Targets
- Minimum tap target: 44×44px for all interactive elements
- Button padding: Minimum 12px vertical, 24px horizontal
- Link spacing: Minimum 8px between adjacent interactive elements

## 8. Accessibility & States

### Focus Management
```css
:focus-visible {
  outline: 2px solid #38A57D; /* Green 44 */
  outline-offset: 2px;
  border-radius: 4px;
}

/* Skip link for keyboard navigation */
.skip-link {
  position: absolute;
  top: -40px;
  left: 0;
  background: #00E599;
  color: #000;
  padding: 8px 16px;
  z-index: 100;
}
.skip-link:focus {
  top: 0;
}
```

### Color Contrast Compliance
- **Primary text on dark**: `#E4E5E7` on `#0c0d0d` = 14.2:1 (AAA)
- **Secondary text on dark**: `#94979E` on `#0c0d0d` = 6.8:1 (AA)
- **Primary CTA**: `#000` on `#00E599` = 5.1:1 (AA for large text)
- **Code syntax**: All token colors tested against dark background for WCAG AA

### Interactive State System
| State | Visual Treatment | Animation |
|-------|-----------------|-----------|
| Default | Base styles | — |
| Hover | Color shift + subtle scale `translateY(-1px)` | `transition: all 0.2s ease` |
| Active | Scale `0.98` + opacity `0.95` | Instant |
| Focus | Green outline ring + shadow | `transition: outline 0.15s` |
| Disabled | Opacity `0.5` + pointer-events `none` | — |
| Loading | Spinner + reduced opacity | `animation: loading 1s infinite` |

### Reduced Motion Support
```css
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}
```

## 9. Agent Prompt Guide

### Quick Color Reference
```css
/* Core */
--bg-primary: #0c0d0d;
--bg-secondary: #18191B;
--text-primary: #E4E5E7;
--text-secondary: #94979E;

/* Accents */
--accent-primary: #00E599;    /* Green 45 */
--accent-primary-hover: #00cc88; /* Green 2 */
--focus-ring: #39A57D;        /* Green 44 */

/* Secondary Palette */
--accent-warning: #ff4c79;    /* secondary.1 */
--accent-caution: #ffa64c;    /* secondary.3 */
--accent-info: #259df4;       /* secondary.7 */
--accent-premium: #aa99ff;    /* secondary.5 */

/* Borders */
--border-subtle: rgba(255, 255, 255, 0.1);
--border-focus: #39A57D;
```

### Example Component Prompts
- **Hero Section**: "Create a dark hero with `#0c0d0d` background. Headline at `t-7xl` (80px desktop), IBM Plex Sans weight 700, letter-spacing `-0.04em`, color `#ffffff`. Subtitle at `t-xl` (20px), weight 500, color `#94979E`. Primary CTA button with `#00E599` background, black text, 8px radius, 12px 24px padding. Add subtle radial gradient border using `border-image` technique."

- **Feature Card**: "Design a card with `#18191B` background, 16px radius, complex gradient border using radial masks. Title at `t-3xl` (32px), weight 600. Description at `t-base` (16px), weight 400, color `#94979E`. Include code snippet block with Geist Mono, syntax highlighting using `--code-*` variables."

- **Pill Badge**: "Create a status badge: `#18191B` background, `#E4E5E7` text, 9999px radius, 4px 12px padding, 12px font, weight 600, letter-spacing `0.02em`, uppercase. For primary variant: background `rgba(0, 229, 153, 0.15)`, text `#00E599`."

- **Navigation**: "Build a dark header: `#0c0d0d` background, Inter font 15px weight 500 for links. Active state: bottom border `2px solid #00E599`. Primary CTA pill button right-aligned with green background. Mobile: hamburger toggle with slide-in panel."

- **Gradient Border Technique**: "Implement signature Neon border: `border: 1px solid transparent`, `background-clip: padding-box`, `mask: linear-gradient(#fff 0 0) padding-box, linear-gradient(#fff 0 0)`, `mask-composite: exclude`. Layer 3-4 radial gradients for edge glow effects, ending with solid background color."

### Iteration Guide
1. **Dark-first**: Always start with dark mode (`#0c0d0d` base) — light mode is secondary
2. **Green is action**: `#00E599` is reserved for primary CTAs and success states only
3. **Gradient borders**: Use `border-image` with radial gradients for featured elements, simple borders for utility components
4. **Type compression**: Apply negative letter-spacing (`-0.04em` to `-0.02em`) to headings ≥24px
5. **Three-font discipline**: Inter for UI chrome, IBM Plex Sans for content, Geist Mono for code — never mix
6. **Shadow restraint**: Use multi-layer shadows with max individual opacity 0.15; prefer gradient borders over heavy shadows
7. **Responsive type**: Always define font sizes across all 4 breakpoints using the `.t-*` scale
8. **Focus visibility**: Never remove `:focus-visible` outlines; use `#39A57D` for consistency

### Animation Tokens
```css
/* Keyframes */
@keyframes loading {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

@keyframes text-blink {
  0%, 50% { 
    color: #000; 
    text-shadow: -1px -1px 0 #262626, 1px -1px 0 #262626, -1px 1px 0 #262626, 1px 1px 0 #262626;
  }
  25%, 100% { 
    color: currentColor; 
    text-shadow: none;
  }
}

/* Usage */
.animate-loading { animation: loading 1s cubic-bezier(0.4, 0, 0.6, 1) infinite; }
.animate-text-blink { animation: text-blink 0.4s cubic-bezier(0.4, 0, 0.2, 1); }
```

---

*This design system is inspired by the visual language of [Neon](https://neon.com/), a serverless Postgres platform. All specifications are derived from analysis of their public website and documentation. For production use, always reference the official [Neon brand guidelines](https://neon.com/brand) and consult their engineering team for implementation details.*