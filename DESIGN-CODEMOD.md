# Design System Inspired by Codemod

## 1. Visual Theme & Atmosphere

Codemod's design embodies the philosophy of a modern developer tool: precision, clarity, and technical confidence. The visual language is built on a **dual-theme foundation** (light/dark) with a focus on readability for code-heavy interfaces and a professional, enterprise-ready aesthetic. The system avoids decorative flourishes in favor of functional minimalism—every pixel serves a purpose.

**Light Mode Foundation:**
- Canvas: Pure white (`#ffffff`) for primary surfaces
- Text: Near-black (`#0a0a0a` / `rgba(10, 10, 10, 0.95)`) for primary content, never pure black to reduce eye strain
- Secondary text: Cool gray (`#525252`) for descriptions and metadata
- Borders: Subtle `1px solid rgba(0, 0, 0, 0.08)` for structure without visual weight

**Dark Mode Foundation:**
- Canvas: Deep charcoal (`#0d1117`) inspired by GitHub's dark theme, providing optimal contrast for syntax highlighting
- Surface: Slightly lighter (`#161b22`) for cards and interactive elements
- Text: Off-white (`#e6edf3`) for primary content, with `#8b949e` for secondary text
- Borders: `1px solid rgba(255, 255, 255, 0.12)` for subtle division

**Core Philosophy:**
- **Code-first legibility**: Typography and spacing prioritize readability of code snippets, diffs, and technical documentation
- **Functional hierarchy**: Visual weight communicates importance, not decoration—bold headings, clear CTAs, muted secondary actions
- **Technical authenticity**: Syntax highlighting, monospace fonts, and diff viewers feel native, not bolted-on
- **Enterprise polish**: Subtle animations, consistent spacing, and refined interactions convey reliability at scale

**Key Characteristics:**
- **Inter** as primary UI font (with `JetBrains Mono` / `Fira Code` for code)
- **Dual-theme support** with seamless toggle and persistent preference
- **Syntax-aware components**: Code blocks, diffs, and terminals use Shiki-powered highlighting
- **GitHub-inspired diff UI**: Side-by-side comparisons with inline change indicators
- **Technical accent color**: `#0066ff` (Codemod Blue) for primary actions, links, and interactive states
- **Status semantics**: Green (`#2ea44f`), Yellow (`#d29922`), Red (`#f85149`) for build/test/deployment states
- **8px baseline grid** with rational scale for predictable, maintainable layouts

---

## 2. Color Palette & Roles

### Core Neutrals (Light Mode)
| Token | Value | Usage |
|-------|-------|-------|
| `--color-canvas-default` | `#ffffff` | Page background, card surfaces |
| `--color-canvas-subtle` | `#f6f8fa` | Section alternation, hover states |
| `--color-fg-default` | `#0a0a0a` | Primary text, headings |
| `--color-fg-muted` | `#525252` | Secondary text, descriptions |
| `--color-fg-subtle` | `#8b949e` | Placeholder, disabled, captions |
| `--color-border-default` | `rgba(0, 0, 0, 0.08)` | Card borders, dividers, inputs |
| `--color-border-muted` | `rgba(0, 0, 0, 0.04)` | Subtle separators, hover states |

### Core Neutrals (Dark Mode)
| Token | Value | Usage |
|-------|-------|-------|
| `--color-canvas-default` | `#0d1117` | Page background |
| `--color-canvas-subtle` | `#161b22` | Card surfaces, hover states |
| `--color-canvas-inset` | `#010409` | Inset elements, code blocks |
| `--color-fg-default` | `#e6edf3` | Primary text |
| `--color-fg-muted` | `#8b949e` | Secondary text |
| `--color-fg-subtle` | `#6e7681` | Placeholder, disabled |
| `--color-border-default` | `rgba(255, 255, 255, 0.12)` | Borders, dividers |
| `--color-border-muted` | `rgba(255, 255, 255, 0.06)` | Subtle separators |

### Primary & Interactive
| Token | Value | Usage |
|-------|-------|-------|
| `--color-accent-primary` | `#0066ff` | Primary CTAs, active links, focus rings |
| `--color-accent-primary-hover` | `#0055cc` | Hover state for primary actions |
| `--color-accent-primary-active` | `#0044aa` | Pressed/active state |
| `--color-accent-secondary` | `#6e84b3` | Secondary buttons, less prominent actions |
| `--color-focus-ring` | `rgba(0, 102, 255, 0.4)` | Keyboard focus outline (with blur) |

