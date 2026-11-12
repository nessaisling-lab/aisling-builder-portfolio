//! Renders the portfolio: Dioxus RSX in, static HTML pages out.
//!
//! Same approach as the L2 portfolio, for the same reason: GitHub Pages serves this repo
//! under `/aisling-builder-portfolio/`, and Dioxus's own static-site mode cannot take a
//! base path (DioxusLabs/dioxus#3942). Rendering each VirtualDom to a String and writing
//! plain files sidesteps it. No router, no hydration, no WASM; links are relative, so the
//! subdirectory never has to be spelled out.

mod content;
mod garage;
mod resume;
mod sky;
mod skyline;
mod theme;

use content::*;
use dioxus::prelude::*;
use std::path::{Path, PathBuf};

const STYLE: &str = include_str!("../assets/style.css");
const SCRIPT: &str = include_str!("../assets/site.js");
/// Live 3D for the Projects hall. It imports assets/vendor/three.module.min.js (three.js r169, MIT).
const TURNTABLE: &str = include_str!("../assets/turntable.js");
const SITE_URL: &str = "https://nessaisling-lab.github.io/aisling-builder-portfolio/";
const NAME: &str = "Aisling (Stan) Leiva-Davila";
const EMAIL: &str = "ness.aisling@gmail.com";
/// Applies the reader's saved reading settings before first paint, so a reader who chose
/// Evening or Larger text never sees a flash of the defaults. brand/ness.js keeps them
/// in sync after that; it uses the same storage key.
const SETTINGS_EARLY: &str = "try{var s=JSON.parse(localStorage.getItem('ness-brand-settings'))||{};\
for(var k in s)if(s[k])document.documentElement.setAttribute('data-'+k,s[k])}catch(e){}";
/// Web3Forms public access key. It is designed to sit in page source.
const FORM_KEY: &str = "a0888a12-c951-4b99-bc7c-fe8da519a5f6";

/// The repository root, where the pages are written and the assets live.
fn site_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn main() -> std::io::Result<()> {
    let out = site_root();
    let mut written = Vec::new();

    write(&out.join("assets/site.css"), &css(), &mut written)?;
    write(&out.join("assets/site.js"), SCRIPT, &mut written)?;
    write(&out.join("assets/turntable.js"), TURNTABLE, &mut written)?;

    // The résumé's three sources. tools/build-resume.sh turns them into PDF, Word, txt, html.
    let cv = out.join("assets/docs/resume");
    write(&cv.join("resume.tex"), &resume::to_tex(), &mut written)?;
    write(&cv.join("resume.md"), &resume::to_md(), &mut written)?;
    write(&cv.join("resume.json"), &resume::to_json(), &mut written)?;

    let home = rsx! { Home { root: "".to_string() } };
    write(&out.join("index.html"), &page("", "", "AI-native builder, Rust first",
        "Aisling (Stan) Leiva-Davila builds with AI in New York: she makes the architecture calls, writes the specs, and reviews, tests and ships, Rust first.",
        home), &mut written)?;

    let work = rsx! { Work { root: "../".to_string() } };
    write(&out.join("work/index.html"), &page("../", "work/", "Projects",
        "Builds in Rust and on small teams: signed exports, on-device AI, Postgres row-level security.",
        work), &mut written)?;

    for (i, b) in BUILDS.iter().enumerate() {
        let body = rsx! { Case { root: "../../".to_string(), i: i } };
        let path = format!("work/{}/", b.slug);
        write(&out.join(&path).join("index.html"), &page("../../", &path, b.name, b.line, body), &mut written)?;
    }

    let integration = rsx! { Integration { root: "../".to_string() } };
    write(&out.join("integration/index.html"), &page("../", "integration/", "Data integration",
        "Data integration for small teams: migrations into Salesforce, app-to-Salesforce syncs, and keeping them running.",
        integration), &mut written)?;

    let experience = rsx! { Experience { root: "../".to_string() } };
    write(&out.join("experience/index.html"), &page("../", "experience/", "Experience",
        "Where Aisling has worked: the Pursuit client capstone, Nisaba, Architechne, and a business of her own.",
        experience), &mut written)?;

    let school = rsx! { School { root: "../".to_string() } };
    write(&out.join("school/index.html"), &page("../", "school/", "School",
        "Pursuit's AI-Native Builder program (inaugural cohort), Per Scholas IT Support, and A&P coursework at Vaughn College.",
        school), &mut written)?;

    let contact = rsx! { Contact { root: "../".to_string() } };
    write(&out.join("contact/index.html"), &page("../", "contact/", "Contact",
        "Email Aisling, or send a message from this page.",
        contact), &mut written)?;

    // About's story moved to Home (Nov 11). Old links still land somewhere useful.
    let about = rsx! { Moved { root: "../".to_string() } };
    write(&out.join("about/index.html"), &page("../", "about/", "About",
        "The About page moved: the story is on the home page now.",
        about), &mut written)?;

    let resume = rsx! { Resume { root: "../".to_string() } };
    write(&out.join("resume/index.html"), &page("../", "resume/", "Résumé",
        "Résumé as PDF (Letter, Legal), plus writing samples.",
        resume), &mut written)?;

    // GitHub Pages serves 404.html at whatever path was missed, so relative links would
    // resolve against the wrong directory. This one page uses the absolute site root.
    let lost = rsx! { NotFound { root: SITE_URL.to_string() } };
    write(&out.join("404.html"), &page(SITE_URL, "404", "Not found", "That page does not exist.", lost), &mut written)?;

    eprintln!("wrote {} files:", written.len());
    for p in written {
        eprintln!("  {}", p.display());
    }
    Ok(())
}

fn write(path: &Path, body: &str, log: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(path, body)?;
    log.push(path.to_path_buf());
    Ok(())
}

/// The stylesheet, with the desk's day and evening colours generated from theme.rs. The
/// selectors match brand/ness.css, so the reading settings' Light choice switches both.
fn css() -> String {
    let day = theme::css_vars(&theme::DAY);
    let evening = theme::css_vars(&theme::EVENING);
    let palettes = format!(
        "/* GENERATED from site/src/theme.rs. Edit the Rust, not this block. Every kit text\n\
         \x20  colour that sits on the desk is held to WCAG 4.5:1 by a const assertion. */\n\
         :root{{\n{day}}}\n\
         @media (prefers-color-scheme: dark){{:root:not([data-theme=light]){{\n{evening}}}}}\n\
         :root[data-theme=dark]{{\n{evening}}}\n"
    );
    STYLE.replace("/*@PALETTES@*/", &palettes)
}

/// A WebVTT file's spoken text as paragraphs, each with the second it starts at: cue text
/// joined in order, with a new paragraph where the narration pauses for a second or more
/// after a full stop.
/// ponytail: no cue tags (<v>, <i>) or entities are handled; the walkthrough captions
/// have none. Strip them here if a captions file ever gains them.
fn transcript(vtt: &str) -> Vec<(f64, String)> {
    let secs = |ts: &str| ts.split(':').fold(0.0, |acc, p| acc * 60.0 + p.parse::<f64>().unwrap_or(0.0));
    let mut paras = vec![(0.0, String::new())];
    let mut last_end = None;
    for block in vtt.replace("\r\n", "\n").split("\n\n") {
        // Skips the WEBVTT header, NOTE blocks and cue identifiers: text starts after the timing line.
        let mut lines = block.lines().skip_while(|l| !l.contains("-->"));
        let Some(timing) = lines.next() else { continue };
        let mut times = timing.split("-->").map(|t| secs(t.split_whitespace().next().unwrap_or("")));
        let (start, end) = (times.next().unwrap_or(0.0), times.next().unwrap_or(0.0));
        let text = lines.map(str::trim).collect::<Vec<_>>().join(" ");
        let cur = &paras.last().expect("never empty").1;
        let sentence_done = cur.trim_end_matches(['"', '\u{201D}', ')']).ends_with(['.', '!', '?']);
        if last_end.is_some_and(|e: f64| start - e >= 1.0) && sentence_done {
            paras.push((start, String::new()));
        }
        let cur = paras.last_mut().expect("never empty");
        if cur.1.is_empty() {
            cur.0 = start;
        } else {
            cur.1.push(' ');
        }
        cur.1.push_str(&text);
        last_end = Some(end);
    }
    paras.retain(|p| !p.1.is_empty());
    paras
}

