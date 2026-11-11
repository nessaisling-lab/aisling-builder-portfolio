---
name: Aisling Builder Portfolio
description: A scribe's desk under a window onto New York, built on the Ness brand kit, matte, legible first.
colors:
  bg: "oklch(0.955 0.008 210)"
  surface: "oklch(0.99 0.004 210)"
  soft: "oklch(0.925 0.014 205)"
  code-bg: "oklch(0.935 0.01 210)"
  ink: "oklch(0.22 0.02 225)"
  ink-2: "oklch(0.36 0.02 225)"
  muted: "oklch(0.47 0.018 225)"
  line: "oklch(0.87 0.012 212)"
  line-strong: "oklch(0.6 0.02 220)"
  accent: "oklch(0.45 0.075 212)"
  accent-ink: "oklch(0.99 0.004 210)"
  ok: "oklch(0.47 0.12 150)"
  risk: "oklch(0.5 0.17 15)"
  brass-deep: "oklch(0.5 0.09 78)"
  enamel: "oklch(0.44 0.14 298)"
  asphalt-r: "oklch(0.240 0.012 240)"
  desk-face: "#F4EAD5"
  desk-top: "#EAD9B6"
  desk-front: "#E1CBA2"
  desk-cut: "#B7996C"
  sky: "#CAE3ED"
  city: "#859DA6"
  city-2: "#5F7984"
  pane: "#A9C4CF"
typography:
  display:
    fontFamily: "Archivo, Arial Narrow, Helvetica Neue, Arial, sans-serif"
    fontSize: "clamp(2.2rem, 4.6vw, 3.6rem)"
    fontWeight: 800
    lineHeight: 1.06
    letterSpacing: "-0.02em"
    fontVariation: "'wdth' 112"
  headline:
    fontFamily: "Archivo, Arial Narrow, Helvetica Neue, Arial, sans-serif"
    fontSize: "clamp(1.5rem, 2.6vw, 2rem)"
    fontWeight: 800
    lineHeight: 1.12
    fontVariation: "'wdth' 108"
  title:
    fontFamily: "Archivo, Arial Narrow, Helvetica Neue, Arial, sans-serif"
    fontSize: "1.2rem"
    fontWeight: 800
    lineHeight: 1.15
    fontVariation: "'wdth' 108"
  body:
    fontFamily: "Atkinson Hyperlegible Next, Atkinson Hyperlegible, Segoe UI, Helvetica Neue, Arial, sans-serif"
    fontSize: "1.0625rem"
    fontWeight: 400
    lineHeight: 1.55
  deck:
    fontFamily: "Atkinson Hyperlegible Next, Atkinson Hyperlegible, Segoe UI, Helvetica Neue, Arial, sans-serif"
    fontSize: "clamp(1.08rem, 1.5vw, 1.22rem)"
    fontWeight: 400
    lineHeight: 1.55
  label:
    fontFamily: "Archivo, Arial Narrow, Helvetica Neue, Arial, sans-serif"
    fontSize: "0.85rem"
    fontWeight: 600
    lineHeight: 1.45
  place:
    fontFamily: "Marcellus SC, Georgia, Times New Roman, serif"
    fontWeight: 400
    letterSpacing: "0.09em"
  mono:
    fontFamily: "Atkinson Hyperlegible Mono, ui-monospace, Cascadia Mono, Consolas, monospace"
    fontSize: "0.88rem"
    fontWeight: 400
    lineHeight: 1.5
  cuneiform:
    fontFamily: "Noto Sans Cuneiform, sans-serif"
    fontWeight: 400
    letterSpacing: "0.08em"
rounded:
  s: "8px"
  pane: "6px"
  m: "12px"
  pill: "999px"
spacing:
  gutter: "max(16px, 3.5vw)"
  scene-gap: "clamp(1.5rem, 3.5vw, 3rem)"
  block: "clamp(2.75rem, 6vw, 4.5rem)"
  bezel: "10px"