### Semantic Status Colors
| Token | Light Value | Dark Value | Usage |
|-------|------------|------------|-------|
| `--color-success` | `#2ea44f` | `#3fb950` | Success states, passing tests, completed migrations |
| `--color-warning` | `#d29922` | `#d29922` | Warnings, pending actions, attention needed |
| `--color-danger` | `#f85149` | `#f85149` | Errors, failures, destructive actions |
| `--color-info` | `#0969da` | `#58a6ff` | Informational messages, neutral updates |

### Syntax Highlighting (Shiki-based)
```css
/* Light Mode Tokens */
--color-syntax-comment: #6a737d;
--color-syntax-keyword: #d73a49;
--color-syntax-string: #032f62;
--color-syntax-function: #6f42c1;
--color-syntax-variable: #e36209;
--color-syntax-number: #005cc5;
--color-syntax-operator: #d73a49;
--color-syntax-tag: #22863a;
--color-syntax-attr: #005cc5;

/* Dark Mode Tokens */
--color-syntax-comment: #8b949e;
--color-syntax-keyword: #ff7b72;
--color-syntax-string: #a5d6ff;
--color-syntax-function: #d2a8ff;
--color-syntax-variable: #ffa657;
--color-syntax-number: #79c0ff;
--color-syntax-operator: #ff7b72;
--color-syntax-tag: #7ee787;
--color-syntax-attr: #79c0ff;
```

### Diff & Change Indicators
| Token | Light Value | Dark Value | Usage |
|-------|------------|------------|-------|
| `--color-diff-add-bg` | `#e6ffed` | `#1f2428` | Background for added lines |
| `--color-diff-add-text` | `#22863a` | `#3fb950` | Text color for additions |
| `--color-diff-remove-bg` | `#ffebe9` | `#2c1c20` | Background for removed lines |
| `--color-diff-remove-text` | `#cf222e` | `#f85149` | Text color for deletions |
| `--color-diff-change-bg` | `#ddf4ff` | `#1f2428` | Background for modified lines |
| `--color-diff-gutter-add` | `#2ea44f` | `#3fb950` | Gutter indicator for additions |
| `--color-diff-gutter-remove` | `#f85149` | `#f85149` | Gutter indicator for deletions |

### Shadows & Depth
```css
/* Light Mode */
--shadow-level-100: 0 1px 3px rgba(0, 0, 0, 0.04), 0 1px 2px rgba(0, 0, 0, 0.02);
--shadow-level-200: 0 4px 12px rgba(0, 0, 0, 0.08), 0 2px 4px rgba(0, 0, 0, 0.04);
--shadow-level-300: 0 8px 24px rgba(0, 0, 0, 0.12), 0 4px 8px rgba(0, 0, 0, 0.06);

/* Dark Mode */
--shadow-level-100: 0 1px 3px rgba(0, 0, 0, 0.2), 0 1px 2px rgba(0, 0, 0, 0.15);
--shadow-level-200: 0 4px 12px rgba(0, 0, 0, 0.3), 0 2px 4px rgba(0, 0, 0, 0.2);
--shadow-level-300: 0 8px 24px rgba(0, 0, 0, 0.4), 0 4px 8px rgba(0, 0, 0, 0.3);
```

---

## 3. Typography Rules

### Font Families
```css
/* UI Font Stack */
--font-family-ui: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', 
  'Helvetica Neue', Arial, sans-serif;

/* Code Font Stack */
--font-family-code: 'JetBrains Mono', 'Fira Code', 'SF Mono', 
  'Menlo', 'Consolas', 'Liberation Mono', monospace;

/* Fallback for syntax highlighting */
--font-family-mono: ui-monospace, SFMono-Regular, SF Mono, 
  Menlo, Consolas, Liberation Mono, monospace;
```

### Type Scale & Hierarchy