/// Seconds as a walkthrough's clock: 1:07.
fn clock(s: f64) -> String {
    let s = s as u64;
    format!("{}:{:02}", s / 60, s % 60)
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Wraps a rendered body in the document shell. `root` is the relative path back to the
/// site root from this page; `path` is this page's path from the root, for canonical URLs.
fn page(root: &str, path: &str, title: &str, desc: &str, body: Element) -> String {
    let body = dioxus_ssr::render_element(body);
    let full_title = if path.is_empty() {
        format!("{NAME} · {title}")
    } else {
        format!("{title} · {NAME}")
    };
    let (t, d) = (esc(&full_title), esc(desc));
    let canonical = if path == "404" { SITE_URL.to_string() } else { format!("{SITE_URL}{path}") };
    let robots = if path == "404" { "<meta name=\"robots\" content=\"noindex\">\n" } else { "" };
    // three.js loads on the Projects hall only, where the turntable needs it.
    let three = if path == "work/" {
        format!("<script type=\"module\" src=\"{root}assets/turntable.js\"></script>\n")
    } else {
        String::new()
    };
    // Case studies share their own card (tools/og-cards.mjs); every other page, the site's.
    let og = match path.strip_prefix("work/").map(|s| s.trim_end_matches('/')) {
        Some(slug) if !slug.is_empty() => format!("assets/og/{slug}.png"),
        _ => "assets/og-image.png".to_string(),
    };
    let header = dioxus_ssr::render_element(rsx! { Top { root: root.to_string(), here: path.to_string() } });
    let footer = dioxus_ssr::render_element(rsx! { Foot { root: root.to_string() } });
    format!(
        "<!doctype html>\n\
         <html lang=\"en\" data-finish=\"matte\">\n\
         <head>\n\
         <meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>{t}</title>\n\
         <meta name=\"description\" content=\"{d}\">\n\
         {robots}<link rel=\"canonical\" href=\"{canonical}\">\n\
         <meta property=\"og:type\" content=\"website\">\n\
         <meta property=\"og:title\" content=\"{t}\">\n\
         <meta property=\"og:description\" content=\"{d}\">\n\
         <meta property=\"og:url\" content=\"{canonical}\">\n\
         <meta property=\"og:image\" content=\"{SITE_URL}{og}\">\n\
         <meta name=\"twitter:card\" content=\"summary_large_image\">\n\
         <meta name=\"theme-color\" content=\"#EAF2F3\" media=\"(prefers-color-scheme: light)\">\n\
         <meta name=\"theme-color\" content=\"#0A191A\" media=\"(prefers-color-scheme: dark)\">\n\
         <link rel=\"icon\" href=\"{root}assets/favicon.svg\" type=\"image/svg+xml\">\n\
         <link rel=\"icon\" href=\"{root}assets/favicon-32.png\" sizes=\"32x32\">\n\
         <link rel=\"apple-touch-icon\" href=\"{root}assets/apple-touch-icon.png\">\n\
         <script>{SETTINGS_EARLY}</script>\n\
         <link rel=\"preload\" href=\"{root}assets/fonts/archivo.woff2\" as=\"font\" type=\"font/woff2\" crossorigin>\n\
         <link rel=\"preload\" href=\"{root}assets/fonts/atkinson-next.woff2\" as=\"font\" type=\"font/woff2\" crossorigin>\n\
         <link rel=\"stylesheet\" href=\"{root}brand/ness.css\">\n\
         <link rel=\"stylesheet\" href=\"{root}assets/site.css\">\n\
         </head>\n\
         <body class=\"nb\">\n\
         <a class=\"skip\" href=\"#main\">Skip to content</a>\n\
         {header}\n\
         <main id=\"main\">\n{body}\n</main>\n\
         {footer}\n\
         <script src=\"{root}brand/ness.js\" defer></script>\n\
         <script src=\"{root}assets/site.js\" defer></script>\n\
         {three}\
         </body>\n\
         </html>\n"
    )
}

// ── shell ────────────────────────────────────────────────────────────────────

#[component]
fn Top(root: String, here: String) -> Element {
    let cur = |p: &str| if here.starts_with(p) { "page" } else { "false" };
    rsx! {
        header { class: "top",
            div { class: "wrap top-in",
                a { class: "brand", href: "{root}", "aria-current": if here.is_empty() { "page" } else { "false" },
                    "{NAME}"
                }
                nav { class: "tabs", "aria-label": "Site",
                    a { href: "{root}work/", "aria-current": cur("work/"), "Projects" }
                    a { href: "{root}experience/", "aria-current": cur("experience/"), "Experience" }
                    a { href: "{root}integration/", "aria-current": cur("integration/"), "Integrations" }
                    a { href: "{root}school/", "aria-current": cur("school/"), "School" }
                    a { href: "{root}resume/", "aria-current": cur("resume/"), "Résumé" }
                }
                // brand/ness.js mounts the reading settings here. Without the script the page
                // still follows the device's light, motion and contrast preferences.
                div { class: "settings-slot" }
            }
        }
    }
}

#[component]
fn Foot(root: String) -> Element {
    rsx! {
        footer { class: "foot",
            div { class: "wrap foot-in",
                div {
                    p { class: "foot-name", "{NAME}" }
                    p {
                        a { href: "mailto:{EMAIL}", "{EMAIL}" }
                        " · "
                        a { href: "{root}contact/", "Contact form" }
                        " · "
                        a { href: "https://www.linkedin.com/in/stan-leiva-davila/", "LinkedIn" }
                        " · "
                        a { href: "https://github.com/nessaisling-lab", "GitHub" }
                    }
                }
                p { class: "colophon",
                    "Rendered from Rust: Dioxus RSX in, static HTML out, no WASM. The skyline is drawn
                     by Rust at build time, and the desk's colours are held to WCAG 4.5:1 by a
                     compile-time assertion. 3D models by Kenney, Quaternius, Kay and reyshapes (CC0, via
                     Poly Pizza), drawn live with three.js (MIT). "
                    a { href: "https://github.com/nessaisling-lab/aisling-builder-portfolio/tree/main/site", "Source" }
                }
            }
        }
    }
}

// ── home: the desk at the window ─────────────────────────────────────────────
//
// One room, seen straight on: her card pinned to the wall, the city through the window,
// the work open on the laptop, a clay tablet beside it, and the other sections in the
// desk's drawers. Every object is a way in except the reeds and the skyline, which are
// marked aria-hidden. Without CSS it reads top to bottom as card, projects, sections.

#[component]
fn Home(root: String) -> Element {
    let skyline = skyline::svg();
    rsx! {
        div { class: "room",
            div { class: "wrap scene",
                section { class: "card", "aria-labelledby": "who",
                    span { class: "pin", "aria-hidden": "true" }
                    div { class: "id",
                        img {
                            class: "id-photo",
                            src: "{root}assets/portrait.jpg",
                            srcset: "{root}assets/portrait.jpg 1x, {root}assets/portrait@2x.jpg 2x",
                            alt: "Portrait of Aisling, smiling",
                            width: "440",
                            height: "550",
                            fetchpriority: "high",
                        }
                        div { class: "id-text",
                            p { class: "id-name", "{NAME}" }
                            p { class: "id-place nb-place", "New York" }
                            dl { class: "id-rows",
                                div { dt { "Builds with" } dd { "AI, Rust first" } }
                                div { dt { "Status" } dd { class: "open", span { span { class: "nw", "Open to work ·" } " " span { class: "nw", "on-site or hybrid" } } } }
                            }
                        }
                    }
                    h1 { id: "who", "I build with AI the way a smith works a forge." }
                    p { class: "story",
                        "I make the architecture calls, write the specs, and review, test and ship what
                         comes out. Rust first, on purpose: a compiler that checks the promise is a
                         promise you can keep."
                    }
                    div { class: "actions",
                        a { class: "nb-btn is-primary", href: "{root}assets/docs/resume/resume-letter.pdf", "Résumé (PDF)" }
                        a { class: "nb-btn", href: "mailto:{EMAIL}", "Email me" }
                    }
                }

                div { class: "window", "aria-hidden": "true", dangerous_inner_html: "{skyline}" }

                section { class: "laptop", "aria-labelledby": "projects-h",
                    div { class: "screen",
                        h2 { id: "projects-h", class: "screen-h",
                            "Projects"
                            span { class: "hint",
                                span { class: "cun", "aria-hidden": "true", "\u{121A5}" }
                                span { class: "nb-place", lang: "sux-Latn", "kin" }
                                " · the work"
                            }
                        }
                        ol { class: "plates",
                            for (i, b) in BUILDS.iter().filter(|b| b.featured).enumerate() {
                                Plate { root: root.clone(), n: i + 1, slug: b.slug.to_string() }
                            }
                        }
                        p { class: "screen-more",
                            a { href: "{root}work/", "All projects, including HouseCheck and Nisaba →" }
                        }
                    }
                }

                // The easter egg: a real tablet's words, and what they mean. A native
                // <details>, so it opens with no script and announces its state.
                div { class: "things",
                details { class: "tablet",
                    summary { "aria-label": "Clay tablet: read the inscription",
                        span { class: "cun", "aria-hidden": "true", "\u{1207E}\u{122AC}" }
                        span { class: "cun", "aria-hidden": "true", "\u{121A0}\u{12097}\u{12100}" }
                        span { class: "tablet-cue", "aria-hidden": "true", "Press the tablet" }
                    }
                    p { class: "tablet-says",
                        span { class: "nb-place", lang: "sux-Latn", "dub-sar · ki-en-gi" }
                        " Scribe · Sumer. Writing began on clay, as record-keeping."
                    }
                }
                div { class: "reeds", "aria-hidden": "true", i {} i {} i {} b {} }
                }
            }
        }

        div { class: "desk",
            nav { class: "wrap drawers", "aria-label": "More about Aisling",
                a { class: "drawer", href: "{root}experience/",
                    span { class: "pull", "aria-hidden": "true" }
                    span { class: "drawer-t", "Experience" }
                    span { class: "drawer-s", "The client capstone, Nisaba, Architechne, my own business" }
                }
                a { class: "drawer", href: "{root}integration/",
                    span { class: "pull", "aria-hidden": "true" }
                    span { class: "drawer-t", "Integrations" }
                    span { class: "drawer-s", "Salesforce syncs, migrations, Postgres" }
                }
                a { class: "drawer", href: "{root}school/",
                    span { class: "pull", "aria-hidden": "true" }
                    span { class: "drawer-t", "School" }
                    span { class: "drawer-s", "Pursuit, Per Scholas, Vaughn College" }
                    span { class: "drawer-x", span { class: "nb-place", lang: "sux-Latn", "é-dub-ba" } " · the tablet house" }
                }
            }
        }

        section { class: "wrap block story-block", "aria-labelledby": "how",
            h2 { id: "how", class: "block-h", "How I got here" }
            div { class: "prose",
                p {
                    "I learned computers the hacker way. Getting Debian and a Wi-Fi driver running on a
                     PlayStation 3 took months of books, forums and boards. Aircraft-mechanic coursework
                     at Vaughn College came next and taught me methodical troubleshooting and respect
                     for a checklist; a family emergency paused that degree, and the habits stayed."
                }
                p {
                    "I ran a short-term-rental business on my own, then did data analysis and lead
                     generation at Architechne. After Per Scholas IT Support, I joined the inaugural
                     cohort of Pursuit's AI-Native Builder program, where I was technical director and
                     repo admin on a client capstone."
                }
                p {
                    "Today I build with Claude Code, Gemini and local models like Qwen. They do the
                     hammering; I choose the metal, make the architecture calls, write the specs, and
                     test every piece before it ships."
                }
            }
            dl { class: "nb-spec now-list",
                dt { "Capstone" } dd { "Presented at Pursuit Demo Day on Nov 9, 2026, then handed the code and docs to the client." }
                dt { "Looking" } dd { "Rust, security and privacy, AI, and backend roles in New York, on-site or hybrid." }
                dt { "Recently" } dd { "Found and closed a forgery path in HouseCheck's signed export by writing an independent verifier." }
            }
        }
    }
}

/// A numbered plate: number and model on the left, name and the hardest number on the right.
#[component]
fn Plate(root: String, n: usize, slug: String) -> Element {
    let b = BUILDS.iter().find(|b| b.slug == slug).expect("featured build exists");
    let num = format!("{n:02}");
    rsx! {
        li {
            a { class: "nb-plate", href: "{root}work/{b.slug}/",
                span { class: "nb-plate-no",
                    if let Some((file, _)) = plate_model(b.slug) {
                        img { class: "plate-model turn", src: "{root}brand/library/{file}-turn.webp", alt: "", width: "64", height: "64" }
                    }
                    "{num}"
                }
                span {
                    span { class: "nb-plate-t", "{b.name}" }
                    span { class: "nb-plate-s", "{b.stat}" }
                }
            }
        }
    }
}

// ── work ─────────────────────────────────────────────────────────────────────

/// The build on show when the Projects page opens: the client capstone.
const ON_SHOW: &str = "gala-guest-list";

/// The builds in the order the program ran them, for the hall's Program order sort: L2's
/// numbered cycles, then L3's pitch and capstone. Resona (L2, its cycle not stated on this
/// site) and Nisaba (a founder build, outside the program) follow.
const PROGRAM: [&str; 7] = ["siteassure", "ziqpu", "housecheck", "excalidraw", "gala-guest-list", "resona", "nisaba"];

/// The builds in hall order, Rust first, then team and client work. The hall's doors, the
/// select strip and the arrow keys all follow it.
fn hall_order() -> Vec<&'static Build> {
    BUILDS.iter().filter(|b| b.group == Group::Integrity)
        .chain(BUILDS.iter().filter(|b| b.group == Group::Team))
        .collect()
}