components:
  button:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.accent}"
    rounded: "{rounded.pill}"
    padding: "0 18px"
    height: "44px"
  button-hover:
    backgroundColor: "{colors.soft}"
  button-primary:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.accent-ink}"
    rounded: "{rounded.pill}"
    padding: "0 18px"
    height: "44px"
  tab:
    textColor: "{colors.ink-2}"
    rounded: "{rounded.pill}"
    padding: "0.7rem 0.95rem"
  tab-hover:
    backgroundColor: "{colors.soft}"
    textColor: "{colors.ink}"
  tab-current:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.accent-ink}"
  plate:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    rounded: "{rounded.m}"
    padding: "10px 16px 10px 14px"
    height: "84px"
  wall-card:
    backgroundColor: "{colors.bg}"
    textColor: "{colors.ink}"
    rounded: "{rounded.m}"
    padding: "1.6rem 1.75rem 1.75rem"
  drawer:
    backgroundColor: "{colors.desk-face}"
    textColor: "{colors.ink}"
    rounded: "{rounded.s}"
    padding: "1rem 1.25rem"
    height: "7.5rem"
  tablet:
    backgroundColor: "{colors.desk-top}"
    textColor: "{colors.ink-2}"
    width: "5.8rem"
    height: "7.4rem"
  laptop-screen:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    rounded: "{rounded.pane}"
  input:
    backgroundColor: "{colors.bg}"
    textColor: "{colors.ink}"
    rounded: "{rounded.s}"
    padding: "0.65rem 0.75rem"
---

# Design System: Aisling Builder Portfolio

## Overview

**Creative North Star: "The Scribe's Desk at the Window"**

The home page is one room seen straight on: a card pinned to the wall, a window onto the New York skyline, a laptop open to the work, a clay tablet and a cup of reeds beside it, and a desk whose three drawers lead to the other sections. Every object is a way in except the reeds and the skyline, which are decoration and are hidden from assistive technology. The character is an ancient Sumerian scribe living in the modern city. It shows only in objects and in a few words, never as a costume or a figure.

The system is two layers. The Ness brand kit (`brand/ness.css`, `brand/ness.js`) supplies the page tokens, buttons, plates, spec sheets, place-name type and the reading-settings panel, and holds its own contrast floor. This site adds the world: warm brass-and-clay desk tints and the kit's cool petrol for the city, generated from `site/src/theme.rs`, where every text-carrying desk colour is checked against WCAG 4.5:1 at compile time. Inner pages (Projects, case studies, Experience, Integrations, School, Résumé, Contact) are laid out on the kit as spec sheets until each gets a world of its own.

The finish is matte everywhere (`data-finish="matte"` on `<html>`). Density is calm: one room above the fold, then plain reading. The build rejects the split hero over a row of equal cards, the AI SaaS look (gradient heroes, identical card grids, tiny uppercase labels, big-number stat tiles), terminal green on black, and glow or bloom.

