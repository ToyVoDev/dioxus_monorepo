# Design System Specification

## 1. Overview & Creative North Star: "The Kinetic Monolith"

This design system is engineered to bridge the gap between high-fidelity creative expression and raw technical utility. Our Creative North Star is **"The Kinetic Monolith"**—a philosophy that treats the UI as a single, sophisticated instrument carved from dark matter.

Unlike standard "flat" dashboards, this system rejects the "box-in-a-box" template. We achieve a premium, editorial feel through **Intentional Asymmetry** and **Tonal Depth**. We prioritize breathing room and optical weight over structural lines. The interface should feel like a high-end physical hardware synth or a custom-built developer environment: precise, expensive, and profoundly functional.

---

## 2. Colors & Surface Philosophy

The palette is rooted in deep neutrals, punctuated by a "Bold Red" primary that signals action and precision.

### The Palette (Material Design Tokens)
* **Background / Surface:** `#131313` (The void from which all elements emerge).
* **Primary (Bold Red):** `#FFB3AD` (High-visibility, used for critical actions and brand signatures).
* **Secondary (Blue-Green):** `#44E2CD` (Tech-forward, color-blind friendly, used for creative workflows).
* **Tertiary (Amber/Yellow):** `#F9BD22` (Utility, warnings, and high-contrast accents).

### The "No-Line" Rule
**Prohibit 1px solid borders for sectioning.** To separate a sidebar from a main content area, do not draw a line. Instead, shift the background token.
* *Example:* A sidebar using `surface-container-high` (`#2A2A2A`) sitting against a main content area using `surface` (`#131313`).

### Surface Hierarchy & Nesting
Treat the UI as a series of physical layers. Use the `surface-container` tiers to create "nested" depth:
1. **Base Layer:** `surface` (`#131313`)
2. **Sectioning:** `surface-container-low` (`#1B1C1C`)
3. **Component Cards:** `surface-container-highest` (`#353535`)

### The "Glass & Gradient" Rule
For floating elements (modals, music player controllers), use **Glassmorphism**. Apply a semi-transparent `surface-variant` with a 20px–40px backdrop blur. For primary CTAs, avoid flat fills; use a subtle linear gradient from `primary` (`#FFB3AD`) to `primary-container` (`#FF5451`) at a 135° angle to give the element "soul" and weight.

---

## 3. Terminal Palette (ANSI 16-Color)

For the HTTP client and portfolio terminal components, we integrate the brand colors into the standard 16-color ASCII palette to ensure the technical console feels like a native part of the brand.

| Color | Normal (0-7) | Bright (8-15) | Token Reference |
| :--- | :--- | :--- | :--- |
| **Black** | `#0E0E0E` | `#353535` | Surface Lowest / Highest |
| **Red** | `#EF4444` | `#FFB3AD` | Primary / Primary Fixed |
| **Green** | `#00A38E` | `#44E2CD` | Secondary / Secondary Fixed |
| **Yellow** | `#B88900` | `#F9BD22` | Tertiary / Tertiary Fixed |
| **Blue** | `#2E5BFF` | `#7092FF` | (Custom Utility) |
| **Magenta** | `#930013` | `#FF5451` | Primary Container |
| **Cyan** | `#005047` | `#62FAE3` | Secondary Container |
| **White** | `#E4E2E1` | `#FFFFFF` | On-Surface |

---

## 4. Typography: The Editorial Scale

We pair **Space Grotesk** (Display/Headlines) with **Inter** (UI/Body) and a high-fidelity **Monospace** for technical data.

* **Display-LG (3.5rem / Space Grotesk):** Reserved for portfolio headers and hero moments. Use tight letter-spacing (-0.02em).
* **Headline-MD (1.75rem / Space Grotesk):** For major section titles.
* **Title-MD (1.125rem / Inter):** Standard UI grouping titles.
* **Body-MD (0.875rem / Inter):** Default text. Increase line-height to 1.6 for readability in music streamer bios.
* **Technical (Monospace):** Used for HTTP responses, code blocks, and the Terminal component.

---

## 5. Elevation & Depth: Tonal Layering

Shadows and lines are relics. We define space through **The Layering Principle**.

* **Ambient Shadows:** When an element must float (e.g., a music "Now Playing" bar), use a shadow with a 40px blur, 0% spread, and 6% opacity. The shadow color should be sampled from the `on-surface` hex, not pure black.
* **The "Ghost Border" Fallback:** If a border is required for accessibility (e.g., input focus), use `outline-variant` (`#5B403E`) at **20% opacity**. Never use 100% opaque lines.
* **Rounding:** Follow the roundedness scale strictly.
* **Cards/Containers:** `lg` (1rem).
* **Buttons/Inputs:** `DEFAULT` (0.5rem).
* **Chips:** `full` (9999px).

---

## 6. Components

### Buttons
* **Primary:** Gradient fill (`primary` to `primary-container`), `on-primary-container` text. No border.
* **Secondary:** `surface-container-highest` fill with a "Ghost Border."
* **Tertiary:** Text only, `primary` color, transitions to a subtle `surface-variant` background on hover.

### Cards & Lists
* **The Divider Rule:** Forbid the use of divider lines. Use **Spacing Scale 4 (1rem)** to separate list items, or alternate background shifts between `surface-container-low` and `surface-container-high`.
* **Music Streamer Cards:** Use `xl` (1.5rem) rounding for album art to contrast with the `lg` rounding of the parent container.

### Inputs (Technical/HTTP Client)
* **States:** Use `surface-container-lowest` for the field background to create an "inset" feel. On focus, the "Ghost Border" becomes 40% opacity `primary` red.
* **Monospace Integration:** All data entry in the HTTP client must use the `label-md` monospace token for character alignment.

---

## 7. Do’s and Don’ts

### Do
* **Use Asymmetry:** In the portfolio, align text to a 12-column grid but leave columns 1-3 empty to create high-end editorial tension.
* **Layer Surfaces:** Place a `surface-container-highest` card inside a `surface-container-low` section.
* **Color-Blind Logic:** Use the Amber (`tertiary`) for warnings and Blue-Green (`secondary`) for success states to ensure accessibility without relying on red/green alone.

### Don’t
* **Don’t use #000000:** It kills the "Monolith" depth. Use `surface-container-lowest` (`#0E0E0E`) for your darkest blacks.
* **Don’t use 1px Borders:** If you feel the need to "separate" something, increase the padding (Spacing Scale) or shift the surface tone.
* **Don’t Over-Round:** Avoid `full` rounding on large cards; it looks juvenile. Stick to the `lg` (1rem) standard for a professional feel.