/// Projects is a garage hall, drawn by garage.rs: the build on show turns on a turntable in
/// front of its walkthrough, which fills the far wall; its spec sheet stands beside it, its
/// transcript opens from the floor, and a list of every build down the left picks the next
/// one. Without script every tile is a link to its build and the capstone stays on show.
#[component]
fn Work(root: String) -> Element {
    let builds = hall_order();
    // Each tile's model for live 3D; turntable.js draws it, and the flipbook stands in until then.
    let models: Vec<String> = builds.iter()
        .map(|b| plate_model(b.slug).map(|(f, _)| format!("{root}brand/library/{f}-model.json")).unwrap_or_default())
        .collect();
    let (desk_at, note_at) = garage::desk_spot();
    rsx! {
        section { class: "hall", "aria-labelledby": "projects-h",
            // The room's camera and its 3D props, for turntable.js.
            div { class: "hall-box", "data-room": garage::room(), "data-props": garage::props_json(&root),
                div { class: "hall-draw", dangerous_inner_html: garage::svg() }
                h1 { id: "projects-h", class: "hall-h1", "Projects" }
                for (i, b) in builds.iter().enumerate() {
                    Show { root: root.clone(), n: i + 1, slug: b.slug.to_string() }
                }
                // Past work, on a clipboard hung on the tool wall; pressing it opens the list.
                button { class: "clipboard", "type": "button", "popovertarget": "egg-log",
                    span { class: "clip-h", "Past work" }
                    span { class: "clip-n", "{EARLIER.len()} earlier builds" }
                }
                // The contact desk: computer, phone and fax; any of it opens Contact. turntable.js
                // draws it in the room in 3D; its still stands in until then. Pointed at, or tapped
                // once on a phone (site.js), it shows its note: kiĝ₂, work, and a message.
                div { class: "desk-spot", style: "{desk_at}",
                    a { class: "hall-desk", href: "{root}contact/",
                        span { class: "visually-hidden", "Contact: write to me" }
                        img { class: "desk-still", src: "{root}brand/library/built--contact-desk-matte.webp", alt: "", width: "256", height: "256" }
                        span { class: "kin-card", style: "{note_at}", "aria-hidden": "true",
                            span { class: "kin-g", "\u{121A5}" }
                            span { class: "kin-t", b { lang: "sux-Latn", "kin" } " · work, and a message" }
                            span { class: "kin-s", "One Sumerian word for both. This desk is for yours." }
                            span { class: "kin-go", "Write to me →" }
                        }
                    }
                }
                // Sorting the list, as a racing game sorts its cars; site.js shows it and sorts.
                div { class: "sort", hidden: true,
                    label { "for": "sort-by", "Sort" }
                    select { id: "sort-by",
                        option { value: "featured", "Featured" }
                        option { value: "program", "Program order" }
                        option { value: "integrity", "Rust · a promise kept" }
                        option { value: "team", "Team & client" }
                    }
                }
                // How to drive the hall, like a game's controls screen.
                button { class: "legend-btn", "type": "button", "popovertarget": "hall-keys", "aria-label": "Controls",
                    span { class: "ico", dangerous_inner_html: ICON_KEYS }
                }
                ol { class: "strip", "aria-label": "Every build",
                    for (i, b) in builds.iter().enumerate() {
                        li { "data-group": if b.group == Group::Integrity { "integrity" } else { "team" },
                            "data-program": "{PROGRAM.iter().position(|s| *s == b.slug).unwrap_or(PROGRAM.len())}",
                            a { class: "tile", href: "{root}work/{b.slug}/", "aria-current": if b.slug == ON_SHOW { "true" } else { "false" },
                                "data-model": "{models[i]}",
                                if let Some((file, _)) = plate_model(b.slug) {
                                    img { class: "plate-model turn", src: "{root}brand/library/{file}-turn.webp", alt: "", width: "44", height: "44" }
                                }
                                span { class: "tn", "{i + 1:02}" }
                                span { class: "tt", "{b.name}" }
                            }
                        }
                    }
                }
            }
        }
        for b in builds.iter() {
            BuildCards { root: root.clone(), slug: b.slug.to_string() }
        }
        Card { id: "hall-keys".to_string(),
            h3 { "Driving the hall" }
            dl { class: "legend",
                dt { kbd { "◀" } kbd { "▶" } } dd { "Step through the builds, anywhere in the hall." }
                dt { kbd { "▲" } kbd { "▼" } } dd { "The same, in the list of builds." }
                dt { kbd { "Enter" } } dd { "Open the build that's on show." }
                dt { "Drag" } dd { "Turn the model on the turntable." }
                dt { "Sort" } dd { "Featured, program order, or one class of build." }
                dt { "0:42" } dd { "A time in the transcript plays the walkthrough from there." }
                dt { kbd { "Esc" } } dd { "Close a card like this one." }
            }
            p { "Text size, spacing, contrast, day or evening, and motion are in Reading settings, at the top." }
        }
        Card { id: "egg-log".to_string(),
            h3 { "Past work · before this hall" }
            ul { class: "card-log",
                for e in EARLIER {
                    li {
                        if e.href.is_empty() { b { "{e.name}" } } else { a { href: "{e.href}", "{e.name}" } }
                        span { "{e.line}" }
                    }
                }
            }
            p { "The four Pursuit L2 builds also have their original write-ups, with measured numbers: "
                a { href: "https://nessaisling-lab.github.io/pursuit-l2-portfolio/", "pursuit-l2-portfolio →" }
            }
        }
    }
}

