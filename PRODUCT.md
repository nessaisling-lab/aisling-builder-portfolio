# Product

## Register

brand

## Users
Employers, hiring managers and recruiters deciding whether to talk to Aisling; Pursuit staff; possible collaborators and
clients for data-integration work. They arrive from a résumé, LinkedIn or a link in an email, often on a phone, and give
the site a minute or two. Some read with low vision, dyslexia or a sensitivity to motion, so legibility and control come
first.

## Product Purpose
Aisling (Stan) Leiva-Davila's portfolio: what she has built and how she builds. She is an AI-native builder in New York.
She works with AI (mostly Claude Code, plus Gemini and local models like Qwen), chooses compiled, systems-level
languages on purpose (Rust first), makes the architecture calls, writes the specs, and reviews, tests and ships what
comes out. The site should make a reader want to talk to her, and every claim on it must hold up in that conversation.

## The worlds (Gran Turismo energy, understated)
Moving through the site should feel like moving between countries and brands in Gran Turismo: each place has its own
flair, palette, models and, optionally, its own theme music. Toned down, so it still reads as professional.

- **Home, About, Résumé:** New York and the scribe's desk. The character is an ancient Sumerian living in NYC, finding a
  way into tech and finance. It shows in objects (a clay tablet, stylus and reeds on a modern desk, the city out the
  window) and in the words (place names, a line or two of her story), never as a costume or a character figure.
  The desk and the clay are light, warm tints of the kit's brass (Aisling, Nov 11: warm fits Sumer); the city stays in
  the kit's cool petrol. Cuneiform and Sumerian words are hints and easter eggs, always paired with the plain English.
- **Home leads with Aisling and her story** (who she is, how she builds: the forge, not the factory); the way into the
  work sits right under it.
- **Main sections**, tabs at the upper right: Career or Experience, Projects, Integrations, and others to be decided
  (School, Résumé). Each section has its own world; ask Aisling for each one as the work reaches it. Projects is a
  garage or a factory.
- **Each project is its own world**, entered from its plate on the Projects page; each plate carries a short summary.
  - SiteAssure: the job site.
  - Ziqpu: Sumerian sky and astrology (Ziqpu's own world: mosaic inlay; clay, shell and gold; the wheat loader).
  - The gala capstone: New York with Asian American touches (Aisling's choice, Nov 11, made knowing it may make the
    client guessable). The client's name, logo and branding still never appear.
  - Resona: computers and graphics, the garage.
  - HouseCheck: left out of the rebrand on purpose (built with a teammate). Keep its presentation; ask before changing it.
  - Nisaba: terminal colors and plates, no 3D, no green on black.
- Finish: matte everywhere (asked Nov 11, 2026).
- Sound: each world gets a sound control, off by default; no music ships until tracks are chosen. Never copyrighted game
  soundtracks or remixes of them; a separate session is researching licensed, free-to-use and self-made music.

## Brand Personality
Clear, calm, confident, quietly delighted by detail, and a little video-gamey. Adult, but lets you geek out. Frank
wording, no corporate speak: say what's built and what isn't.

## Anti-references
- The kit's anti-references: Gran Turismo's own logos, type, screens, cars or race markers (its feeling only); an object
  on a plinth that you drag to turn with nothing to teach; the AI SaaS look (gradient heroes, identical card grids, tiny
  uppercase labels, big-number stat tiles); terminal green on black; glow halos and bloom.
- The current site's "lab notebook" look (warm paper, ruled grid, Newsreader serif) is evidence for content and
  structure only. This is a redesign, not a polish of it.

## Design Principles
- **Truth before polish.** Nothing implies more than she has done. She doesn't write code by hand; the site never says
  or implies she does. The Salesforce work is "tested against a stand-in org". Unbuilt things are labeled plainly.
- **Legible first.** Reading settings on every page, the kit's contrast floor, and everything works with no motion and
  no sound. Music never autoplays.
- **The flat version comes first.** 3D, ASCII video and sound open on request and add nothing the flat page lacks.
- **The work leads** (Experience mode). The interface steps back from the projects.
- **Rust is the site's DNA.** Pages stay rendered by the Dioxus crate in `site/`; new behavior prefers Rust where it is
  practical.

## Constraints
- Static GitHub Pages under `/aisling-builder-portfolio/`; CI fails if committed pages differ from `cargo run` output.
- Fonts are self-hosted (no Google requests); the kit's fonts must be self-hosted too (SIL OFL).
- Copy the kit's `ness.css`, `ness.js`, `explore3d.js` and chosen Library models into `brand/`; never link to `D:\`.
- ASCILINE: its web player is MIT and may ship; its engine and Rust core are AGPL, so they run only offline to make
  `.ascf` files and never go into this repo.
- Credit teammates by name and role (Aisling's choice, Nov 11). The capstone client is never named, and the site's test
  fails if the name appears. No age-revealing dates. No prices. "Open to work" stays.
- Her name is "Aisling (Stan) Leiva-Davila"; Aisling is the preferred name.

## Accessibility & Inclusion
WCAG 2.2 AA at least; high contrast reaches 7:1 for reading text. Text to 138%, roomy spacing, reduced motion, plain
surfaces, day and evening themes, keyboard and phone width. Sound is opt-in with a visible control.