| Role | Font | Size (rem) | Weight | Line Height | Letter Spacing | Usage |
|------|------|-----------|--------|-------------|----------------|-------|
| Display Hero | Inter | 3.5rem (56px) | 700 | 1.1 | -0.02em | Main page headlines, campaign titles |
| Section Heading | Inter | 2rem (32px) | 700 | 1.2 | -0.01em | Feature sections, dashboard headers |
| Card Title | Inter | 1.25rem (20px) | 600 | 1.3 | 0 | Feature cards, repository names |
| Body Large | Inter | 1.125rem (18px) | 400 | 1.6 | 0 | Introductions, documentation lead text |
| Body | Inter | 1rem (16px) | 400 | 1.5 | 0 | Standard reading text, UI copy |
| Body Small | Inter | 0.875rem (14px) | 400 | 1.4 | 0 | Metadata, timestamps, secondary info |
| Code Block | JetBrains Mono | 0.875rem (14px) | 400 | 1.6 | 0 | Inline code, terminal output, diffs |
| Terminal | JetBrains Mono | 0.8125rem (13px) | 400 | 1.5 | 0 | CLI output, command examples |
| Button / Nav | Inter | 0.875rem (14px) | 500 | 1.2 | 0 | Navigation, button text, tabs |
| Badge / Label | Inter | 0.75rem (12px) | 600 | 1.3 | 0.02em | Status badges, version tags, filters |
| Caption | Inter | 0.75rem (12px) | 400 | 1.3 | 0 | Helper text, form labels, footnotes |

### Typography Principles
- **Readable at scale**: Body text uses 16px minimum with 1.5 line-height for comfortable reading of technical content
- **Code parity**: Monospace fonts match common IDE defaults (JetBrains Mono, Fira Code) for seamless context switching
- **Weight hierarchy**: 400 (body), 500 (UI/interactive), 600 (emphasis), 700 (headings) — clear visual scanning
- **Tight headings**: Display text uses slightly negative letter-spacing (-0.02em) for compact, impactful headlines
- **Accessible contrast**: All text meets WCAG AA minimum (4.5:1 for normal text, 3:1 for large text)

### OpenType Features (Enabled on headings & code)
```css
font-feature-settings: 
  "tnum" on,    /* Tabular numbers for aligned metrics */
  "lnum" on,    /* Lining numerals for consistency */
  "zero" on,    /* Slashed zero in code fonts */
  "ss01" on;    /* Alternative glyphs for better code readability */
```

---

## 4. Component Stylings

### Buttons

**Primary (Codemod Blue)**
```css
background: var(--color-accent-primary);
color: #ffffff;
padding: 0.5rem 1rem; /* 8px 16px */
border-radius: 6px;
border: 1px solid transparent;
font-weight: 500;
transition: background 0.15s ease, transform 0.05s ease;

&:hover {
  background: var(--color-accent-primary-hover);
}
&:active {
  transform: scale(0.98);
  background: var(--color-accent-primary-active);
}
&:focus-visible {
  outline: 2px solid var(--color-focus-ring);
  outline-offset: 2px;
  box-shadow: var(--shadow-level-200);
}
```

**Secondary**
```css
background: var(--color-canvas-subtle);
color: var(--color-fg-default);
border: 1px solid var(--color-border-default);
/* Same padding/radius as primary */
&:hover {
  background: var(--color-border-muted);
}
```

**Ghost / Text Button**
```css
background: transparent;
color: var(--color-accent-primary);
padding: 0.25rem 0.5rem;
border-radius: 4px;
&:hover {
  background: var(--color-canvas-subtle);
  text-decoration: none;
}
```

**Danger / Destructive**
```css
background: var(--color-danger);
color: #ffffff;
/* Same structure as primary */
&:hover {
  filter: brightness(0.95);
}
```

**Pill Badge (Status Indicators)**
```css
background: var(--color-canvas-subtle);
color: var(--color-fg-default);
padding: 0.25rem 0.5rem; /* 4px 8px */
border-radius: 9999px;
font-size: 0.75rem;
font-weight: 600;
line-height: 1.3;
display: inline-flex;
align-items: center;
gap: 0.25rem;

/* Semantic variants */
&[data-status="success"] {
  background: rgba(46, 164, 79, 0.15);
  color: var(--color-success);
}
&[data-status="warning"] {
  background: rgba(210, 153, 34, 0.15);
  color: var(--color-warning);
}
&[data-status="danger"] {
  background: rgba(248, 81, 73, 0.15);
  color: var(--color-danger);
}
```