/// One build on show: its walkthrough and transcript, what it is, its spec sheet, and its model
/// on the turntable. Only `ON_SHOW` is visible until site.js shows another.
#[component]
fn Show(root: String, n: usize, slug: String) -> Element {
    let b = BUILDS.iter().find(|b| b.slug == slug).expect("build exists");
    let on = b.slug == ON_SHOW;
    let spoken = b.media.as_ref().and_then(|m| m.captions).map(|c| {
        transcript(&std::fs::read_to_string(site_root().join(c)).expect("captions file exists"))
    });
    let model = plate_model(b.slug);
    rsx! {
        article { class: "show", id: "show-{b.slug}", hidden: !on, "aria-labelledby": "show-{b.slug}-h",
            div { class: "screen",
                if let Some(m) = &b.media {
                    div { class: "reel",
                        BuildVideo { root: root.clone(), slug: b.slug.to_string() }
                        // A big play button over the poster; site.js shows it. Without script the
                        // video's own controls do the job.
                        if !m.autoplay {
                            button { class: "play", "type": "button", hidden: true, "aria-label": "Play the walkthrough" }
                        }
                    }
                } else {
                    div { class: "no-reel",
                        p { "No walkthrough for this one yet. The build page has the whole story." }
                    }
                }
            }
            // The walkthrough's words, opened from the floor of the hall.
            if let Some(paras) = &spoken {
                details { class: "transcript",
                    summary { span { class: "tx-ico", "aria-hidden": "true" } "Transcript" }
                    div { class: "tx", tabindex: "0", role: "region", "aria-label": "Walkthrough transcript",
                        for (t, para) in paras {
                            p { button { class: "ts", "type": "button", "data-t": "{t}", "aria-label": "Play from {clock(*t)}", "{clock(*t)}" } " {para}" }
                        }
                    }
                }
            }
            div { class: "hud about",
                p { class: "bay", "Bay " b { "{n:02}" } " / {BUILDS.len():02}" }
                // More about the build: a card with where each of its links goes.
                button { class: "more", "type": "button", "popovertarget": "card-{b.slug}", "aria-label": "More about {b.name}",
                    span { class: "ico", dangerous_inner_html: ICON_MORE }
                }
                // The name opens the case study, in one click.
                h2 { id: "show-{b.slug}-h", tabindex: "-1",
                    a { href: "{root}work/{b.slug}/", "{b.name}" span { class: "go", "aria-hidden": "true", "\u{A0}→" } }
                }
                p { class: "kick", "{b.kicker} · {b.year}" }
                p { class: "line", "{b.line}" }
                p { class: "role", "{b.role}" }
            }
            aside { class: "hud spec-sheet", "aria-label": "Spec",
                p { class: "spec-h", "Spec" }
                // The three headline figures; the rest, and the whole stack, are on its card.
                for f in b.figs.iter().take(3) {
                    div { class: "sfig",
                        b { "{f.v}" }
                        span { "{f.k}" }
                        if let Some(pct) = ratio(f.v) {
                            span { class: "sbar", "aria-hidden": "true", i { style: "width:{pct:.1}%" } }
                        }
                    }
                }
                button { class: "all-specs", "type": "button", "popovertarget": "specs-{b.slug}", "All specs" span { "aria-hidden": "true", " +" } }
            }
            if let Some((file, _)) = model {
                div { class: "turntable", "aria-hidden": "true", "data-model": "{root}brand/library/{file}-model.json",
                    img { class: "turn", src: "{root}brand/library/{file}-turn-lg.webp", alt: "", width: "192", height: "192",
                        loading: if on { "eager" } else { "lazy" } }
                }
            }
        }
    }
}

/// A figure written as "a / b" as a share of 100, for its meter bar. Other figures get no bar:
/// a bar needs a scale, and "250 buildings" has none.
fn ratio(v: &str) -> Option<f64> {
    let (a, b) = v.split_once('/')?;
    let num = |s: &str| s.trim().replace(',', "").parse::<f64>().ok();
    let (a, b) = (num(a)?, num(b)?);
    (b > 0.0 && a <= b).then(|| a / b * 100.0)
}

