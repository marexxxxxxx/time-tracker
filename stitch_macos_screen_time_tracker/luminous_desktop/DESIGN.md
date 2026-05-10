---
name: Luminous Desktop
colors:
  surface: '#faf9fe'
  surface-dim: '#dad9df'
  surface-bright: '#faf9fe'
  surface-container-lowest: '#ffffff'
  surface-container-low: '#f4f3f8'
  surface-container: '#eeedf3'
  surface-container-high: '#e9e7ed'
  surface-container-highest: '#e3e2e7'
  on-surface: '#1a1b1f'
  on-surface-variant: '#414755'
  inverse-surface: '#2f3034'
  inverse-on-surface: '#f1f0f5'
  outline: '#717786'
  outline-variant: '#c1c6d7'
  surface-tint: '#005bc1'
  primary: '#0058bc'
  on-primary: '#ffffff'
  primary-container: '#0070eb'
  on-primary-container: '#fefcff'
  inverse-primary: '#adc6ff'
  secondary: '#006e28'
  on-secondary: '#ffffff'
  secondary-container: '#6ffb85'
  on-secondary-container: '#00732a'
  tertiary: '#4c4aca'
  on-tertiary: '#ffffff'
  tertiary-container: '#6664e4'
  on-tertiary-container: '#fffbff'
  error: '#ba1a1a'
  on-error: '#ffffff'
  error-container: '#ffdad6'
  on-error-container: '#93000a'
  primary-fixed: '#d8e2ff'
  primary-fixed-dim: '#adc6ff'
  on-primary-fixed: '#001a41'
  on-primary-fixed-variant: '#004493'
  secondary-fixed: '#72fe88'
  secondary-fixed-dim: '#53e16f'
  on-secondary-fixed: '#002107'
  on-secondary-fixed-variant: '#00531c'
  tertiary-fixed: '#e2dfff'
  tertiary-fixed-dim: '#c2c1ff'
  on-tertiary-fixed: '#0c006a'
  on-tertiary-fixed-variant: '#3631b4'
  background: '#faf9fe'
  on-background: '#1a1b1f'
  surface-variant: '#e3e2e7'
typography:
  display:
    fontFamily: Inter
    fontSize: 40px
    fontWeight: '700'
    lineHeight: 48px
    letterSpacing: -0.02em
  headline-lg:
    fontFamily: Inter
    fontSize: 28px
    fontWeight: '600'
    lineHeight: 34px
    letterSpacing: -0.01em
  headline-md:
    fontFamily: Inter
    fontSize: 22px
    fontWeight: '600'
    lineHeight: 28px
  body-lg:
    fontFamily: Inter
    fontSize: 17px
    fontWeight: '400'
    lineHeight: 24px
  body-md:
    fontFamily: Inter
    fontSize: 15px
    fontWeight: '400'
    lineHeight: 20px
  label-md:
    fontFamily: Inter
    fontSize: 13px
    fontWeight: '500'
    lineHeight: 16px
    letterSpacing: 0.01em
  label-sm:
    fontFamily: Inter
    fontSize: 11px
    fontWeight: '600'
    lineHeight: 14px
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  unit: 4px
  xs: 4px
  sm: 8px
  md: 16px
  lg: 24px
  xl: 32px
  xxl: 48px
  margin-desktop: 40px
  gutter: 20px
---

## Brand & Style

This design system captures the focused, airy essence of desktop productivity. It leans heavily into a **Glassmorphic** and **Minimalist** hybrid style, prioritizing clarity and intentionality. The goal is to evoke a sense of "digital wellness"—calm, organized, and premium. 

The aesthetic is characterized by a "Vibrancy" effect, where background colors subtly bleed through semi-transparent surfaces, creating a layered, physical presence. It is designed for users who appreciate high-end software craftsmanship, utilizing expansive white space to reduce cognitive load. The emotional response should be one of "quiet power"—a professional tool that feels light and effortless to use.

## Colors