### Code Blocks & Diffs

**Code Block Container**
```css
background: var(--color-canvas-inset);
border: 1px solid var(--color-border-default);
border-radius: 8px;
padding: 1rem;
font-family: var(--font-family-code);
font-size: 0.875rem;
line-height: 1.6;
overflow-x: auto;

/* Optional header */
&::before {
  content: attr(data-language);
  display: block;
  font-size: 0.75rem;
  color: var(--color-fg-muted);
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--color-border-muted);
  margin-bottom: 0.5rem;
}
```

**Diff Viewer (GitHub-style)**
```css
.diff-container {
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  overflow: hidden;
  font-family: var(--font-family-code);
  font-size: 0.8125rem;
}

.diff-line {
  display: flex;
  padding: 0 1rem;
  &.added {
    background: var(--color-diff-add-bg);
    color: var(--color-diff-add-text);
  }
  &.removed {
    background: var(--color-diff-remove-bg);
    color: var(--color-diff-remove-text);
  }
  &.changed {
    background: var(--color-diff-change-bg);
  }
}

.diff-gutter {
  width: 3rem;
  text-align: right;
  padding-right: 0.75rem;
  color: var(--color-fg-muted);
  user-select: none;
  &.added::after { content: "+"; color: var(--color-diff-gutter-add); }
  &.removed::after { content: "−"; color: var(--color-diff-gutter-remove); }
}

.diff-content {
  flex: 1;
  white-space: pre;
  tab-size: 2;
}
```

### Cards & Containers
```css
.card {
  background: var(--color-canvas-default);
  border: 1px solid var(--color-border-default);
  border-radius: 12px;
  padding: 1.25rem;
  transition: box-shadow 0.2s ease, border-color 0.2s ease;
  
  &:hover {
    border-color: var(--color-border-muted);
    box-shadow: var(--shadow-level-200);
  }
  
  /* Featured / Hero variant */
  &.card--featured {
    border-radius: 16px;
    padding: 1.5rem;
    box-shadow: var(--shadow-level-300);
  }
}
```

### Inputs & Forms
```css
.input {
  background: var(--color-canvas-default);
  border: 1px solid var(--color-border-default);
  border-radius: 6px;
  padding: 0.5rem 0.75rem;
  font-size: 1rem;
  color: var(--color-fg-default);
  transition: border-color 0.15s ease, box-shadow 0.15s ease;
  
  &::placeholder {
    color: var(--color-fg-subtle);
  }
  
  &:focus {
    outline: none;
    border-color: var(--color-accent-primary);
    box-shadow: 0 0 0 3px var(--color-focus-ring);
  }
  
  &:disabled {
    background: var(--color-canvas-subtle);
    color: var(--color-fg-muted);
    cursor: not-allowed;
  }
}
```

### Navigation
```css
.nav {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  padding: 1rem 0;
  
  &__link {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--color-fg-default);
    text-decoration: none;
    padding: 0.25rem 0;
    border-bottom: 2px solid transparent;
    transition: color 0.15s ease, border-color 0.15s ease;
    
    &:hover {
      color: var(--color-accent-primary);
    }
    
    &[aria-current="page"] {
      color: var(--color-accent-primary);
      border-bottom-color: var(--color-accent-primary);
    }
  }
  
  &__cta {
    margin-left: auto;
  }
}

/* Mobile */
@media (max-width: 768px) {
  .nav {
    flex-wrap: wrap;
    &__links {
      display: none; /* Hamburger toggle */
    }
  }
}
```

### Terminal / CLI Output
```css
.terminal {
  background: var(--color-canvas-inset);
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  padding: 1rem;
  font-family: var(--font-family-code);
  font-size: 0.8125rem;
  line-height: 1.5;
  color: var(--color-fg-default);
  
  &__prompt {
    color: var(--color-success);
    &::after {
      content: "$";
      margin-right: 0.5rem;
    }
  }
  
  &__command {
    color: var(--color-fg-default);
  }
  
  &__output {
    color: var(--color-fg-muted);
    margin-left: 1.5rem;
  }
  
  &__error {
    color: var(--color-danger);
  }
}
```