/// A build's video: a silent build loop plays while on screen (if motion is allowed); a
/// narrated walkthrough never autoplays and has its captions on.
#[component]
fn BuildVideo(root: String, slug: String) -> Element {
    let b = BUILDS.iter().find(|b| b.slug == slug).expect("build exists");
    let m = b.media.as_ref().expect("build has media");
    rsx! {
        if m.autoplay {
            video {
                src: "{root}{m.video}",
                poster: "{root}{m.poster}",
                width: "{m.w}",
                height: "{m.h}",
                // Boolean attributes take their own name as the value: dioxus-ssr writes a Rust
                // `true` as `controls=true`, which HTML does not allow.
                controls: "controls",
                muted: "muted",
                "loop": "loop",
                playsinline: "playsinline",
                preload: "none",
                "aria-label": "{m.alt}",
                "data-autoplay": "true",
            }
        } else {
            video {
                src: "{root}{m.video}",
                poster: "{root}{m.poster}",
                width: "{m.w}",
                height: "{m.h}",
                controls: "controls",
                playsinline: "playsinline",
                preload: "none",
                "aria-label": "{m.alt}",
                if let Some(c) = m.captions {
                    track { kind: "captions", src: "{root}{c}", srclang: "en", label: "English", default: "default" }
                }
            }
        }
    }
}

// ── case study ───────────────────────────────────────────────────────────────

#[component]
fn Case(root: String, i: usize) -> Element {
    let b = &BUILDS[i];
    let prev = &BUILDS[(i + BUILDS.len() - 1) % BUILDS.len()];
    let next = &BUILDS[(i + 1) % BUILDS.len()];
    // The video is fig. 1 when there is one, so the stills number on from it.
    let first_still = if b.media.is_some() { 2 } else { 1 };
    // Read when the page renders, from the same committed file the <track> serves.
    let spoken = b.media.as_ref().and_then(|m| m.captions).map(|c| {
        transcript(&std::fs::read_to_string(site_root().join(c)).expect("captions file exists"))
    });
    rsx! {
        article { class: "wrap case",
            nav { class: "crumbs", "aria-label": "Breadcrumb",
                a { href: "{root}work/", "Projects" }
                span { "aria-hidden": "true", " / " }
                span { "aria-current": "page", "{b.slug}" }
            }
            header { class: "case-head",
                h1 { "{b.name}" }
                p { class: "case-meta", "{b.kicker} · {b.year}" }
                p { class: "deck", "{b.line}" }
                p { class: "status-line",
                    span { class: "dot", "aria-hidden": "true" }
                    "{b.status}"
                }
            }

            div { class: "case-grid",
                aside { class: "spec", "aria-label": "Project facts",
                    div { class: "figs",
                        for f in b.figs {
                            div { class: "fig",
                                span { class: "fig-v", "{f.v}" }
                                span { class: "fig-k", "{f.k}" }
                            }
                        }
                    }
                    // Open by default; site.js folds it on phones so the video comes first.
                    details { class: "spec-more", open: "open",
                    summary { if b.team.is_empty() { "Role and stack" } else { "Role, team and stack" } }
                    dl { class: "spec-dl",
                        div { dt { "Role" } dd { "{b.role}" } }
                        if !b.team.is_empty() {
                            div {
                                dt { "With" }
                                dd {
                                    for (j, p) in b.team.iter().enumerate() {
                                        TeamMember { j: j, name: p.name.to_string(), handle: p.handle.to_string(), role: p.role.to_string() }
                                    }
                                }
                            }
                        }
                        div { dt { "Stack" } dd { class: "stack-chips",
                            for s in b.stack { span { class: "chip", "{s}" } }
                        } }
                    }
                    }
                    if !b.links.is_empty() {
                        div { class: "spec-links",
                            for l in b.links {
                                a { class: "btn small ghost", href: "{l.href}", "{l.label} →" }
                            }
                        }
                    }
                }

                div { class: "case-body",
                    if let Some(m) = &b.media {
                        figure { class: if m.w > m.h { "media wide" } else { "media tall" },
                            BuildVideo { root: root.clone(), slug: b.slug.to_string() }
                            figcaption { "fig. 1 · {m.alt}" }
                        }
                    }
                    // The narration as text, for readers who can't or won't play the video.
                    if let Some(paras) = &spoken {
                        details { class: "transcript",
                            summary { "Transcript" }
                            for (t, para) in paras {
                                p { button { class: "ts", "type": "button", "data-t": "{t}", "aria-label": "Play from {clock(*t)}", "{clock(*t)}" } " {para}" }
                            }
                        }
                    }
                    if GALLERY.iter().any(|s| s.slug == b.slug) {
                        div { class: "gallery",
                            for (n, s) in GALLERY.iter().filter(|s| s.slug == b.slug).enumerate() {
                                GalleryFig { n: n + first_still, src: format!("{root}{}", s.src), alt: s.alt.to_string(), caption: s.caption.to_string() }
                            }
                        }
                    }
                    for k in 0..b.sections.len() {
                        CaseSection { i: i, k: k }
                    }
                    if let Some((lead, cmd)) = b.verify {
                        div { class: "verify",
                            h2 { class: "verify-h", "Verify it yourself" }
                            p { "{lead}" }
                            // Focusable and named because it scrolls sideways on narrow screens.
                            pre { tabindex: "0", role: "region", "aria-label": "Command to verify it yourself",
                                code { "{cmd}" }
                            }
                        }
                    }
                    if let Some(n) = b.note {
                        p { class: "note", "{n}" }
                    }
                }
            }

            nav { class: "pager", "aria-label": "More builds",
                a { class: "pager-prev", href: "{root}work/{prev.slug}/",
                    span { class: "pager-k", "← previous" }
                    span { class: "pager-n", "{prev.name}" }
                }
                a { class: "pager-next", href: "{root}work/{next.slug}/",
                    span { class: "pager-k", "next →" }
                    span { class: "pager-n", "{next.name}" }
                }
            }
        }
    }
}

#[component]
fn GalleryFig(n: usize, src: String, alt: String, caption: String) -> Element {
    rsx! {
        figure {
            // No aria-label: the image's alt text is the link's name.
            a { href: "{src}",
                img { src: "{src}", alt: "{alt}", width: "1600", height: "900", loading: "lazy", decoding: "async" }
            }
            figcaption { "fig. {n} · {caption}" }
        }
    }
}

#[component]
fn TeamMember(j: usize, name: String, handle: String, role: String) -> Element {
    let sep = if j > 0 { ", " } else { "" };
    rsx! {
        "{sep}"
        a { href: "https://github.com/{handle}", "{name}" }
        if !role.is_empty() {
            span { class: "quiet", " ({role})" }
        }
    }
}

#[component]
fn CaseSection(i: usize, k: usize) -> Element {
    let s = &BUILDS[i].sections[k];
    let num = format!("{:02}", k + 1);
    rsx! {
        section { class: "cs",
            h2 {
                span { class: "cs-n", "aria-hidden": "true", "{num}" }
                "{s.title}"
            }
            p { class: if s.points.is_empty() { "cs-lead first" } else { "cs-lead" }, "{s.lead}" }
            if !s.points.is_empty() {
                ul {
                    for p in s.points {
                        li { "{p}" }
                    }
                }
            }
        }
    }
}

// ── integration ──────────────────────────────────────────────────────────────

#[component]
fn Integration(root: String) -> Element {
    rsx! {
        section { class: "wrap page-head",
            h1 { "Data integration" }
            p { class: "deck",
                "I connect the systems a team already pays for, so nobody retypes a list again. The proof
                 so far is a nonprofit's gala; the same work fits any small team that runs on Salesforce."
            }
        }

        section { class: "wrap block",
            h2 { class: "block-h", "The work" }
            div { class: "rules",
                for c in INTEGRATION {
                    div { class: "rule",
                        h3 { "{c.title}" }
                        p { "{c.covers}" }
                        p { "{c.leaves}" }
                    }
                }
            }
        }

        section { class: "wrap block",
            h2 { class: "block-h", "How I work" }
            ul { class: "integration-rules",
                for r in INTEGRATION_RULES {
                    li { "{r}" }
                }
            }
        }

        section { class: "wrap block",
            h2 { class: "block-h", "Proof" }
            p { class: "deck",
                "The gala capstone: a ticketing-platform import that keeps staff corrections, and a
                 Salesforce sync tested against a stand-in org."
            }
            p { class: "more",
                a { class: "text-link", href: "{root}work/gala-guest-list/", "Read the gala case study →" }
            }
        }

        section { class: "wrap block cta",
            h2 { class: "block-h", "Have systems that don't talk to each other?" }
            div { class: "actions",
                a { class: "nb-btn is-primary", href: "{root}contact/", "Talk about a project" }
            }
        }
    }
}

