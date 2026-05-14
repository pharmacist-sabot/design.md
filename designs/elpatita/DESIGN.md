# DESIGN-ELPATITA

> A design system for El Patita — a promotional restaurant site mixing a bright fast-food palette with rounded card geometry and a lightweight SPA-like flow.

## Design System Overview

**Source Inspiration:** | [El Patita](https://elpatita.com) |
**Status:** | Complete |

---

## 1. Visual Theme & Atmosphere

El Patita is built around a warm orange core, a secondary aqua accent, and soft off-white surfaces. Instead of a conventional top navigation, the primary navigation is a floating bottom dock: a pill-shaped bar fixed near the lower edge of the viewport, paired with a separate floating footer trigger on the lower-right. That single decision defines the product's personality more than any individual card or image.

The visual language is intentionally playful. The home screen is a collage grid of food photography, looping food videos, brand tiles, and one oversized CTA. Menu items are presented as rounded promo cards with a gradient hover frame. The local section turns into an information dashboard with contact cards, map embed, and FAQ accordion blocks. Legal and 404 pages reuse the same palette and rounded surfaces but collapse into simpler document-style containers.

Typography is single-family and utility-driven. **Nunito** is used everywhere, in three weights only: 400, 600, and 700. There is no display-face/body-face split. Personality comes from composition, color blocking, uppercase labels, rounded containers, and motion rather than font contrast.

Dark mode is a true alternate surface treatment rather than a full token system rewrite. The site swaps the page canvas to near-black, inverts several highlighted cards, changes the mode icon from moon to sun, and preserves orange as the primary brand anchor. Because only selected components receive dark-mode overrides, the result is a hybrid inversion: recognizably the same UI, but with darker framing and higher contrast around cards and navigation.

**Key Characteristics:**

- Floating bottom navigation (`nav-dock`) centered horizontally with a 30px pill radius and orange border.
- Brand palette led by `#f0781f` (orange), `#ec5228` (orange-strong), and `#17c6d2` (aqua).
- Single-family typography stack: `Nunito` in regular 400, semibold 600, and bold 700.
- Rounded geometry dominates the system: 20px cards, 30px pills, and 50% circles for icon buttons and badges.
- Hero/home section uses a 7-column editorial collage on desktop, then progressively compresses to 4-column and 2-column layouts.
- Carta cards use a hidden gradient border revealed on hover, with a cream inset body and floating pill metadata.
- Motion is visible and brand-carrying: slide-in nav, fade-in section changes, bouncing loader stripes, icon hover lifts, and expanding CTA circle fill.
- Footer behaves like a floating utility menu, not a traditional full-width footer.

---

## 2. Color Palette & Roles

### Brand & Accent

| Token | Hex | Usage |
|-------|-----|-------|
| `orange-strong` | `#ec5228` | Hotter orange used for emphasis, hero brand highlights, FAQ heading background, and some active accents |
| `orange` | `#f0781f` | Main brand color. Used for nav border/background, cart nav surface, primary badges, CTA section background, and theme-color metadata |
| `orange-soft` | `#f8dfce` | Soft peach surface used for title panels, FAQ cards, and light accent zones |
| `aqua` | `#17c6d2` | Secondary accent used for logo tile, loader stripe, map/location support, and informational emphasis |
| `aqua-dark` | `#31acb5` | Dark-mode substitute for aqua surfaces |
| `whatsapp` | `#7eed4f` | One-off semantic accent for the phone/contact heading |

### Surface

| Token | Hex | Usage |
|-------|-----|-------|
| `white-soft` | `#f4f6f5` | Main light card fill used inside menu cards and copyright text |
| `white` | `#ffffff` | Page background in light mode, CTA fill, tooltip text contrast, legal nav underline, and light-on-dark inversion target |
| `black-deep` | `#0e081b` | Dark-mode page canvas and 404 page background |
| `black` | `#3b3b3b` | Dark neutral used for brand tiles, gif panel, footer surface, and dark-mode nav surface |
| `gray-dark` | `#545454` | Dark-mode expanded/secondary panel background |
| `gray-light` | `#d9d9d9` | Neutral badge surface for category pills |

### Text

| Token | Hex | Usage |
|-------|-----|-------|
| `text-dark` | `#3b3b3b` | Default readable dark text on light surfaces |
| `text-inverse` | `#ffffff` | Text and icon color on orange, aqua, and dark surfaces |
| `text-orange` | `#ec5228` | Heading highlight on light panels and key highlighted words in hero title/body |
| `text-aqua` | `#17c6d2` | Used in local address heading |

### Semantic & Utility

| Token | Value | Usage |
|-------|-------|-------|
| `focus` | `#f8dfce` | Visible focus ring for anchors and buttons via a 3px outline |
| `combo-gradient` | `linear-gradient(90deg, #f0781f, #f8dfce, #17c6d2)` | Menu-card hover frame |
| `shadow-card` | `rgba(0, 0, 0, 0.2)` | Hover shadow for cart pills, FAQ open state, and raised light surfaces |

---

## 3. Typography Rules

### Font Family

El Patita uses a strict one-family setup:

- **Nunito Regular** — 400 weight for body copy, descriptions, legal text, and supporting labels.
- **Nunito SemiBold** — 600 weight for subheadings, vertical image labels, and contact info emphasis.
- **Nunito Bold** — 700 weight for headlines, badges, accordions, and highly visible callouts.

No fallback-specific styling is defined beyond `sans-serif`. The design system assumes Nunito is present through bundled local `woff2` assets and should remain the primary voice of the product.

### Hierarchy

| Token | Size | Weight | Line Height | Letter Spacing | Use |
|-------|------|--------|-------------|----------------|-----|
| `hero-title` | `2.9em` desktop, `2.4em` ≤1280, `2em` ≤1024, `1.7em` ≤768, `1.6em` ≤480 | 700 | browser default | default | Main home headline `LA POLLERIA EL PATITA` |
| `section-title` | `1.5em` desktop, `1.2em` ≤1280, `1.05em` ≤1024 | 700 | browser default | default | `CARTA`, `LOCAL`, and FAQ section title blocks |
| `brand-mark` | `2.1em` desktop, `1.6em` ≤1280, `1.3em` ≤1024, `1.1em` ≤480 | 600 or 700 visual weight | browser default | default | `EL PATITA` brand lockup in the hero brand tile |
| `card-title` | `clamp(1.3em, 2vw, 1.6em)` | 700 | browser default | default | Menu card titles |
| `body-md` | `1em` base | 400 | browser default | default | General body copy and navigation labels |
| `body-sm` | `clamp(0.8em, 1vw, 0.95em)` | 400 | `1.3` on descriptions | default | Menu descriptions, badge text, utility labels |
| `body-xs` | `clamp(0.75em, 1.5vw, 0.9em)` | 400 | browser default | default | FAQ accordion body |
| `legal-title` | `2em` desktop, `1.5em` ≤768 | 700 | browser default | default | Legal page H1 |
| `legal-heading` | `1.4em` desktop, `1.1em` ≤768 | 700 | browser default | default | Legal page H2 |
| `caption` | `0.8em` desktop, `0.75em` ≤768 | 400 | browser default | default | Footer items and copyright |

### Principles

- Typography does not rely on tracking or display compression. Scale and weight carry hierarchy.
- Highlight words inside headings and lead text are colored, not resized.
- Cards stay readable by constraining description width to 80% desktop and 95% on smaller viewports.
- The system uses uppercase labels in strategic places (`EL PATITA`, category badges, CTA) to add energy without changing font family.
- Vertical writing is used once as a decorative typographic gesture inside `papas-banner` and collapses to horizontal on very small screens.

---

## 4. Component Stylings

### Navigation

**`nav-dock`** — floating bottom navigation

- Fixed at bottom center with `left: 0`, `right: 0`, `margin: 0 auto`, and `margin-bottom: 26px` desktop.
- `max-width: 350px`, orange 2px border, 30px radius, and optional dark fill in dark mode.
- Contains logo, three icon-only anchors, and a circular dark-mode toggle.
- Uses `slideUp` entrance animation on page load.

**`nav-icon-link`** — primary section switcher

- Circular padded icon target (`8px` padding) with tooltip text implemented via `::before`.
- Hover lifts icon upward and rotates slightly.
- Active state adds a centered 22px white underline that expands to 30px on hover.

**`darkmode-toggle`** — theme switch

- Circular icon control built from hidden checkbox plus SVG icon.
- Tooltip label changes from `Oscuro` to `Claro` in dark mode.
- JavaScript swaps icon from moon to sun and persists state in `localStorage`.

### Loader

**`loader-screen`** — full-page entrance loader

- Fixed full-viewport white or dark background depending on theme.
- Contains four 8px-wide vertical stripes inside a rotated loader frame.
- Stripe colors cycle through aqua, orange-strong, orange-soft, and orange with staggered delays.
- Hidden 200ms after load, then removed from DOM after transition end.

### Home / Inicio

**`hero-image-card`** — chicken and wings imagery

- Full-bleed image cards with `object-fit: cover`, 20px radius, and scale/opacity hover behavior.

**`brand-tile`** — combined logo and wordmark

- Dark background tile with logo plus `EL PATITA` wordmark.
- Spans two columns on desktop and becomes full-width on very small screens.

**`video-strip`** — looping food videos panel

- Orange-strong panel with three autoplaying, muted, inline videos.
- Uses contained video sizing rather than edge-to-edge crop.

**`papas-banner`** — vertical product banner

- Orange panel with semi-transparent absolute-positioned fries photo.
- Centers a vertical `CROCANTES` label using `writing-mode: vertical-lr`; becomes horizontal on ≤480px.

**`logo-tile`** — brand icon showcase

- Aqua square/rectangular tile featuring only the favicon logo.
- Logo rotates and scales on hover.

**`hero-title-panel`** — headline and supporting copy

- Orange-soft panel with centered headline and paragraph.
- Key inline spans shift to orange-strong.

**`brasas-video-card`** — single looping ember/brasas video panel

- Dark tile used as a contrast anchor among warmer surfaces.

**`cta-promo-panel`** — promotions entry point

- Orange background container holding `animated-link`.

**`animated-link`** — primary CTA button

- White pill button with orange arrows, dark expanding circle, and sliding text.
- Hover state changes label to white, shifts arrows in opposite directions, rounds corners down from pill to 12px, and expands the center circle to `220px`.
- Active state scales to `0.95` and adds a soft ring-like box shadow.

### Carta

**`cart-title-panel`** — Carta section title

- Orange-soft background, orange-strong text, same geometry as other section headers.

**`cart-nav`** — category navigation rail

- Orange rounded container with white pill links.
- Active and hover states invert links to white background with dark text and drop shadow.
- Categories: Promociones, Brasas, Parrillas, Guarniciones, Bebidas.

**`combo-card`** — product card

- Flex column card with centered content, cream inner background, and hidden gradient border revealed on hover.
- Uses 20px outer radius, 16px image radius, and clamped spacing for responsive density.
- Hover translates downward by 6px instead of lifting upward, which gives the cards a pressed-forward feel.

**`product-info`** — metadata pill row

- Wrap-enabled flex row containing category, price, discount, and optional half-price badge.

**`badge-category`**

- Gray-light pill with black-deep text.
- Used for labels such as `NUEVO`, `POPULAR`, `TENDENCIA`, `PERSONAL`.

**`badge-price`**

- Orange pill with white text.

**`badge-discount`**

- Deep-black pill in both themes because the selector applies `--negro-puro` globally.
- Displays struck-through old price when present.

**`badge-price-half`**

- Aqua pill for half-liter beverage pricing.

### Local

**`local-title-panel`** — local section heading

- Matches cart title styling.

**`local-message-card`**

- Aqua background with white text/icon.
- Communicates `Servicio Presencial` and acts like a status banner.

**`contact-card`**

- Dark background, orange-strong or semantic-colored heading, white detail copy.
- Three variants: hours, address, phone.
- Each card contains a large icon and wrapped text stack.

**`location-image-card`**

- Dark background with Huanchaco image underneath a centered white overlay label.
- Image sits at `opacity: 0.75` and scales on hover.

**`map-card`**

- Full-size embedded Google Map with 20px rounded iframe.

### FAQ

**`faq-title-panel`**

- Orange-strong background with white text.

**`faq-item`**

- Orange-soft rounded container housing a native `details` element.

**`faq-accordion`**

- Native disclosure UI restyled with custom emoji circle icon.
- Closed state keeps orange-soft background and dark text.
- Open state becomes white, gains a 10px orange-strong left border, rounded shape, and shadow.
- Summary icon changes from `👇` to `☝️` and rotates when opened.

### Footer

**`footer-toggle-button`**

- Fixed utility trigger in the lower-right corner of the viewport.
- 50px dark circular button desktop, 44px on very small screens.

**`footer-menu`**

- Popover-like dark panel positioned above the toggle, scaled from `0` to `1` on hover/focus.
- Width `220px` desktop, `175px` on mobile.

**`footer-item`**

- Light text links with subtle hover color shift to orange.

**`footer-item-survey`**

- White pill on dark background with black text.
- Hover inverts to aqua with white text and slight scale-up.

**`github-link`**

- Icon-only link that rotates and scales on hover.

### Legal & Error States

**`legal-container`**

- Cream document card (`white-soft`) centered on the page with 20px radius.
- Dark mode swaps this container to black base with white text.
- Headings stay orange and orange-strong; inline links underline and deepen on hover.

**`error-page`**

- Full-screen deep-black background.

**`error-container`**

- Centered white panel with 30px radius, orange heading, dark body text, and orange pill back button.

**`error-button`**

- Orange pill CTA that deepens to orange-strong on hover.

### Border Radius Scale

| Token | Value | Use |
|-------|-------|-----|
| `sm` | `14px` | Small-screen compressed card/image radius below 480px |
| `md` | `16px` | Inner combo card image radius |
| `lg` | `20px` | Default tile/card/panel radius across the site |
| `xl` | `30px` | Navigation dock, tooltip pills, price/category pills, footer menu corners, legal CTA |
| `full` | `50%` | Icon buttons, circular CTA fill, footer trigger, dark-mode toggle, FAQ summary icon |

### Geometry Rules

- Major content surfaces are rounded rectangles at 20px by default.
- Action chips and metadata use elongated pills at 30px.
- True circles are reserved for icon-only controls and decorative fill transitions.
- On very small devices, all primary cards and embeds compress from 20px to 14px to avoid oversized curvature.

---

## 5. Layout Principles

### Spacing System

- Base spacing behavior clusters around `6px`, `8px`, `10px`, `12px`, `16px`, `20px`, `28px`, and `36px`.
- Core section padding: `6.5dvh 10%` on desktop, `6dvh 4%` below 1440px, `10dvh 2%` below 1280px, and `12px` top with `72px` bottom below 768px.
- Reusable card paddings:
  - `nav-dock`: `10px 20px` desktop, `8px 16px` mobile, `6px 12px` at very small widths.
  - `hero-title-panel` / `local-title-panel` / `faq-title-panel`: `20px 6%`.
  - `cart-nav`: `28px 6%`.
  - `footer-menu`: `16px` desktop, `12px` mobile.
  - `legal-container`: `30px 4%` desktop, `20px 4%` mobile.

### Grid & Container

- **Home / Inicio**: 7-column, 3-row collage grid on desktop. Major blocks span 2 to 4 columns.
- **Carta**: title/navigation row above horizontally scrollable product grids. Desktop uses 3 columns for promotions/guarniciones/bebidas and 4 columns for brasas/parrillas.
- **Local**: 6-column, 4-row information grid on desktop with contact stack on the left, image/map in the middle, FAQ stack on the right.
- **Legal**: single centered container maxed at 1000px.
- **404**: vertically and horizontally centered single panel.

### Whitespace Philosophy

- Whitespace is compact-to-medium rather than luxurious. The system behaves like a promotional microsite optimized for dense value communication.
- Larger panels still preserve breathing room through rounded framing and colored blocks rather than large empty margins.
- The bottom navigation and floating footer intentionally leave the page body unobstructed by pushing persistent chrome to the lower edge.

---

## 6. Depth & Elevation

| Level | Treatment | Use |
|-------|-----------|-----|
| 0 — flat | Flat color fill, no shadow | Most tiles, hero panels, title panels, nav dock |
| 1 — hover transform | `transform: scale(0.97)` or icon shifts | Shared hover behavior on visual blocks and icons |
| 2 — raised light | `box-shadow: 0 5px 12px rgba(0, 0, 0, 0.2)` | Active cart-nav pills and some emphasized hover surfaces |
| 3 — accordion lift | `box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2)` plus border-left | Expanded FAQ details |
| 4 — gradient frame | Hidden pseudo-element gradient border that fades in | Menu combo cards on hover |

### Decorative Depth

- Loader stripes animate by scaling vertically, creating rhythmic depth without shadows.
- The CTA button uses a growing circle overlay from the center to simulate a liquid fill transition.
- Food images rely on crop and opacity changes on hover rather than layered overlays.
- The footer menu appears by scale transform from the bottom-right corner, reinforcing its utility-popover role.

---

## 7. Responsive Behavior

### Breakpoints

| Name | Width | Key Changes |
|------|-------|-------------|
| Desktop XL | `> 1440px` | Main sections use `6.5dvh 10%` padding and full 7-column / 6-column desktop grids |
| Desktop | `≤ 1440px` | Horizontal padding tightens to `4%` |
| Laptop | `≤ 1280px` | Section padding becomes `10dvh 2%`, hero scales down, combo cards become min-width 280px for horizontal scrolling |
| Tablet | `≤ 1024px` | Home grid collapses from 7 columns to 4; menu grids reduce to 2 columns; local cards wrap content more aggressively |
| Mobile | `≤ 768px` | Bottom padding added for fixed chrome, carta title/nav stack vertically, local grid becomes 2 columns, footer menu shrinks |
| Small Mobile | `≤ 480px` | Home becomes a 2-column then mostly single-span collage, menu grids become 1 column, radii compress to 14px, footer trigger shrinks |

### Collapsing Strategy

- The hero collage keeps its tile identity while reassigning spans instead of switching to a single-column list immediately.
- Carta categories remain button-like pills and wrap across lines until the smaller mobile layout.
- Local section reorders from 6-column dashboard to 2-column, then 1-column stacked flow.
- Footer stays floating at every breakpoint, only shrinking dimensions and internal spacing.
- Decorative vertical typography in `papas-banner` becomes horizontal only at the smallest breakpoint.

### Touch Targets

- Nav and theme controls remain circular padded targets even on the smallest layout.
- Footer toggle remains 50px desktop and 44px on small mobile.
- CTA and cart pills maintain generous internal padding for tap use.
- FAQ summary rows keep a dedicated circular icon plus padded text block for touch clarity.

### Motion Reduction

- A `prefers-reduced-motion: reduce` query globally suppresses animations and transitions by collapsing duration to `0.01ms` and disabling smooth scrolling.

---

## 8. Accessibility & States

### Focus States

- `focus` token (`#f8dfce`) provides a visible focus ring at 3px outline for all anchors and buttons.
- Keyboard navigation is supported natively via the `details` disclosure element for FAQ accordions.
- The dark-mode toggle uses a hidden checkbox for accessible toggling.

### Color & Contrast

- Text on orange and aqua surfaces uses `text-inverse` (`#ffffff`) for maximum contrast.
- Primary body text (`text-dark` `#3b3b3b`) on white surfaces provides adequate reading contrast.
- Dark mode increases contrast by inverting the page canvas to near-black (`#0e081b`) and using white text.
- Focus outline (`#f8dfce`) provides a light peach ring against both light and dark surfaces.

### Motion Preferences

- `prefers-reduced-motion: reduce` is respected globally.
- When active, all animations and transitions collapse to `0.01ms` duration.
- Smooth scrolling is disabled for users with motion sensitivity.

### Do's and Don'ts

**Do:**
- Use `orange` and `orange-strong` as the primary emotional drivers of the interface.
- Keep major content panels at 20px radius unless the breakpoint explicitly compresses them.
- Preserve the floating bottom navigation and floating footer trigger; they are core to the product identity.
- Use Nunito for every text role instead of introducing extra display or mono families.
- Let motion stay visible on primary interactions: nav icons, CTA arrows, footer popover, loader stripes, and FAQ expansion.
- Keep menu cards centered and badge-driven, with pricing exposed before description.
- Use aqua as a support accent, not a replacement for orange.

**Don't:**
- Don't convert the experience into a standard top-nav restaurant layout; the bottom-dock pattern is fundamental here.
- Don't replace the 20px/30px rounded system with sharp corners or glassmorphism effects.
- Don't overuse shadows; most depth comes from color blocking, gradients, and transforms.
- Don't introduce extra font families or ultra-thin type weights.
- Don't flatten the CTA hover treatment; the arrow swap and expanding circle are signature behaviors.
- Don't remove the dark-mode inversion from nav, legal surfaces, and highlighted cards without redesigning the whole theme.

### Known Gaps

- The CSS defines selective dark-mode overrides rather than a complete dual-theme token matrix, so some light-surface components retain their original styling in dark mode.
- The minified assets (`css/styles.min.css`, `js/script.min.js`) are treated as build mirrors of the readable source, not separate design authorities.
- No explicit form system exists beyond navigation, toggles, accordion controls, and footer interactions.
- The system does not define a formal spacing-token scale in code; spacing above is reverse-engineered from repeated literal values.
- Accessibility states beyond focus-visible and reduced motion are minimal; hover remains the dominant interaction affordance.

---

## 9. Agent Prompt Guide

### Quick Reference

- All CSS custom properties are defined in `:root` as the authoritative token layer for the live UI.
- The relationship between orange, orange-strong, and aqua must be preserved before adding any new accent.
- When creating new sections, choose from existing panel archetypes: `hero-title-panel`, `combo-card`, `contact-card`, or `legal-container`.
- Reuse the 20px card radius and 30px pill radius before inventing new corner values.
- If a new interaction is added, prefer transform, opacity, or color transitions over heavy shadows or blur.
- Keep dark mode additive and selective, matching the current pattern instead of designing a disconnected second theme.

### Iteration Order

1. Start from the CSS custom properties in `:root`; they are the authoritative token layer for the live UI.
2. Preserve the relationship between orange, orange-strong, and aqua before adding any new accent.
3. When creating new sections, choose between existing panel archetypes first: `hero-title-panel`, `combo-card`, `contact-card`, or `legal-container`.
4. Reuse the 20px card radius and 30px pill radius before inventing new corner values.
5. If a new interaction is added, prefer transform, opacity, or color transitions over heavy shadows or blur.
6. Keep dark mode additive and selective, matching the current pattern instead of designing a disconnected second theme.

### Example Prompt

```
Create a new promotions section for El Patita using the existing `combo-card` pattern. Use `orange-soft` as the section background, `orange` for CTA buttons, and `aqua` for any informational badges. Follow the 20px card radius and 30px pill radius conventions. Include hover gradient borders on cards. Support dark mode by swapping the section background to `black-deep` and cards to `black`.
```