### Repository / Package Cards
```css
.package-card {
  display: grid;
  grid-template-columns: 1fr auto;
  gap: 1rem;
  padding: 1rem;
  border: 1px solid var(--color-border-default);
  border-radius: 12px;
  background: var(--color-canvas-default);
  
  &:hover {
    border-color: var(--color-accent-primary);
    box-shadow: var(--shadow-level-200);
  }
  
  &__meta {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  
  &__name {
    font-size: 1rem;
    font-weight: 600;
    color: var(--color-fg-default);
  }
  
  &__description {
    font-size: 0.875rem;
    color: var(--color-fg-muted);
  }
  
  &__actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
}
```

---

## 5. Layout Principles

### Spacing System
- **Base unit**: 4px (for fine-grained control in code-heavy UIs)
- **Scale**: 4, 8, 12, 16, 20, 24, 32, 40, 48, 64, 80, 96, 128px
- **CSS Variables**:
```css
--space-1: 0.25rem;  /* 4px */
--space-2: 0.5rem;   /* 8px */
--space-3: 0.75rem;  /* 12px */
--space-4: 1rem;     /* 16px */
--space-5: 1.25rem;  /* 20px */
--space-6: 1.5rem;   /* 24px */
--space-8: 2rem;     /* 32px */
--space-10: 2.5rem;  /* 40px */
--space-12: 3rem;    /* 48px */
--space-16: 4rem;    /* 64px */
--space-20: 5rem;    /* 80px */
--space-24: 6rem;    /* 96px */
--space-32: 8rem;    /* 128px */
```

### Grid & Container
- **Max content width**: 1200px for documentation, 1440px for dashboards
- **Hero sections**: Centered, single-column, generous vertical padding (64–96px)
- **Feature grids**: 2–3 column responsive grids for cards, repositories, or metrics
- **Code-heavy sections**: Full-width containers with horizontal padding only (no max-width constraint for wide diffs)
- **Sidebar layouts**: 240px fixed sidebar for navigation, collapsible on mobile

### Whitespace Philosophy
- **Content density control**: Tight spacing for code blocks (1.5 line-height), generous margins for prose (1.6–1.8)
- **Section separation**: 64–96px vertical padding between major sections; 32–48px for subsections
- **Visual breathing room**: Cards and interactive elements have internal padding of 16–24px minimum
- **Code context**: Inline code has `0.2em` horizontal padding; blocks have `1rem` padding

### Border Radius Scale
| Token | Value | Usage |
|-------|-------|-------|
| `--radius-none` | 0 | Dividers, full-bleed elements |
| `--radius-small` | 4px | Buttons, inputs, small interactive elements |
| `--radius-medium` | 8px | Cards, modals, code blocks |
| `--radius-large` | 12px | Featured cards, repository cards |
| `--radius-xlarge` | 16px | Hero cards, promotional sections |
| `--radius-full` | 9999px | Pills, badges, avatars |
| `--radius-circle` | 50% | User avatars, status indicators |

---

## 6. Depth & Elevation

| Level | CSS Variable | Usage |
|-------|-------------|-------|
| `--elevation-0` | `none` | Base canvas, text blocks |
| `--elevation-100` | `var(--shadow-level-100)` | Hover states, dropdowns, small modals |
| `--elevation-200` | `var(--shadow-level-200)` | Cards, popovers, medium modals |
| `--elevation-300` | `var(--shadow-level-300)` | Featured cards, large modals, floating panels |
| `--elevation-focus` | `0 0 0 3px var(--color-focus-ring)` | Keyboard focus indicator |

### Shadow Philosophy
Codemod's elevation system uses **layered, low-opacity shadows** to create natural depth without visual noise:
- Each shadow level combines 2–3 layers with cumulative opacity never exceeding 0.12
- Blur radii scale with elevation (2px → 4px → 8px) for soft, ambient occlusion
- Dark mode shadows use higher opacity values to maintain contrast against dark backgrounds
- Focus rings use a solid color with blur (`box-shadow: 0 0 0 3px rgba(0, 102, 255, 0.4)`) for clear, accessible indication

### Decorative Elements
- **Subtle gradients**: Used sparingly for section headers or hero backgrounds (e.g., `linear-gradient(135deg, #0066ff10, transparent)`)
- **Animated indicators**: Pulsing dots for "processing" states, smooth transitions for state changes
- **No decorative illustrations**: Visual interest comes from code syntax, data visualizations, and clean typography