// ── experience, school, contact ──────────────────────────────────────────────
//
// The three drawers of the home desk. Each gets its own world later (asked as we reach
// it); for now they share the kit's spec-sheet look.

/// One job on the sky: its place in the list, the job, where it crosses the meridian (as CSS),
/// and its facts with where each sits along the track.
type SkyJob = (usize, &'static Job, String, Vec<(String, &'static str)>);

/// Experience is an-ki, heaven and earth: where I've worked is plotted on the sky (drawn by
/// sky.rs), the stack stands on the horizon as a ziggurat, and how I work is the ground.
#[component]
fn Experience(root: String) -> Element {
    let jobs: Vec<SkyJob> = JOBS.iter().enumerate().map(|(i, j)| {
        let y = format!("--y:{:.2}%", sky::crossing(i) / sky::H * 100.0);
        let facts = job_facts(j.org).iter().enumerate().map(|(k, f)| {
            let (x, y) = sky::fact_at(i, k);
            (format!("--fx:{x:.2}%;--fy:{y:.2}%"), *f)
        }).collect();
        (i, j, y, facts)
    }).collect();
    rsx! {
        section { class: "sky", "aria-labelledby": "exp-h",
            div { class: "chart",
                div { class: "sky-draw", dangerous_inner_html: sky::svg(JOBS.len()) }
                div { class: "chart-head",
                    h1 { id: "exp-h", "Experience" }
                    p { class: "deck",
                        "A client capstone, a terminal I'm building solo, data work, and a business I ran on
                         my own. The same habit runs through all of it: own the thing end to end."
                    }
                }
                button { class: "mer-label", "type": "button", "popovertarget": "egg-ziqpu", "The meridian" }
                button { class: "glyph an", "type": "button", "popovertarget": "egg-anki", "aria-label": "an, Sumerian for the sky", "\u{1202D}" }
                // Each job crosses the meridian at its star; press the name for the whole story.
                ol { class: "jobs",
                    for (i, j, y, facts) in jobs {
                        li { class: if i % 2 == 0 { "job r" } else { "job l" }, style: "{y}",
                            span { class: "star", "aria-hidden": "true" }
                            span { class: "mul", "aria-hidden": "true", "mul · star" }
                            div { class: "job-label",
                                p { class: "when", "{j.when}" }
                                h2 { class: "org",
                                    button { "type": "button", "popovertarget": "job-{i}", "{j.org}" }
                                }
                                p { class: "role", "{j.role}" }
                            }
                            ul { class: "facts",
                                for (style, f) in facts {
                                    li { style: "{style}", "{f}" }
                                }
                            }
                        }
                    }
                }
                Ziggurat {}
                StackSteps {}
            }
        }
        section { class: "earth", "aria-labelledby": "how-h",
            div { class: "wrap earth-in",
                div { class: "earth-head",
                    h2 { id: "how-h", "How I work" }
                    button { class: "glyph ki", "type": "button", "popovertarget": "egg-anki", "aria-label": "ki, Sumerian for the earth", "\u{121A0}" }
                }
                div { class: "earth-rules",
                    for r in RULES {
                        div { class: "earth-rule",
                            h3 { "{r.head}" }
                            p { "{r.body}" }
                        }
                    }
                }
                p { class: "more", a { href: "{root}resume/", "The full résumé →" } }
            }
        }
        for (i, j) in JOBS.iter().enumerate() {
            Card { id: format!("job-{i}"),
                p { class: "card-k", "{j.when}" }
                h3 { "{j.org}" }
                p { class: "card-role", "{j.role}" }
                p { "{j.body}" }
            }
        }
        for t in TOOLS {
            ToolCard { root: root.clone(), name: t.name.to_string() }
        }
        Card { id: "egg-anki".to_string(),
            p { class: "card-glyph", "aria-hidden": "true", "\u{1202D}\u{121A0}" }
            h3 { span { class: "nb-place", lang: "sux-Latn", "an-ki" } " · heaven and earth" }
            p {
                "The Sumerian word for the universe was an-ki: "
                span { class: "nb-place", lang: "sux-Latn", "an" }
                ", the sky, and "
                span { class: "nb-place", lang: "sux-Latn", "ki" }
                ", the earth. This page is laid out the same way: where I've worked on the sky, how I work on the ground under it."
            }
        }
        Card { id: "egg-e2".to_string(),
            p { class: "card-glyph", "aria-hidden": "true", "\u{1208D}" }
            h3 { span { class: "nb-place", lang: "sux-Latn", "é" } " · house, temple" }
            p { "A ziggurat was a stepped platform with a shrine at the top. In Sumerian, é is a house, and a temple: the god's house." }
        }
        Card { id: "egg-ziqpu".to_string(),
            h3 { span { class: "nb-place", "ziqpu" } " · highest point" }
            p {
                "Babylonian astronomers kept a list of ziqpu stars and timed them as they crossed the meridian
                 overhead, to tell the hour of the night. My project "
                a { href: "{root}work/ziqpu/", "Ziqpu" }
                " shares the name."
            }
        }
    }
}

/// Icons for the hall's buttons and the build cards' links, drawn in the text colour.
const ICON_KEYS: &str = r#"<svg viewBox="0 0 24 24" aria-hidden="true"><path fill-rule="evenodd" d="M3 6h18a1 1 0 0 1 1 1v10a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1zm1 2v8h16V8zm1 1h2v2H5zm3 0h2v2H8zm3 0h2v2h-2zm3 0h2v2h-2zm3 0h2v2h-2zM5 12h2v2H5zm3 0h8v2H8zm9 0h2v2h-2z"/></svg>"#;
const ICON_MORE: &str = r#"<svg viewBox="0 0 24 24" aria-hidden="true"><path fill-rule="evenodd" d="M12 2a10 10 0 1 1 0 20 10 10 0 0 1 0-20zm0 2a8 8 0 1 0 0 16 8 8 0 0 0 0-16zm-1 3.5h2v2h-2zm0 3.5h2v6h-2z"/></svg>"#;
const ICON_CASE: &str = r#"<svg viewBox="0 0 24 24" aria-hidden="true"><path fill-rule="evenodd" d="M6 2h9l5 5v15H6zm2 2v16h10V8h-4V4zm1 8h8v1.6H9zm0 3.5h8v1.6H9z"/></svg>"#;
const ICON_DOWN: &str = r#"<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M11 3h2v9.2l3.3-3.3 1.4 1.4L12 16l-5.7-5.7 1.4-1.4 3.3 3.3zM4 18h16v2H4z"/></svg>"#;
const ICON_OUT: &str = r#"<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M14 3h7v7h-2V6.4l-8.3 8.3-1.4-1.4L17.6 5H14zM5 5h6v2H7v10h10v-4h2v6H5z"/></svg>"#;
/// GitHub's mark (Octicons, MIT).
const ICON_GITHUB: &str = r#"<svg viewBox="0 0 16 16" aria-hidden="true"><path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0 0 16 8c0-4.42-3.58-8-8-8z"/></svg>"#;