**Key Characteristics:**
- Two lights, one place: day (light brass and clay, petrol city) and evening (the same desk in low light, the city's windows lit).
- Contrast is a build failure, not a review comment: desk colours go through `assert_readable!`.
- Engraved containers: double inset hairlines, not drop shadows.
- Sumerian is a hint beside the English, never instead of it.
- Still by default; the flat page works with no script, no motion and no sound.

## Colors

Cool kit petrol and slate for the page and the city, set against a light, warm desk in brass and clay tints.

### Primary
- **Kit Petrol** (`accent`): where you are and what you can press. Links, the primary button fill, the current tab, focus rings, list markers, the drawer title on hover. It is asserted at 4.5:1 on the drawer face in both lights.

### Neutral
- **Petrol Mist** (`bg`): the page and the wall card's face; also the window frame, mullion and sill.
- **Paper White** (`surface`): the room behind the scene, the laptop screen, plates, spec panels, forms, the résumé sheet.
- **Soft Wash** (`soft`): hover fill for buttons and tabs, video letterbox.
- **Deep Slate Ink** (`ink`), **Second Ink** (`ink-2`), **Slate Muted** (`muted`): text in three steps. Muted is still held to 4.5:1 on the drawers; nothing is lighter than muted.
- **Hairline** (`line`) and **Strong Hairline** (`line-strong`): rules between rows, the inner and outer lines of engraved containers. Form field edges use the strong line so a field's edge reaches 3:1.
- **Asphalt** (`asphalt-r`): the laptop's body around the screen.
- **Code Wash** (`code-bg`): code blocks in the Integrations verification panels.

### The desk and the city (site world, from `theme.rs`)
- **Drawer Face** (`desk-face`): the drawer fronts, the only desk surface that carries body-size text. Evening `#3B3121`.
- **Clay Tablet** (`desk-top`): the desk's top edge (a 16px band) and the clay tablet. Evening `#312718`.
- **Desk Front** (`desk-front`): the panel the drawers sit in. Decorative. Evening `#271D10`.
- **Incised Line** (`desk-cut`): drawer edges, the tablet's double incision, the reeds. Decorative. Evening `#62533C`.
- **Day Sky** (`sky`), **Street Front** (`city`), **Set-Back Block** (`city-2`): the skyline. Decorative; no text sits on them. Evening `#1B2A3C`, `#0E1C28`, `#08131C`.
- **Window Glass** (`pane`): by day every pane is this one glass (`lit` and `unlit` are equal). In the evening `unlit` is `#162633` and `lit` is lamplight `#E6D8A3`, about two panes in seven, chosen by a deterministic hash so the page never changes on its own.

### Accent marks
- **Works Green** (`ok`): the open-to-work dot and the case-study status dot, always beside words.
- **Crimson** (`risk`): the contact form's error message.
- **Brass** (`brass-deep`): the drawer pulls. Decoration only.
- **Enamel** (`enamel`): the pin holding the wall card. Decoration only.

### Named Rules
**The Desk Holds Its Text Rule.** Any kit text colour that can land on the drawer face or the tablet (ink, ink-2, muted, accent) is asserted at 4.5:1 against it in both worlds by a `const` assertion in `theme.rs`. Small lines get no large-text discount. A new desk colour, or a change to the kit's inks, is mirrored into `theme.rs` so the check still means something.

**The Lit Window Rule.** Lit windows are the evening's one event. By day pane and building stay under 1.6:1 so the skyline never reads as lit at noon; in the evening lit panes exceed 7:1 against the buildings. Both are unit tests.

**The One Meaning Rule.** Petrol means place and action, green means works today, crimson means risk. Brass and enamel are decoration only: never text, status or data.

## Typography

**Display Font:** Archivo, variable width (62 to 125) and weight (with Arial Narrow, Helvetica Neue, Arial)
**Body Font:** Atkinson Hyperlegible Next, roman and italic (with Atkinson Hyperlegible, Segoe UI, Arial)
**Label/Mono Font:** Atkinson Hyperlegible Mono, for code only
**Place Font:** Marcellus SC, for place names and transliterated Sumerian only
**Sign Font:** Noto Sans Cuneiform, subset to the eight signs the site uses (DUB, SAR, KI, EN, GI, KIN, E2, BA)

**Character:** A stretched, heavy grotesque for what you scan, set against the most legible body face available, with an inscriptional small-cap serif kept to a few words at a time.

### Hierarchy
- **Display** (800, `clamp(2.2rem, 4.6vw, 3.6rem)`, 1.06, width 112%, -0.02em): page titles. On the wall card the headline steps down to `clamp(1.85rem, 2.6vw, 2.25rem)` at most 17ch.
- **Headline** (800, `clamp(1.5rem, 2.6vw, 2rem)`, width 108%): section headings, at most 30ch. Case-study section headings are 1.5rem with a muted tabular number beside them.
- **Title** (800, 1.1 to 1.3rem, width 106 to 108%): her name on the card, drawer titles, project names in the log, pager names, footer name. Plate titles are the kit's 700 1.1rem.
- **Body** (400, 1.0625rem, 1.55): reading text at a 66ch measure (`--measure`); the kit caps paragraphs at 68ch. Reading settings scale size, line height and spacing.
- **Deck** (400, `clamp(1.08rem, 1.5vw, 1.22rem)`, ink-2, 60ch): the line under a page title.
- **Label** (Archivo 600 to 700, 0.82 to 0.92rem, sentence case, muted): ID-row and spec-sheet terms, form labels. Never uppercase, never letter-spaced.
- **Place** (Marcellus SC 400, +0.09em): "New York" on the card, the Sumerian words on the tablet, the laptop hint and the School drawer.
- **Mono** (0.88rem, 1.5): code blocks only.

### Named Rules
**The Beside-the-English Rule.** Marcellus SC and cuneiform never stand alone. Every Sumerian word carries `lang="sux-Latn"` and sits next to its English gloss ("kin · the work", "é-dub-ba · the tablet house"); every cuneiform sign is `aria-hidden`, with the meaning in text or in the control's label.

**The Tabular Figures Rule.** Numbers that line up (plate numbers, case-study figures, timeline dates, résumé dates, project stats) use lining tabular figures.

## Layout

The page is centred in a `--wrap` of 86rem with a gutter of `max(16px, 3.5vw)`. Block rhythm is `clamp(2.75rem, 6vw, 4.5rem)` between sections; the footer sits `clamp(4rem, 9vw, 6rem)` below the last block.

**The home scene** is a named-area grid. Wide: three columns `10fr 11fr auto` with the card spanning both rows on the left, the window across the top right, and the laptop and the tablet-and-reeds group below it, all ending on the desk's top edge. Column gap `clamp(1.5rem, 3.5vw, 3rem)`.
- **At 85rem and below** the tablet no longer fits beside the laptop without its plates stacking, so it stands under the card: `5fr 6fr`, areas card/window, card/laptop, things/laptop.
- **At 64rem and below** everything stacks in reading order: card, window, laptop, things, with a 1.25rem row gap. The laptop becomes a closed rounded slab.
- **At 40rem and below** the portrait shrinks to 5.5rem, ID rows stack, plates and drawers go to one column.
- The laptop screen is a size container: plates go to two columns once the screen is at least 30rem wide, whatever the viewport.

**The desk** is a full-width band below the scene: a 16px clay top edge, then three equal drawers with a 1rem gap.

**Inner pages** follow the kit's spec-sheet model: thin full-width rules between rows, label columns beside wide values. The project log is a four-column row (name, line, stat, year); case studies are a reading column beside a sticky 16 to 20rem spec panel. At 52rem and below every one of these collapses to one column, the spec panel unsticks and moves above the body, and on phones role, team and stack fold into a disclosure so the video comes first. At 46rem and below the top bar stops sticking and the five tabs take their own wrapping row so none hides off screen; the settings button shows its icon alone but keeps its words for screen readers.

**Print** drops the top bar, footer, pager, media and the decorative window, tablet and reeds.

## Elevation & Depth

Depth comes from objects and engraving, not from shadows. The finish is matte: surfaces get a hairline and at most a 1px lift (`0 1px 2px var(--shadow)`, the kit's `--lift`). The only longer shadow is the tablet's translation slip, which floats over the scene. The window is built from rings of `box-shadow` spread (frame, outline) and pseudo-elements (sill, mullion), not from blur.

### Shadow Vocabulary
- **Engraved double hairline** (`inset 0 0 0 1px var(--line-strong), inset 0 0 0 4px var(--bg), inset 0 0 0 5px var(--line)`): the wall card. Plates, spec panels and rule cards use the same structure at 3px/4px on surface.
- **Pin and card lift** (`0 1px 2px var(--shadow)`): the enamel pin and the wall card.
- **Drawer seat** (`inset 0 0 0 1px var(--desk-cut), 0 1px 0 var(--desk-cut)`): a drawer at rest; on hover or focus the outline darkens to ink-2 and the ledge moves above (`0 -1px 0`).
- **Floating slip** (`0 0 0 1px var(--line-strong), 0 8px 20px -10px var(--shadow)`): the tablet's translation only.

### Named Rules
**The Matte Rule.** No glaze, sheen, glow or bloom on this site. `data-finish="matte"` is set on every page, and depth stays at a hairline and a 1px lift.

**The Engraved Tag Rule.** A container that holds something you choose or read as a record (wall card, plate, spec panel, rule card) gets the double inset hairline, like an engraved tag. Plain panels (forms, verify panels, the résumé sheet, transcripts) get a single inset hairline.

## Shapes

Rounded but not soft: 8px for drawers, chips, inputs, photos and slips; 12px for the wall card, plates, spec panels and the laptop; 6px panes for the window glass and the laptop screen; pills for buttons and tabs. The tablet has its own clay silhouette, 18px at the top and 10px at the bottom, with a double incised border. Drawer pulls are 2.9rem brass pills; the pin is a 14px enamel circle centred on the card's top edge. Reeds are 5px rounded staves leaning a few degrees in a rounded cup.

## Components

### Buttons
Kit buttons, quiet and firm.
- **Shape:** full pill (999px), 44px minimum height, 0 18px padding, 1.5px petrol border.
- **Default:** Paper White with petrol text; hover fills Soft Wash.
- **Primary:** petrol fill, Paper White text. One primary per group: Résumé on the wall card, Send on the form, "Talk about a project".
- **Disabled:** strong-hairline border, muted text.

### Navigation
- **Top bar:** sticky, Petrol Mist, hairline under it, at least 4rem tall. Her name in Archivo 800 at width 112% on the left; tabs on the right; the reading-settings button last.
- **Tabs:** Archivo 650, 0.95rem, width 106%, pill hit area 0.7rem by 0.95rem in ink-2. Hover fills Soft Wash; the current page (`aria-current="page"`) fills petrol with Paper White text.
- **Phone:** see Layout (46rem).

### Plates
The way to choose a project. A number column (plate number in tabular Archivo, with a 40px matte Library model above it when the project has one) beside the name and the project's hardest number. Paper White, 12px corners, the engraved double hairline; hover darkens the outer line to ink-2. On the home laptop they stand at least 84px tall in a numbered list.

### Wall card
Her ID card pinned to the wall: Petrol Mist face, engraved double hairline, enamel pin. A 7.5rem 4:5 portrait beside the name, the place in Marcellus SC, and spec rows (Archivo label, 650 value) on hairlines; the open-to-work row leads with the green dot. Then the headline, a two-line story at 52ch, and the Résumé and Email buttons.

### Window
The city through the glass, drawn as inline SVG at build time by `site/src/skyline.rs` from a fixed list of buildings (a rounded crown, two New York water tanks). A 10px Petrol Mist frame with an outer strong hairline, a 10px centre mullion and a sill that overhangs by 18px. Height `clamp(12rem, 19vw, 17rem)`. Decorative and `aria-hidden`; CSS colours the panes per world.

### Clay tablet
The signature interaction. A native `<details>`: the summary is the tablet (two lines of cuneiform, labelled "Clay tablet: read the inscription"), with a muted "Press the tablet" cue above it until it is opened. Opening it shows a floating slip with the Sumerian in Marcellus SC and the English translation. It needs no script and announces its state. Hover lifts it 3px.

### Drawers
The other sections, set into the desk. Drawer Face fill, 8px corners, incised edge, a brass pull spanning the left, then the section title (Archivo 800, 1.3rem), a one-line summary in ink-2, and an optional Sumerian hint line in muted. On hover or focus the drawer eases out 4px toward the reader and the title turns petrol.

### Spec sheets
The kit's `nb-spec` for definition lists: a strong rule on top, a hairline under each row, terms in ink-2, values in Archivo 600. The site sets values left-aligned in body type where they are sentences. Case studies carry a sticky spec panel: figures in a two-column grid, then role, team and stack rows, then links.

### Timeline, log and rules
Records on rules. The timeline and project log are rows on hairlines under a strong top rule, with tabular dates in muted. Rule cards (Experience, School, Integrations) are a three-column set of engraved panels: a title, an ink-2 line, then the rule itself in ink.

### Inputs / Fields
- **Style:** Petrol Mist background, 1.5px strong-hairline border (3:1), 8px corners, 0.65rem by 0.75rem padding, inheriting body type. Labels in Archivo 700 above.
- **Focus:** border turns petrol plus a 3px petrol outline at 1px offset.
- **Status:** a live line under the form; errors in crimson, always with an email fallback in words. The form posts normally without script.

### Reading settings
From the kit, mounted into the top bar's slot by `site.js`: text size, line spacing, contrast, light (day or evening), motion and finish, saved in local storage and applied before first paint. Without the kit's script the slot stays empty and hidden. The site's palette block uses the kit's selectors, so the Light choice switches the desk and the city along with the page.

## Do's and Don'ts

### Do:
- **Do** generate desk and city colours from `site/src/theme.rs` and let `assert_readable!` hold every text pair on a desk surface to 4.5:1; never hand-edit the generated `/*@PALETTES@*/` block.
- **Do** make kit changes in the kit (D:\ness-brand), run its `node build.mjs`, and copy `dist/ness.css` and `dist/ness.js` into `brand/`. The copies here are not edited.
- **Do** pair every Sumerian word and cuneiform sign with its English, and mark Sumerian `lang="sux-Latn"` and signs `aria-hidden`.
- **Do** keep motion still by default: drawers ease 4px and the tablet lifts 3px over 0.2 to 0.25s on `cubic-bezier(.16,1,.3,1)`; the kit removes every transition under reduced motion or the Motion setting; build recordings play only while on screen and never under reduced motion.
- **Do** build interactions on native elements first (`<details>` for the tablet and folded specs, a real form post) and treat `site.js` as enhancement only.
- **Do** keep 44px minimum targets, a 3px petrol focus ring, a skip link, and field edges at 3:1.
- **Do** self-host every font with its OFL licence beside it, and record provenance for every raster that ships (see the sidecar).

### Don't:
- **Don't** put text on the sky or skyline, or on the desk front or incised colours; only the drawer face and the tablet carry text.
- **Don't** light the windows by day.
- **Don't** add glaze, sheen, glow halos or bloom; the finish is matte.
- **Don't** use brass or enamel for text, status or data.
- **Don't** add a kicker above a heading or tiny uppercase letter-spaced labels; labels are sentence case.
- **Don't** use Marcellus SC for anything but place names and Sumerian words, or cuneiform outside the tablet and the Projects hint.
- **Don't** use terminal green on black, gradient heroes or identical card grids.
- **Don't** show the capstone client's name, logo or branding in any image or text.