---

## 7. Responsive Behavior

### Breakpoints
| Name | Width | Key Adjustments |
|------|-------|----------------|
| `xs` | < 480px | Single-column layout, condensed navigation, touch-optimized targets |
| `sm` | 480–767px | Stacked cards, simplified diffs, mobile-first navigation |
| `md` | 768–1023px | 2-column grids, expanded sidebar, full diff viewers |
| `lg` | 1024–1279px | 3-column layouts, multi-pane dashboards, side-by-side code editors |
| `xl` | ≥ 1280px | Maximum content width, advanced layout options, multi-repo views |

### Touch Targets & Accessibility
- **Minimum tap target**: 44×44px for all interactive elements
- **Focus indicators**: Visible `2px` outline with `3px` blur ring for keyboard navigation
- **Reduced motion**: Respect `prefers-reduced-motion` for animations and transitions
- **High contrast mode**: Ensure semantic colors maintain contrast in Windows High Contrast mode

### Collapsing Strategy
- **Navigation**: Horizontal links → hamburger menu with slide-in panel
- **Code diffs**: Side-by-side → unified view on narrow screens
- **Dashboards**: Multi-widget grid → stacked, scrollable layout
- **Repository lists**: Grid → list view with condensed metadata
- **Terminal output**: Horizontal scroll preserved; font size scales to 0.75rem on mobile

### Image & Media Behavior
- **Syntax screenshots**: Maintain aspect ratio with `max-width: 100%` and whisper borders
- **Animated demos**: Lazy-loaded with `loading="lazy"` and reduced-motion fallbacks
- **Avatars / logos**: Circular crop with `object-fit: cover` for consistency

---

## 8. Accessibility & States

### Focus Management
```css
/* Global focus style */
*:focus-visible {
  outline: 2px solid var(--color-focus-ring);
  outline-offset: 2px;
  box-shadow: var(--shadow-level-200);
}

/* Skip link for keyboard users */
.skip-link {
  position: absolute;
  top: -40px;
  left: 0;
  background: var(--color-accent-primary);
  color: white;
  padding: 0.5rem 1rem;
  z-index: 1000;
  transition: top 0.2s ease;
  
  &:focus {
    top: 0;
  }
}
```

### Interactive States
| State | Visual Treatment | Example Use |
|-------|-----------------|-------------|
| `default` | Standard appearance | Initial render |
| `hover` | Background tint + subtle scale (1.02) | Button, link, card |
| `active` | Scale(0.98) + darker background | Button press, menu select |
| `focus` | Blue ring + shadow | Keyboard navigation |
| `disabled` | Opacity 0.5 + `cursor: not-allowed` | Unavailable actions |
| `loading` | Spinner + reduced opacity | Async operations |
| `error` | Red border + icon + helper text | Form validation |

### Color Contrast Compliance
- **Primary text** (`#0a0a0a` / `#e6edf3`) on canvas: > 18:1 (WCAG AAA)
- **Secondary text** (`#525252` / `#8b949e`) on canvas: > 7:1 (WCAG AAA)
- **Accent on canvas** (`#0066ff` on white/dark): > 4.5:1 (WCAG AA for normal text)
- **Status colors**: All semantic colors tested against both light/dark canvases for AA compliance

### ARIA & Semantic HTML
- All interactive elements use appropriate `role`, `aria-*`, and keyboard handlers
- Code blocks use `<pre><code>` with `role="region"` and `aria-label`
- Diff viewers include `aria-live="polite"` for dynamic updates
- Navigation uses `<nav>` with `aria-label`; skip links for main content

---

## 9. Agent Prompt Guide

### Quick Color Reference
```css
/* Core */
--color-canvas-default: #ffffff / #0d1117;
--color-fg-default: #0a0a0a / #e6edf3;
--color-border-default: rgba(0,0,0,0.08) / rgba(255,255,255,0.12);

/* Interactive */
--color-accent-primary: #0066ff;
--color-accent-primary-hover: #0055cc;
--color-focus-ring: rgba(0, 102, 255, 0.4);

/* Semantic */
--color-success: #2ea44f / #3fb950;
--color-warning: #d29922;
--color-danger: #f85149;

/* Code */
--color-syntax-keyword: #d73a49 / #ff7b72;
--color-syntax-string: #032f62 / #a5d6ff;
--color-diff-add-bg: #e6ffed / #1f2428;
--color-diff-remove-bg: #ffebe9 / #2c1c20;
```