/// The hall's two cards for one build: more about it, with each place its links go (an icon
/// and the site's name, so nothing is a surprise), and its whole spec sheet.
#[component]
fn BuildCards(root: String, slug: String) -> Element {
    let b = BUILDS.iter().find(|b| b.slug == slug).expect("build exists");
    let host = |h: &str| h.split("//").nth(1).and_then(|r| r.split('/').next()).unwrap_or(h).trim_start_matches("www.").to_string();
    let icon = |l: &Link| if l.label.starts_with("Download") { ICON_DOWN } else if l.href.contains("github.com") { ICON_GITHUB } else { ICON_OUT };
    rsx! {
        Card { id: format!("card-{}", b.slug),
            p { class: "card-kick", "{b.kicker} · {b.year}" }
            h3 { "{b.name}" }
            p { "{b.line}" }
            p { class: "card-role", "{b.role}" }
            ul { class: "card-links",
                li { a { href: "{root}work/{b.slug}/",
                    span { class: "ico", dangerous_inner_html: ICON_CASE } span { class: "lbl", "Case study" } span { class: "where", "this site" }
                } }
                for l in b.links {
                    li { a { href: "{l.href}",
                        span { class: "ico", dangerous_inner_html: icon(l) } span { class: "lbl", "{l.label}" } span { class: "where", "{host(l.href)}" }
                    } }
                }
            }
        }
        Card { id: format!("specs-{}", b.slug),
            p { class: "card-kick", "Spec" }
            h3 { "{b.name}" }
            dl { class: "spec-all",
                for f in b.figs { div { dt { "{f.v}" } dd { "{f.k}" } } }
            }
            p { class: "schips", for c in b.stack { span { "{c}" } } }
            p { "More in the case study: " a { href: "{root}work/{b.slug}/", "{b.name} →" } }
        }
    }
}

/// A card that opens over the page: the HTML popover, so it needs no script, closes on Esc or
/// a click outside, and comes back to the button that opened it.
#[component]
fn Card(id: String, children: Element) -> Element {
    rsx! {
        div { class: "card", id: "{id}", "popover": "auto",
            button { class: "card-x", "type": "button", "popovertarget": "{id}", "popovertargetaction": "hide", "aria-label": "Close", "×" }
            {children}
        }
    }
}

/// A tool's card: what it is, and the builds whose pages show it in use.
#[component]
fn ToolCard(root: String, name: String) -> Element {
    let t = tool(&name).expect("tool exists");
    let used: Vec<&Build> = t.used.iter().filter_map(|(s, _)| BUILDS.iter().find(|b| b.slug == *s)).collect();
    rsx! {
        Card { id: tool_id(t.name),
            h3 { "{t.name}" }
            p { "{t.what}" }
            if !used.is_empty() {
                p { class: "card-used",
                    "Used in "
                    for (k, b) in used.iter().enumerate() {
                        if k > 0 { if k + 1 == used.len() { " and " } else { ", " } }
                        a { href: "{root}work/{b.slug}/", "{b.name}" }
                    }
                }
            }
        }
    }
}

/// The ziggurat's layers, foundation first: Rust at the bottom, AI at the summit.
const ZIG: [&str; 7] = ["Rust", "Integrity", "Integration", "Web", "Ops", "Also", "AI"];

/// Spreads a layer's tools over the ziggurat's four sides by length, so its band reads evenly
/// all the way round.
fn four_sides<'a>(items: &[&'a str]) -> [Vec<&'a str>; 4] {
    let total: usize = items.iter().map(|s| s.len() + 3).sum();
    let mut out: [Vec<&str>; 4] = Default::default();
    let (mut side, mut run) = (0, 0);
    for it in items {
        if side < 3 && run * 4 >= total * (side + 1) {
            side += 1;
        }
        out[side].push(it);
        run += it.len() + 3;
    }
    out
}

/// The stack as a ziggurat, in CSS 3D: seven terraces, each with its tools written round its
/// four sides, and a shrine on top. site.js turns it; every tool is a button that opens its card.
#[component]
fn Ziggurat() -> Element {
    rsx! {
        div { class: "zig", tabindex: "0", role: "group",
            "aria-label": "My stack as a ziggurat, Rust at the foundation, AI at the top; each tool opens its card.",
            div { class: "model",
                for (i, name) in ZIG.iter().enumerate() {
                    Tier { i, name: name.to_string() }
                }
                Shrine {}
            }
        }
    }
}

const TIER_W: f64 = 22.5; // em, the foundation's width
const TIER_STEP: f64 = 1.875; // em narrower per terrace
const TIER_H: f64 = 2.125; // em tall

#[component]
fn Tier(i: usize, name: String) -> Element {
    let row = STACK.iter().find(|r| r.dt == name).expect("stack row exists");
    let items: Vec<&str> = row.dd.split(" · ").collect();
    let w = TIER_W - TIER_STEP * i as f64;
    let tier = format!("--i:{i};--w:{w}em;transform:translateY(-{}em)", TIER_H * (i + 1) as f64);
    let sides: Vec<(usize, String, Vec<&str>)> = four_sides(&items).into_iter().enumerate()
        .map(|(k, s)| (k, format!("transform:rotateY({}deg) translateZ({}em)", k * 90, w / 2.0), s))
        .collect();
    rsx! {
        div { class: "tier", style: "{tier}",
            for (k, style, side) in sides {
                div { class: "face f{k}", style: "{style}",
                    div { class: "band",
                        if k == 0 { b { class: "layer", "{name}" } }
                        if side.is_empty() { span { class: "layer-name", "aria-hidden": "true", "{name}" } }
                        for (n, t) in side.iter().enumerate() {
                            if n > 0 { span { class: "sep", "aria-hidden": "true", "·" } }
                            button { class: "tool", "type": "button", "popovertarget": tool_id(t), "{t}" }
                        }
                    }
                }
            }
            div { class: "cap" }
        }
    }
}

/// The shrine on the summit, é, the god's house: a small temple with the niched walls of a
/// Sumerian temple and an arched door. The door is the easter egg, like the tablet at home.
#[component]
fn Shrine() -> Element {
    let (w, h) = (4.5, 2.6);
    let tier = format!("--i:7;--w:{w}em;--h:{h}em;transform:translateY(-{}em)", TIER_H * 7.0 + h);
    let sides: Vec<(usize, String)> = (0..4).map(|k| (k, format!("transform:rotateY({}deg) translateZ({}em)", k * 90, w / 2.0))).collect();
    rsx! {
        div { class: "tier shrine", style: "{tier}",
            for (k, style) in sides {
                div { class: "face f{k}", style: "{style}",
                    if k == 0 {
                        button { class: "door", "type": "button", "popovertarget": "egg-e2", "aria-label": "The shrine's door: é, the god's house" }
                    } else {
                        button { class: "door", "type": "button", "popovertarget": "egg-e2", tabindex: "-1", "aria-hidden": "true" }
                    }
                }
            }
            div { class: "cap" }
        }
    }
}

/// Narrow screens: the ziggurat as low terraces under the shrine, AI at the top and Rust at the
/// foundation. Each terrace is a collapsible that opens to its tools, one at a time, so it never
/// stands taller than the shrine and seven names.
#[component]
fn StackSteps() -> Element {
    rsx! {
        div { class: "steps", role: "group", "aria-label": "My stack, from AI at the top to Rust at the foundation",
            div { class: "shrine-flat",
                button { class: "door", "type": "button", "popovertarget": "egg-e2", "aria-label": "The shrine's door: é, the god's house" }
            }
            for (i, name) in ZIG.iter().enumerate().rev() {
                StepTier { i, name: name.to_string() }
            }
        }
    }
}

#[component]
fn StepTier(i: usize, name: String) -> Element {
    let row = STACK.iter().find(|r| r.dt == name).expect("stack row exists");
    let items: Vec<&str> = row.dd.split(" · ").collect();
    let count = format!("{} tools", items.len());
    rsx! {
        details { class: "step", "name": "stack", style: "--i:{i}",
            summary { span { class: "step-name", "{name}" } span { class: "step-count", "{count}" } }
            div { class: "step-tools",
                for t in items {
                    button { class: "tool", "type": "button", "popovertarget": tool_id(t), "{t}" }
                }
            }
        }
    }
}