The palette is rooted in the "system" colors familiar to desktop environments. 

- **Primary (System Blue):** Used for primary actions, active states, and focus indicators.
- **Secondary (System Green):** Primarily used for "Success" states and positive growth metrics in charts.
- **Tertiary (System Indigo):** Used for secondary categories or to differentiate data sets in complex views.
- **Surface & Background:** The background uses a slightly off-white light gray to provide contrast for the pure white cards and panels.
- **Vibrancy:** Sidebars and toolbars should utilize a 70-80% opacity with a high-saturation backdrop blur (20px+) to simulate the macOS translucency effect.

## Typography

This design system utilizes **Inter** for its neutral, highly legible characteristics that closely mirror Apple’s San Francisco typeface. 

- **Hierarchy:** Use `display` only for dashboard hero numbers (e.g., total screen time). 
- **Body Text:** `body-md` (15px) is the standard desktop body size, providing a modern, slightly compact feel.
- **Weight:** Medium (500) and Semi-Bold (600) weights are used frequently for labels to maintain legibility against translucent backgrounds.
- **Tracking:** Headlines use slightly negative letter spacing to feel tighter and more premium, while small labels use positive tracking for clarity.

## Layout & Spacing

The layout follows a **Fixed Grid** approach for internal content containers, centered within a fluid application shell. 

- **Application Shell:** Uses a sidebar (250px-300px width) with a vibrancy effect, pinned to the left.
- **Grid:** A 12-column system is used within the main content area, with generous 40px outer margins to maintain the "airy" feel.
- **Rhythm:** An 8px linear scale drives all spacing. Dashboard cards should typically be separated by `lg` (24px) spacing to allow the white space to act as a separator rather than lines.
- **Mobile Reflow:** On mobile, the 12-column grid collapses to 1 column with 16px side margins; the sidebar transforms into a bottom navigation bar or a full-screen overlay menu.

## Elevation & Depth

Depth is communicated through **Backdrop Blurs** and **Low-Contrast Outlines** rather than heavy shadows.

- **Level 1 (Background):** The base canvas (`#F5F5F7`).
- **Level 2 (Translucent):** Sidebars and toolbars with `backdrop-filter: blur(30px)` and a 1px inner border of white at 20% opacity.
- **Level 3 (Cards):** Pure white surfaces with a very soft, high-diffusion shadow (`y: 4, blur: 20, color: rgba(0,0,0,0.04)`) and a 1px solid border (`#D1D1D6`).
- **Level 4 (Modals/Popovers):** Floating elements with a more pronounced shadow and a slightly thicker 1.5px border to distinguish them from the background.

## Shapes

The design system uses a distinctive **Soft Rounded** language. 

- **Primary Containers:** Large dashboard cards and main window frames must use a minimum of **20px** corner radius.
- **Nested Elements:** To maintain visual harmony, inner elements (like buttons inside a card) use a smaller **10px** radius.
- **Continuity:** Ensure "squircle" (continuous curvature) logic is applied where possible in CSS via custom shapes or high-quality SVG masks to truly replicate the macOS aesthetic.

## Components

- **Buttons:** Primary buttons use a solid Blue background with white text. Secondary buttons use a subtle gray tint (`rgba(0,0,0,0.05)`) with Blue text. No heavy gradients; use a very subtle 2% brightness increase on hover.
- **Cards:** Essential for this system. Cards must be white, bordered, and contain generous internal padding (24px).
- **Segmented Controls:** Used for switching time views (Day/Week). These should look like a single recessed track with a white "pill" that slides behind the active text.
- **Charts:** Progress bars and bar charts use highly rounded caps (pill-shaped). Use the primary blue and secondary green with 20% opacity versions of the same color for the "track" background.
- **Sidebars:** Use a vibrancy effect with `label-md` for navigation items. The active state is indicated by a blue icon and a subtle background highlight.
- **Input Fields:** Clean, white backgrounds with a 1px border. On focus, the border color changes to Blue with a 3px soft blue outer glow (halo).