### Example Component Prompts

**Hero Section**
> "Create a hero section on white background. Headline at 3.5rem Inter weight 700, line-height 1.1, letter-spacing -0.02em, color #0a0a0a. Subtitle at 1.25rem weight 400, color #525252, max-width 600px. Primary CTA button (#0066ff bg, white text, 6px radius, 8px 16px padding) and secondary ghost button. Include a terminal snippet below showing `npx codemod init` with syntax highlighting."

**Repository Card**
> "Design a package card: white background, 1px solid rgba(0,0,0,0.08) border, 12px radius. Title at 1rem weight 600, description at 0.875rem weight 400 color #525252. Include a pill badge for version (`0.75rem, 600, 9999px radius, rgba(0,102,255,0.1) bg`). Add a 'Run' button (primary style) and 'View Source' link (ghost style). Hover: border-color #0066ff, shadow level-200."

**Diff Viewer Snippet**
> "Build a diff block: container with 8px radius, 1px border. Two-column layout: gutter (3rem wide, right-aligned) + content. Added lines: bg #e6ffed (light) / #1f2428 (dark), text #22863a / #3fb950. Removed lines: bg #ffebe9 / #2c1c20, text #cf222e / #f85149. Font: JetBrains Mono 0.8125rem, line-height 1.5. Gutter shows '+' / '−' in semantic colors."

**Dark Mode Toggle**
> "Create a theme toggle button: 32×32px circle, transparent bg, border 1px solid rgba(0,0,0,0.08). Icon: sun/moon SVG, 16px, color #525252. Hover: bg rgba(0,0,0,0.04). Focus: blue ring. On toggle: persist preference in localStorage, apply `[data-theme='dark']` to `<html>`, transition colors with `0.2s ease`."

**Dashboard Widget**
> "Design a metrics card: 12px radius, white bg, subtle border. Header: title at 1rem weight 600, timestamp at 0.75rem color #8b949e. Body: large metric value at 2rem weight 700, trend indicator (▲/▼) with semantic color. Footer: 'View details' link in accent color. Hover: shadow level-200. Include optional sparkline chart using SVG with stroke #0066ff."

### Iteration Guide
1. **Always support both themes**: Every component must have light/dark variants using CSS variables
2. **Code is content**: Prioritize readability of syntax—use JetBrains Mono, proper line-height, and Shiki tokens
3. **Functional over decorative**: Shadows, borders, and animations should clarify hierarchy, not decorate
4. **Diff clarity first**: Added/removed states must be distinguishable in both themes and colorblind-safe
5. **Enterprise polish**: Subtle transitions (0.15–0.2s), consistent spacing, and clear focus states build trust
6. **Accessibility by default**: Contrast ratios, focus indicators, and ARIA attributes are non-negotiable
7. **Mobile is technical too**: Code blocks scroll horizontally; diffs switch to unified view; touch targets ≥44px
8. **Performance matters**: Lazy-load heavy syntax highlights; use `will-change` sparingly; prefer CSS transforms

### Theme Toggle Implementation
```javascript
// Persist preference
const theme = localStorage.getItem('theme') || 
  (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
document.documentElement.setAttribute('data-theme', theme);

// Toggle handler
function toggleTheme() {
  const current = document.documentElement.getAttribute('data-theme');
  const next = current === 'dark' ? 'light' : 'dark';
  document.documentElement.setAttribute('data-theme', next);
  localStorage.setItem('theme', next);
}
```

```css
/* CSS variable mapping */
[data-theme='light'] {
  --color-canvas-default: #ffffff;
  /* ... other light tokens */
}
[data-theme='dark'] {
  --color-canvas-default: #0d1117;
  /* ... other dark tokens */
}

/* Smooth transition for theme switch */
* {
  transition: background-color 0.2s ease, 
              border-color 0.2s ease, 
              color 0.2s ease;
}
```

---

> **Codemod Design Mantra**: *"Clarity enables confidence."*  
> Every visual decision should reduce cognitive load for developers performing high-stakes code migrations. When in doubt: simplify, clarify, and let the code speak.