#[component]
fn School(root: String) -> Element {
    rsx! {
        section { class: "wrap page-head",
            h1 { "School" }
            p { class: "deck",
                "Pursuit's AI-Native Builder program, Per Scholas IT Support, and aircraft-mechanic
                 coursework at Vaughn College."
            }
        }
        section { class: "wrap block", "aria-labelledby": "schools",
            h2 { id: "schools", class: "block-h", "Programs and certificates" }
            Timeline { education: true }
            p { class: "more", a { href: "{root}work/", "What I built at Pursuit →" } }
        }
    }
}

#[component]
fn Contact(root: String) -> Element {
    rsx! {
        section { class: "wrap page-head",
            h1 { "Contact" }
            p { class: "deck", "The fastest way to reach me is email. The form below sends me one too." }
        }
        section { class: "wrap block", "aria-labelledby": "write",
            h2 { id: "write", class: "visually-hidden", "Write to me" }
            div { class: "contact",
                div { class: "contact-direct",
                    p { class: "contact-mail", a { href: "mailto:{EMAIL}", "{EMAIL}" } }
                    p {
                        a { href: "https://www.linkedin.com/in/stan-leiva-davila/", "LinkedIn" }
                        " · "
                        a { href: "https://github.com/nessaisling-lab", "github.com/nessaisling-lab" }
                        " · "
                        a { href: "https://github.com/aislingld-pursuit", "github.com/aislingld-pursuit" }
                        " (Pursuit cohort work)"
                    }
                }
                form { class: "form", id: "contactForm", action: "https://api.web3forms.com/submit", method: "POST",
                    input { "type": "hidden", name: "access_key", value: "{FORM_KEY}" }
                    input { "type": "hidden", name: "subject", value: "New message from the portfolio" }
                    input { "type": "hidden", name: "from_name", value: "Portfolio" }
                    input { class: "hp", "type": "checkbox", name: "botcheck", tabindex: "-1", autocomplete: "off" }
                    label { r#for: "cf-name", "Name" }
                    input { id: "cf-name", "type": "text", name: "name", autocomplete: "name", required: "required" }
                    label { r#for: "cf-email", "Email" }
                    input { id: "cf-email", "type": "email", name: "email", autocomplete: "email", required: "required" }
                    label { r#for: "cf-msg", "Message" }
                    textarea { id: "cf-msg", name: "message", rows: "5", required: "required" }
                    button { class: "nb-btn is-primary", "type": "submit", "Send" }
                    p { class: "form-status", id: "formStatus", role: "status", "aria-live": "polite" }
                }
            }
        }
    }
}

/// The old About page. Its story is on Home now; site.js forwards old #anchors.
#[component]
fn Moved(root: String) -> Element {
    rsx! {
        section { class: "wrap page-head moved",
            h1 { "About moved" }
            p { class: "deck", "My story is on the home page now. You might be looking for:" }
            div { class: "actions",
                a { class: "nb-btn is-primary", href: "{root}", "Home" }
                a { class: "nb-btn", href: "{root}experience/", "Experience" }
                a { class: "nb-btn", href: "{root}school/", "School" }
                a { class: "nb-btn", href: "{root}contact/", "Contact" }
            }
        }
    }
}

#[component]
fn Timeline(education: bool) -> Element {
    let rows = if education { SCHOOLS } else { JOBS };
    rsx! {
        ol { class: "timeline",
            for r in rows {
                li {
                    span { class: "tl-when", "{r.when}" }
                    div {
                        h3 { "{r.org} " span { class: "quiet", "· {r.role}" } }
                        p { "{r.body}" }
                    }
                }
            }
        }
    }
}

// ── résumé ───────────────────────────────────────────────────────────────────

#[component]
fn Resume(root: String) -> Element {
    rsx! {
        section { class: "wrap page-head",
            h1 { "Résumé" }
            p { class: "deck", "One page, in Letter or Legal." }
            div { class: "actions",
                a { class: "btn", href: "{root}assets/docs/resume/resume-letter.pdf", "Download PDF (Letter)" }
            }
        }
        section { class: "wrap block",
            Cv {}
        }
        section { class: "wrap block",
            h2 { class: "block-h", "Documents" }
            ul { class: "docs",
                for d in DOCS {
                    li {
                        span { class: "doc-name", "{d.name}" }
                        span { class: "doc-files",
                            for f in d.files {
                                a { class: "chip link", href: "{root}{f.1}", "{f.0}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// The résumé as a page, from the same data as the PDF and Word files.
#[component]
fn Cv() -> Element {
    rsx! {
        article { class: "cv",
            p { class: "cv-summary", "{resume::SUMMARY}" }
            h2 { "Experience" }
            for e in resume::EXPERIENCE { CvEntry { e: e } }
            h2 { "Technical projects" }
            for p in resume::PROJECTS {
                div { class: "cv-entry",
                    div { class: "cv-row",
                        h3 { a { href: "{p.url}", "{p.name}" } span { class: "quiet", " · {p.kind}" } }
                        span { class: "cv-when", "{p.year}" }
                    }
                    p { class: "cv-desc", "{p.desc}" }
                    ul { for b in p.bullets { li { "{b}" } } }
                }
            }
            h2 { "Education" }
            for e in resume::EDUCATION { CvEntry { e: e } }
            h2 { "Skills" }
            dl { class: "cv-skills",
                for (k, v) in resume::SKILLS {
                    div { dt { "{k}" } dd { "{v}" } }
                }
            }
        }
    }
}

#[component]
fn CvEntry(e: &'static resume::Entry) -> Element {
    if e.compact {
        let rest = e.compact_rest();
        let right = e.compact_right();
        return rsx! {
            div { class: "cv-entry compact",
                div { class: "cv-row",
                    h3 { "{e.org}" span { class: "quiet", " · {rest}" } }
                    span { class: "cv-when", "{right}" }
                }
            }
        };
    }
    rsx! {
        div { class: "cv-entry",
            div { class: "cv-row",
                h3 { "{e.org}" span { class: "quiet", " · {e.role}" } }
                span { class: "cv-when", "{e.when}" }
            }
            p { class: "cv-desc", "{e.desc}" span { class: "quiet", " · {e.place}" } }
            if !e.bullets.is_empty() {
                ul { for b in e.bullets { li { "{b}" } } }
            }
        }
    }
}

#[component]
fn NotFound(root: String) -> Element {
    rsx! {
        section { class: "wrap page-head",
            h1 { "Not found" }
            p { class: "deck", "No such page. It may have moved when the site went multi-page." }
            div { class: "actions",
                a { class: "btn", href: "{root}", "Home" }
                a { class: "nb-btn", href: "{root}work/", "Projects" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::transcript;

    /// Header, NOTE and cue ids are dropped; a pause after a full stop starts a paragraph,
    /// a pause mid-sentence does not.
    #[test]
    fn program_order_has_every_build() {
        use crate::{BUILDS, PROGRAM};
        let mut p: Vec<&str> = PROGRAM.to_vec();
        p.sort();
        let mut b: Vec<&str> = BUILDS.iter().map(|b| b.slug).collect();
        b.sort();
        assert_eq!(p, b, "Program order must name every build once, or sorting by it drops one");
    }

    #[test]
    fn transcript_joins_cues_into_paragraphs() {
        let vtt = "WEBVTT\r\n\r\nNOTE made by hand\r\n\r\n1\r\n00:00:00.500 --> 00:00:02.000\r\nRight now,\r\n\r\n\
                   00:00:03.500 --> 00:00:05.000 line:90%\r\nit is one sentence.\r\n\r\n\
                   00:00:05.050 --> 00:00:06.000\r\nStill the first.\r\n\r\n\
                   00:01:07.000 --> 00:01:08.000\r\nA new beat\r\non two lines.\r\n";
        assert_eq!(
            transcript(vtt),
            [(0.5, "Right now, it is one sentence. Still the first.".to_string()), (67.0, "A new beat on two lines.".to_string())]
        );
    }
}
