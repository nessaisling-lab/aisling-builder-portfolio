//! Everything the site says, as data. Layout lives in main.rs; facts live here, so a
//! new build is one entry in `BUILDS` rather than edits in five places.
//!
//! Sources: the L2 portfolio READMEs, the résumé, and the capstone handoff. Numbers are
//! the measured ones from those documents, with the date they were measured where the
//! source gives one. Nothing here is an estimate dressed as a measurement.

#[derive(PartialEq)]
pub struct Fig {
    pub v: &'static str,
    pub k: &'static str,
}

#[derive(PartialEq)]
pub struct Section {
    pub title: &'static str,
    pub lead: &'static str,
    pub points: &'static [&'static str],
}

#[derive(PartialEq)]
pub struct Link {
    pub label: &'static str,
    pub href: &'static str,
}

#[derive(PartialEq)]
pub struct Person {
    pub name: &'static str,
    pub handle: &'static str,
    pub role: &'static str,
}

#[derive(PartialEq)]
pub struct Media {
    /// Paths relative to the site root.
    pub video: &'static str,
    pub poster: &'static str,
    pub alt: &'static str,
    pub w: u32,
    pub h: u32,
    /// WebVTT captions, for recordings with narration.
    pub captions: Option<&'static str>,
    /// Silent loops play while on screen; narrated walkthroughs wait for the reader.
    pub autoplay: bool,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Group {
    /// Rust, and a guarantee the program makes.
    Integrity,
    /// Team and client work.
    Team,
}

#[derive(PartialEq)]
pub struct Build {
    pub slug: &'static str,
    pub name: &'static str,
    pub kicker: &'static str,
    pub year: &'static str,
    pub group: Group,
    /// One line, used on cards and in the page description.
    pub line: &'static str,
    /// The single hardest number, set as a readout on cards.
    pub stat: &'static str,
    pub role: &'static str,
    pub team: &'static [Person],
    pub status: &'static str,
    pub figs: &'static [Fig],
    pub stack: &'static [&'static str],
    pub links: &'static [Link],
    pub media: Option<Media>,
    pub sections: &'static [Section],
    /// A shell command a reader can run to check a claim themselves.
    pub verify: Option<(&'static str, &'static str)>,
    pub note: Option<&'static str>,
    pub featured: bool,
}

pub const BUILDS: &[Build] = &[
    Build {
        slug: "housecheck",
        name: "HouseCheck",
        kicker: "Pursuit L2 · Cycle 4 capstone",
        year: "2026",
        group: Group::Integrity,
        line: "Carfax for NYC apartments: a Building Health Card scored from public city data, \
               with an export a stranger can verify offline.",
        stat: "176 tests · Ed25519 · live",
        role: "Lead builder · 192 of 201 commits",
        team: &[Person { name: "Antonin Lesov", handle: "antoninlesov", role: "frontend" }],
        status: "Live",
        figs: &[
            Fig { v: "176", k: "tests, clippy clean" },
            Fig { v: "250", k: "buildings served live" },
            Fig { v: "26,343", k: "violations stored" },
            Fig { v: "2.51 MB", k: "whole data artifact" },
            // Measured 2026-10-30: 50 warm GET /search calls over 5 addresses, median round trip
            // from New York to the live API (p10-p90: 49-81 ms).
            Fig { v: "60 ms", k: "search round trip, warm" },
            Fig { v: "9.89×", k: "description compression" },
        ],
        stack: &["Rust", "axum", "SQLite", "ed25519-dalek", "sha2", "TypeScript", "Docker", "Fly.io", "Vercel"],
        links: &[
            Link { label: "Live demo", href: "https://housecheck-wine.vercel.app" },
            Link { label: "Source", href: "https://github.com/nessaisling-lab/housecheck" },
            Link {
                label: "Export write-up",
                href: "https://github.com/nessaisling-lab/pursuit-l2-portfolio/tree/main/cycle-4-housecheck#the-export-is-the-hard-part",
            },
        ],
        media: Some(Media {
            video: "assets/work/housecheck/walkthrough.mp4",
            poster: "assets/work/housecheck/walkthrough-poster.jpg",
            alt: "Narrated walkthrough: the live app on a phone, from an address to its Building Health \
                  Card and export, then that export verified offline, tampered with, and forged",
            w: 1280,
            h: 720,
            captions: Some("assets/work/housecheck/walkthrough.en.vtt"),
            autoplay: false,
        }),
        sections: &[
            Section {
                title: "What it is",
                lead: "Before you sign a New York lease, the information that would tell you whether \
                       the building is maintained already exists, and it is public. It is also spread \
                       across four agency websites in formats designed for clerks. HouseCheck puts it \
                       on one card, scored 0–100 across condition, legal, neighborhood and \
                       accessibility, with every number linked to the dataset it came from.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "Six public datasets, one artifact, no database server.",
                points: &[
                    "HPD violations, 311, PLUTO, DOB, DOHMH and Census ACS, ingested with \
                     completeness checks and block-compressed per building, measured at 9.89×.",
                    "A read-only SQLite artifact baked into the container: no database server, no \
                     connection pool, no cold-start query.",
                    "Repair speed is the one measure of behaviour rather than state, with three \
                     states instead of two: a median, “nothing closed since 2023”, or genuinely no \
                     data.",
                ],
            },
            Section {
                title: "The hard part",
                lead: "Signing the export turned out not to be enough.",
                points: &[
                    "Each export carries an append-only hash chain, entry_hash[i] = sha256(entry_hash[i-1] \
                     ++ payload_hash[i]), with the head signed by Ed25519. Source dataset IDs and \
                     retrieval times sit inside the signed region.",
                    "Verification has three outcomes, not two: signed and intact, intact but unsigned, \
                     tampered. Collapsing the middle one would let an unsigned document pass as an \
                     authenticated one.",
                    "I wrote an independent verifier in Python, with a different crypto library and \
                     the document as its only input, and found a forgery path: a row rewritten to \
                     “NO VIOLATIONS OF ANY KIND AT THIS ADDRESS”, the chain recomputed and re-signed \
                     with the forger's own key, verified as intact.",
                    "The fix was publishing the signing key at a stable endpoint, separate from the \
                     documents it signs, so a reader has something independent to compare against.",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "Stated here rather than discovered later.",
                points: &[
                    "250 buildings, about 0.1% of the city.",
                    "Class I violations are excluded; 753 records were skipped at ingest, and the \
                     card says so.",
                    "It is a signal, not a legal ruling, and it does not give legal advice.",
                ],
            },
        ],
        verify: Some((
            "Every live figure is served by the app itself. The signing key is published apart from the documents it signs:",
            "curl -s https://housecheck-nessa.fly.dev/meta",
        )),
        note: Some(
            "The walkthrough records the live app, then runs HouseCheck's own verification code on \
             the file it downloaded: the terminal output is real. Narrated in a synthetic clone of my \
             own voice, made on my machine; made with Claude Code.",
        ),
        featured: false,
    },
    Build {
        slug: "gala-guest-list",
        name: "Gala Guest List Tool",
        kicker: "Pursuit L3 · client capstone",
        year: "2026",
        group: Group::Team,
        line: "One guest list for a NYC nonprofit's annual gala: a staff app, a private sponsor \
               portal, and an iPad check-in that keeps working offline.",
        stat: "180 of 223 PRs · 791 access-rule checks",
        role: "Technical director & repo admin",
        team: &[
            Person { name: "Natalie Walker", handle: "wsvdesign", role: "product lead, UI/UX lead" },
            Person { name: "Antonin Lesov", handle: "antoninlesov", role: "front end, UI/UX" },
            Person { name: "Cornell Robertson", handle: "CodeToTheCore", role: "quality assurance" },
        ],
        status: "Stable build · presented at Pursuit Demo Day, Nov 9, 2026",
        // Re-measured 2026-11-10 on the team repo's main: PRs from its history; checks from running
        // the access-rule suite (791), the app tests (315) and the import engine (157). Lines: Nov 5.
        figs: &[
            Fig { v: "180 / 223", k: "merged PRs authored" },
            Fig { v: "1,250+", k: "automated checks" },
            Fig { v: "791", k: "access-rule checks" },
            Fig { v: "15.6k", k: "lines in the build, generated files excluded" },
            Fig { v: "107", k: "dated decisions logged" },
            Fig { v: "3 · 2", k: "apps · environments" },
        ],
        stack: &[
            "Supabase (Postgres, Auth, Edge Functions)",
            "row-level security",
            "React",
            "TypeScript",
            "Vite",
            "Tailwind",
            "Vitest",
            "Playwright",
            "Cloudflare Pages + Access",
            "Salesforce REST API",
            "Claude Code",
        ],
        links: &[],
        media: Some(Media {
            video: "assets/work/gala-guest-list/walkthrough.mp4",
            poster: "assets/work/gala-guest-list/walkthrough-poster.jpg",
            alt: "Narrated walkthrough: one gala from setup to the thank-you notes, across the staff \
                  app, the sponsor portal and the iPad check-in, on invented data",
            w: 1280,
            h: 720,
            captions: Some("assets/work/gala-guest-list/walkthrough.en.vtt"),
            autoplay: false,
        }),
        sections: &[
            Section {
                title: "What it is",
                lead: "A nonprofit ran its annual gala across four disconnected sources: ticket \
                       sales, sponsorship pledges paid by check or bank transfer, comp tickets \
                       tracked by individual staff, and a shared spreadsheet. Sponsors emailed guest \
                       names that staff retyped, badges were mail-merged by hand, and the door ran \
                       on a printed list.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "Three apps over one Postgres database, built by a team of four.",
                points: &[
                    "Staff app: gala setup, sponsorship levels, a ticketing-platform import, orders \
                     with a payment ledger, tables, badges, a staff-approved email queue, CSV reports \
                     and a change log.",
                    "Sponsor portal: a private link per order, where a sponsor names and edits their \
                     guests until the deadline. No account needed.",
                    "iPad check-in: A-to-Z search, QR e-ticket scanning, walk-ins, and check-in that \
                     keeps working when the Wi-Fi drops and syncs when it returns.",
                ],
            },
            Section {
                title: "The hard part",
                lead: "As technical director I owned the backend, and most of it is about who can do what.",
                points: &[
                    "Row-level security in Postgres for three staff roles, a sign-in hook that admits \
                     only the staff list, and 6-digit email codes instead of passwords.",
                    "Sponsor links and QR e-tickets on random tokens of at least 128 bits, stored only \
                     as SHA-256 hashes, scoped to one order or one seat, and expiring at the deadline \
                     or after gala night.",
                    "An append-only change log of every edit (who, when, before and after) that only \
                     gala admins can read, and offline sync where the first arrival wins when two \
                     iPads check in the same guest.",
                    "When the team ran out of GitHub Actions minutes, a local CI fast lane: a full \
                     seven-stage run marks the commit on GitHub, so every PR still gets the complete \
                     check.",
                ],
            },
            Section {
                title: "Getting data in and out",
                lead: "Guests arrive from a ticketing platform and leave for Salesforce, and nobody \
                       retypes a list on the way.",
                points: &[
                    "A ticketing-platform import engine that matches each purchase to an existing \
                     record, keeps staff corrections through the next import, and records where every \
                     record came from.",
                    "A direct Salesforce sync over its REST API: one Account per sponsor, one \
                     Opportunity per order in the gala's Campaign, check, ACH and wire payments, and \
                     each guest as a Contact and Campaign Member, matched by email so none is \
                     duplicated.",
                    "Tested against a stand-in Salesforce org that refuses what Salesforce refuses. \
                     The client chose import files for now, so its Salesforce partner reviews every \
                     change first.",
                    "In progress on a private build, not part of the handoff: Opportunity Products, \
                     the client's own record types, ticket purchases arriving on their own, and a \
                     settings page.",
                ],
            },
            Section {
                title: "Demo Day and the handoff",
                lead: "Presented at Pursuit Demo Day at Scandinavia House on November 9, 2026, with the \
                       code and docs handed to the client on November 10.",
                points: &[
                    "Four plain-language study guides, for the backend, staff app, sponsor portal and \
                     iPad check-in, so anyone on the team can explain the build without overclaiming. \
                     Each has a map of the parts, step-by-step tours and a self-test, and each was \
                     fact-checked against the code.",
                    "The Demo Day deck: the team's animated HTML deck rendered frame by frame and \
                     encoded at 1080p into Google Slides with the videos inside, so every take is \
                     identical, at the rehearsed 3:34 and in a 3:00 cut.",
                    "A team slide of offline QR contact cards: one code per teammate that saves a full \
                     contact with no internet needed.",
                    "Supported the technical review by the client's web agency.",
                ],
            },
            Section {
                title: "Bugs the testing caught",
                lead: "Click-through tests of every button on a local copy found real defects.",
                points: &[
                    "A QR check-in that admitted the guest without saying who had come in.",
                    "Relationship managers receiving the whole change log, which only gala admins \
                     should read.",
                    "A payment form that let the same check, or an extra zero, through (from a \
                     teammate's review).",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "A working prototype, not production.",
                points: &[
                    "Every record in testing and in the walkthrough is invented. Production hosting \
                     and real data are the client's decision after the handoff.",
                    "The Salesforce sync has run only against a stand-in org, never the client's real \
                     Salesforce.",
                    "The apps sit behind a sign-in gate and the client stays unnamed here, so there is \
                     no public link. Numbers are as of November 2026.",
                ],
            },
        ],
        verify: None,
        note: Some(
            "Re-recorded for this portfolio on a private local copy with invented data: the app's \
             code is unchanged, and the client's name and logo are replaced at recording time with a \
             neutral stand-in. Narrated in a synthetic clone of my own voice, made on my machine; made \
             with Claude Code.",
        ),
        featured: true,
    },
    Build {
        slug: "resona",
        name: "Resona / Wisper",
        kicker: "Pursuit L2",
        year: "2026",
        group: Group::Integrity,
        line: "Privacy-first voice-to-text: 100% on-device via whisper.cpp, so the audio never \
               leaves the machine.",
        stat: "100% on-device · 4 GPU backends",
        role: "Builder · co-developed with Jimmy Ong",
        team: &[Person { name: "Jimmy Ong", handle: "jimmyronin", role: "co-developer" }],
        status: "Installable · v0.2.0-beta.29",
        figs: &[
            Fig { v: "100%", k: "on-device transcription" },
            Fig { v: "4", k: "GPU backends, one core" },
            Fig { v: "3", k: "desktop platforms" },
            Fig { v: "beta.29", k: "latest installer" },
        ],
        stack: &["Rust", "Tauri 2", "whisper.cpp", "whisper-rs", "cpal", "React", "TypeScript", "CUDA · Vulkan · Metal"],
        links: &[
            Link {
                label: "Download v0.2.0-beta.29",
                href: "https://github.com/aislingld-pursuit/L2-Clone-Prodject/releases/tag/v0.2.0-beta.29",
            },
            Link { label: "Source", href: "https://github.com/aislingld-pursuit/L2-Clone-Prodject" },
        ],
        media: Some(Media {
            video: "assets/work/resona/walkthrough.mp4",
            poster: "assets/work/resona/walkthrough-poster.jpg",
            alt: "Narrated walkthrough: the installed app transcribing a file on the GPU, a network check taken during \
                  that transcription, the editable transcript, and its export formats",
            w: 1280,
            h: 720,
            captions: Some("assets/work/resona/walkthrough.en.vtt"),
            autoplay: false,
        }),
        sections: &[
            Section {
                title: "What it is",
                lead: "Desktop dictation, live and from files, that runs entirely on the machine. \
                       The privacy claim is checkable by anyone in ten seconds: disconnect the \
                       network and it keeps transcribing.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "Dictation is the one input that is inherently sensitive.",
                points: &[
                    "Microphone capture through cpal, energy-based voice-activity detection, then \
                     incremental Whisper, so text appears as you speak.",
                    "One Rust core and one codebase for CUDA, Vulkan, Metal and CPU; Tauri 2 builds \
                     every target from the same source.",
                    "Tier gating lives in the Rust core with Ed25519-signed licences, so an \
                     entitlement check is not a frontend suggestion.",
                ],
            },
            Section {
                title: "The hard part",
                lead: "Making the privacy claim testable rather than promised.",
                points: &[
                    "Mainstream dictation streams audio to someone else's server, with a privacy \
                     policy as the only thing between the recording and a training set.",
                    "On first run it profiles the machine and recommends a model size, instead of \
                     making the user guess between tiny and large.",
                    "A security pass found and fixed an SSRF path and an unsafe export (SEC-001, \
                     SEC-002).",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "One defect kept visible on purpose.",
                points: &[
                    "The hardware probe reports 32041 GB RAM on a 32 GB machine: megabytes labelled as \
                     gigabytes. It stays documented rather than cropped out, because it is the class \
                     of bug that survives by looking like a big impressive number.",
                ],
            },
        ],
        verify: None,
        note: Some(
            "Recorded from the installed app. The network check in the video is unedited: Resona's own process \
             opened no connections while transcribing; Microsoft's WebView2 runtime, which draws the \
             interface, made one, to Microsoft. Narrated in a synthetic clone of my own voice, made on my machine; made with Claude Code.",
        ),
        featured: true,
    },
    Build {
        slug: "nisaba",
        name: "Nisaba Capital Charting",
        kicker: "Founder · solo build",
        year: "2026",
        group: Group::Integrity,
        line: "A native-Rust financial-intelligence terminal that pairs market data with an \
               astrology engine validated to sub-arcsecond precision.",
        stat: "~25k lines of Rust · 132 tests",
        role: "Founder & sole builder",
        team: &[],
        status: "Windows installer v12.2.0 · pre-revenue",
        figs: &[
            Fig { v: "~25k", k: "lines of Rust, solo, 30 days" },
            Fig { v: "132", k: "tests · 0 warnings" },
            Fig { v: "46", k: "SQLx migrations" },
            Fig { v: "+37.06%", k: "paper-trade ROI, verified" },
        ],
        stack: &["Rust", "Iced", "axum", "SQLx", "Swiss Ephemeris", "WGSL shaders"],
        links: &[
            Link { label: "Download v12.2.0 (Windows)", href: "https://github.com/nessaisling-lab/nisaba-capital-charting/releases/tag/v12.2.0" },
            Link { label: "Source", href: "https://github.com/nessaisling-lab/nisaba-capital-charting" },
            Link { label: "Nisaba brand site", href: "https://nessaisling-lab.github.io/nisaba-site/" },
        ],
        media: None,
        sections: &[
            Section {
                title: "What it is",
                lead: "A desktop terminal that pairs conventional market data with a Hellenistic \
                       astrology engine. The thesis isn't that astrology predicts markets. It's that \
                       strong investors don't dismiss a data source because it's unconventional; \
                       they measure it.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "Conventional plumbing, held to the same bar as the unconventional part.",
                points: &[
                    "12 data APIs and 10 native Rust scrapers feeding 46 SQLx migrations.",
                    "An axum REST sidecar beside an Iced desktop UI with WGSL shaders.",
                    "An ephemeris engine validated to sub-arcsecond precision against Swiss Ephemeris \
                     and JPL data.",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "Paper trading only.",
                points: &[
                    "The +37.06% is verified paper-trade ROI, not real money and not a forecast.",
                    "Pre-revenue. Presented at Pursuit Demo Day on May 6, 2026, to Pursuit staff, \
                     fellow Builders and a Fortune 500 industry reviewer.",
                ],
            },
        ],
        verify: None,
        note: None,
        featured: false,
    },
    Build {
        slug: "siteassure",
        name: "SiteAssure",
        kicker: "Pursuit L2 · Cycle 2",
        year: "2026",
        group: Group::Integrity,
        line: "A tamper-evident, offline-first OSHA compliance logger, driven by voice.",
        stat: "18 commands · zero stubs · 3-OS CI",
        role: "Builder",
        team: &[Person { name: "Raphael Ribot", handle: "raphaelribot-pursuit", role: "UI revisions" }],
        status: "Feature-complete · build from source",
        figs: &[
            Fig { v: "18", k: "commands, zero stubs" },
            Fig { v: "100%", k: "audit integrity" },
            Fig { v: "3", k: "OS CI matrix" },
        ],
        stack: &["Rust", "cross-platform desktop", "GitHub Actions"],
        links: &[Link { label: "Source (siteassure/)", href: "https://github.com/nessaisling-lab/L2-C2-Solution" }],
        media: Some(Media {
            video: "assets/work/siteassure/walkthrough.mp4",
            poster: "assets/work/siteassure/walkthrough-poster.jpg",
            alt: "Narrated walkthrough: a field note, its safety flags and what they cost, a resolution appended to \
                  the audit chain, and the trade-risk dashboard",
            w: 1280,
            h: 720,
            captions: Some("assets/work/siteassure/walkthrough.en.vtt"),
            autoplay: false,
        }),
        sections: &[
            Section {
                title: "What it is",
                lead: "A construction site's safety record is a legal document before it is anything \
                       else. SiteAssure keeps it append-only: every create, amend and delete is \
                       chained and hash-verified, so nothing is silently altered or removed.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "Each entry's hash includes the one before it.",
                points: &[
                    "Editing any historical record breaks every hash after it, so tampering is \
                     detectable by anyone holding the file, not only whoever runs the server.",
                    "Network egress is cut to an explicit allowlist rather than a kill-switch, so a \
                     new call site cannot quietly reach the internet by default.",
                    "The same records, read a second way, drive a trade-risk dashboard with no extra \
                     data entry.",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "Feature-complete and uninstallable.",
                points: &[
                    "There is no signed installer yet, so running it means building from source. \
                     That is tracked as the next task rather than hidden.",
                    "This is the hash chain HouseCheck's export uses two cycles later. It was proven \
                     here first, on OSHA records.",
                ],
            },
        ],
        verify: None,
        note: Some(
            "Recorded from v0.1.4, built from source with CUDA, on an empty database. The field note and the \
             resolution note are spoken into the app's mic input through a virtual audio cable, and the app \
             transcribes both on the GPU. Narration and notes are a synthetic clone of my own voice, made on my \
             machine; made with Claude Code.",
        ),
        featured: true,
    },
    Build {
        slug: "ziqpu",
        name: "Ziqpu",
        kicker: "Pursuit L2 · Cycle 3",
        year: "2026",
        group: Group::Integrity,
        line: "An astrology decision tool with two visible agents: one measures, one interprets, \
               and the separation is the integrity guarantee.",
        stat: "2 agents · offline ephemeris · no keys",
        role: "Team lead · 3 people",
        team: &[],
        status: "Runs locally",
        figs: &[
            Fig { v: "2", k: "agents, measurer and interpreter" },
            Fig { v: "0", k: "API keys or network needed" },
            Fig { v: "Q6_K", k: "Qwen3 14B, picked by benchmark on this laptop" },
        ],
        stack: &["Rust", "Dioxus", "axum", "Postgres", "MCP", "ANISE / JPL ephemeris", "local models"],
        links: &[Link { label: "Source", href: "https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3" }],
        media: Some(Media {
            video: "assets/work/ziqpu/walkthrough.mp4",
            poster: "assets/work/ziqpu/walkthrough-poster.jpg",
            alt: "Narrated walkthrough: first run with invented birth data, the machine benchmark, template \
                  readings for five IPOs, and the measures behind them",
            w: 1280,
            h: 720,
            captions: Some("assets/work/ziqpu/walkthrough.en.vtt"),
            autoplay: false,
        }),
        sections: &[
            Section {
                title: "What it is",
                lead: "It scores your natal chart against the chart of anything with a date of \
                       origin (v1: stocks, by IPO). Measurement and meaning are different programs, \
                       so a language model cannot touch the arithmetic.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "The sequence is fixed in code rather than chosen by the model.",
                points: &[
                    "Hamun-ana computes positions and aspects and returns structured JSON only. It \
                     never interprets.",
                    "Ungasaga reads those measures back in three beats (measured, meaning, reminder) \
                     and declines to give advice.",
                    "A read-only axum sidecar and an MCP server sit beside the Dioxus desktop UI.",
                ],
            },
            Section {
                title: "The hard part",
                lead: "An unknown birth time is honest.",
                points: &[
                    "With no birth hour, the app withholds the Ascendant and Midheaven rather than \
                     guessing them.",
                    "It benchmarks the machine and recommends the largest quantisation that actually \
                     fits, and downloads nothing until you say so.",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "For reflection, not prediction.",
                points: &[
                    "Not financial, medical, legal or psychological guidance. The interpreter is built \
                     to decline advice rather than disclaim it in a footer.",
                ],
            },
        ],
        verify: None,
        note: Some(
            "Recorded from v1.6.1 on a clean profile, with invented birth data; the readings shown come from the \
             local template. Narrated in a synthetic clone of my own voice, made on my machine; made with Claude Code.",
        ),
        featured: true,
    },
    Build {
        slug: "excalidraw",
        name: "Excalidraw: First-Save Features",
        kicker: "Pursuit L3 · client pitch",
        year: "2026",
        group: Group::Team,
        line: "Two features shipped into Excalidraw's open-source codebase, aimed at one number: new \
               users who save their first drawing.",
        stat: "22 of 27 commits · WCAG AA",
        role: "Builder · 22 of 27 commits on main",
        team: &[
            Person { name: "Jillian Krebsbach", handle: "JillK83", role: "" },
            Person { name: "Lawrence Carrillo", handle: "lawrencecarrillo", role: "" },
            Person { name: "Cornell Robertson", handle: "CodeToTheCore", role: "" },
        ],
        status: "Shipped · live A/B demo",
        figs: &[
            Fig { v: "2", k: "approved features shipped" },
            Fig { v: "6", k: "tagged releases" },
            Fig { v: "3,135", k: "users per arm, sized" },
        ],
        stack: &["TypeScript", "React", "Vite", "Vitest", "GitHub Actions", "CodeQL"],
        links: &[
            Link { label: "Live demo", href: "https://nessaisling-lab.github.io/Excalidraw-L3-Cycle-2-Features-Pitch/?arm=nudge" },
            Link { label: "Source", href: "https://github.com/nessaisling-lab/Excalidraw-L3-Cycle-2-Features-Pitch" },
        ],
        media: Some(Media {
            video: "assets/work/excalidraw/walkthrough.mp4",
            poster: "assets/work/excalidraw/walkthrough-poster.jpg",
            alt: "Narrated walkthrough: the live demo's control arm, then the nudge arm's save button, one-time \
                  prompt, format picker, and keyboard focus in the dark theme",
            w: 1280,
            h: 720,
            captions: Some("assets/work/excalidraw/walkthrough.en.vtt"),
            autoplay: false,
        }),
        sections: &[
            Section {
                title: "What it is",
                lead: "A client pitch built as working code inside Excalidraw itself: a one-time \
                       first-save prompt, and a persistent “Save to file” button with a format \
                       picker.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "Both arms of the experiment run in one live demo.",
                points: &[
                    "The demo switches between control and treatment arms from the URL, so a reviewer \
                     can see both without a build.",
                    "CI with CodeQL, a GitHub Pages demo, and every colour pair measured to WCAG AA.",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "No result is claimed.",
                points: &[
                    "The test was sized at 3,135 users per arm, but it never ran on real traffic, so \
                     there is no KPI result to report.",
                ],
            },
        ],
        verify: None,
        note: Some(
            "Recorded from the live demo with its default timings; the 15-second wait before the prompt is cut \
             and labelled. A pitch prototype, not affiliated with Excalidraw. Narrated in a synthetic clone of my own voice, made on my machine; made with Claude Code.",
        ),
        featured: false,
    },
];

/// Still images for a case study, shown as a captioned gallery.
pub struct Still {
    pub slug: &'static str,
    pub src: &'static str,
    pub alt: &'static str,
    pub caption: &'static str,
}

/// The capstone stills are frames from the portfolio re-recording of the walkthrough:
/// a private local copy on invented data, with the client's identity swapped for a
/// neutral stand-in at recording time. Every frame and the narration were OCR- and
/// Whisper-checked for the client's name before publishing (zero hits).
pub const GALLERY: &[Still] = &[
    Still {
        slug: "gala-guest-list",
        src: "assets/work/gala-guest-list/06-record-a-payment.webp",
        alt: "Staff app: recording a $12,500 check from an invented sponsor, Acme Community \
              Partners, with the check number and deposit date.",
        caption: "A sponsor's check is recorded: amount, check number, deposit date. No card or bank details, ever.",
    },
    Still {
        slug: "gala-guest-list",
        src: "assets/work/gala-guest-list/09-sponsor-portal.webp",
        alt: "Sponsor portal: a sponsor's private page with open guest seats to fill and a \
              balance of $12,500 of $25,000.",
        caption: "The sponsor's private link: name your guests, see what's paid. No account needed.",
    },
    Still {
        slug: "gala-guest-list",
        src: "assets/work/gala-guest-list/19-ipad-qr-check-in.webp",
        alt: "iPad check-in: a scanned QR ticket confirms Sam Ortega is checked in, with an Undo \
              button, above an A-to-Z guest list.",
        caption: "At the door: the scan says who came in, and Undo is one tap away.",
    },
    Still {
        slug: "gala-guest-list",
        src: "assets/work/gala-guest-list/22-live-arrivals-dashboard.webp",
        alt: "Staff dashboard: commitments, money received, seats named and arrivals, with \
              follow-ups listed by sponsor.",
        caption: "After the doors close: money, seats and arrivals on one screen.",
    },
];

/// The Library model on each project's plate (brand/library/, by Kenney, CC0), with what it
/// shows. HouseCheck keeps its own look (built with a teammate) and Nisaba has no 3D, so
/// neither has one; a plate without a model shows its number alone.
pub const PLATE_MODELS: &[(&str, &str, &str)] = &[
    ("gala-guest-list", "racing-kit--tent", "an event tent"),
    ("resona", "furniture-kit--radio", "a vintage radio"),
    ("siteassure", "city-kit-roads--construction-cone", "a traffic cone"),
    ("ziqpu", "nature-kit--crops-wheatstageb", "ripe ears of wheat, for Spica, the ear of grain"),
    ("housecheck", "modular-buildings--building-sample-tower-c", "a New York walk-up with a shopfront"),
    ("nisaba", "furniture-kit--books", "a stack of books, for the goddess of writing and accounting"),
    ("excalidraw", "furniture-kit--desk", "a desk"),
];

pub fn plate_model(slug: &str) -> Option<(&'static str, &'static str)> {
    PLATE_MODELS.iter().find(|m| m.0 == slug).map(|m| (m.1, m.2))
}

/// Earlier work, listed on the Work page without a case study of its own.
pub struct Earlier {
    pub name: &'static str,
    pub line: &'static str,
    pub href: &'static str,
}

pub const EARLIER: &[Earlier] = &[
    Earlier {
        name: "Berkshire Hathaway, Reimagined",
        line: "UX/UI redesign on seven paired principles; WCAG 2.2 AA; an 8-section design rationale.",
        href: "https://nessaisling-lab.github.io/Final-Berkshire-Hathaway-redesign/",
    },
    Earlier {
        name: "Nuclear Force: Escape Router",
        line: "Built on a fork of Nicolercc's nuclear-shelter-app: NYC blast zones, a shelter finder over \
               17 real sites, evacuation routing, packaged for Windows with Electron.",
        href: "https://github.com/aislingld-pursuit/Nightfall-Win-x64",
    },
    Earlier {
        name: "NYC Showroom: Inventory Tracker",
        line: "Multi-user desktop app over 2,730+ items in Postgres, with roles and a who-did-what audit log.",
        href: "https://github.com/aislingld-pursuit/Inventorix-Unified-Live-Dashboard-Part-4-of-Solutions-",
    },
    Earlier {
        name: "PS3 Debian + custom Wi-Fi driver",
        line: "C and Linux device drivers on a PlayStation 3; where I learned to bring up unfamiliar systems.",
        href: "",
    },
];

pub struct Rule {
    pub head: &'static str,
    pub body: &'static str,
}

pub const RULES: &[Rule] = &[
    Rule {
        head: "Measure, don't estimate.",
        body: "Figures in my repos are measured and say so. Anything derived is labelled derived; \
               anything unverified says unverified.",
    },
    Rule {
        head: "Verify against production, not the repo.",
        body: "Most real defects I have found were invisible from inside the editor: a search that \
               answered a Manhattan address with a Brooklyn building, a failed lookup that kept the \
               previous result on screen.",
    },
    Rule {
        head: "An absent answer is still an answer.",
        body: "The honest design often has three states, not two: signed, unsigned but intact, \
               tampered. Collapsing the middle one is how software states something reassuring it \
               has not earned.",
    },
];

pub struct Job {
    pub when: &'static str,
    pub org: &'static str,
    pub role: &'static str,
    pub body: &'static str,
}

pub const JOBS: &[Job] = &[
    Job {
        when: "Sep – Nov 2026",
        org: "Pursuit client capstone (NYC nonprofit)",
        role: "Technical director & repo admin",
        body: "Gala guest list, sponsor portal and offline iPad check-in on a team of four. Postgres \
               row-level security, hashed single-purpose tokens, an append-only change log, a \
               ticketing-platform import and a Salesforce sync tested against a stand-in org; 180 of \
               the team's 223 merged pull requests.",
    },
    Job {
        when: "Apr 2026 – now",
        org: "Nisaba Capital Charting",
        role: "Founder & builder",
        body: "A ~25k-line native-Rust financial-intelligence terminal, built solo in 30 days; 132 \
               tests, zero warnings, Windows installer.",
    },
    Job {
        when: "Sep 2025 – Mar 2026",
        org: "Architechne",
        role: "Data analyst & partnership coordinator",
        body: "A Python + AI scraper and a MySQL territory database for luxury-design lead \
               generation, cutting manual research about 50%.",
    },
    Job {
        when: "Jul 2017 – Jan 2020",
        org: "Airbnb",
        role: "Self-employed property manager",
        body: "Ran a short-term-rental business solo for 2.5 years: listing, marketing, bookkeeping, \
               vendors, guests.",
    },
];

pub const SCHOOLS: &[Job] = &[
    Job {
        when: "May 2026 – now",
        org: "Pursuit",
        role: "AI-Native Builder, inaugural cohort",
        body: "600+ hour, 7-month in-person intensive: 10+ end-to-end builds and 8+ PRDs. Client \
               capstone Demo Day: November 9, 2026.",
    },
    Job {
        when: "Oct 2025",
        org: "Per Scholas",
        role: "IT Support",
        body: "CompTIA A+, Google IT Support Professional Certificate, ServiceNow ITSM Fundamentals.",
    },
    Job {
        when: "Coursework complete",
        org: "Vaughn College",
        role: "Aircraft Mechanic, Airframe & Powerplant",
        body: "All technical coursework toward the FAA A&P rating.",
    },
];

/// One kind of data-integration work: what it covers, and what the team is left holding.
pub struct Capability {
    pub title: &'static str,
    pub covers: &'static str,
    pub leaves: &'static str,
}

/// Data integration, shown as what I build rather than an offer: no prices, no availability.
pub const INTEGRATION: &[Capability] = &[
    Capability {
        title: "Migrate and clean up",
        covers: "Spreadsheets, old CRMs and duplicate records, moved into Salesforce.",
        leaves: "A dry run on a copy first, then the import, with every record traced to its source.",
    },
    Capability {
        title: "Sync an outside app into Salesforce",
        covers: "Ticketing, donations, events or forms, into Accounts, Opportunities, payments, \
                 Contacts and Campaign Members.",
        leaves: "A tested sync, plus import files a Salesforce partner can review before anything \
                 changes.",
    },
    Capability {
        title: "Keep it running",
        covers: "Watching syncs after launch, fixing what breaks, and making small changes.",
        leaves: "An alert when a job stops, and a written log of every change.",
    },
];

/// How the integration work is done. Each is a rule the gala capstone already runs on.
pub const INTEGRATION_RULES: &[&str] = &[
    "Tested on invented data before anything touches yours.",
    "Keys stay on the server, never in a browser or a spreadsheet.",
    "No email or record goes out without your approval.",
    "Built with Claude Code; no AI inside the app unless you ask for it.",
];

pub struct Row {
    pub dt: &'static str,
    pub dd: &'static str,
}

pub const STACK: &[Row] = &[
    Row { dt: "Rust", dd: "axum · SQLx · rusqlite · Tauri 2 · Dioxus · Iced · whisper-rs · cpal" },
    Row { dt: "Integrity", dd: "Ed25519 · SHA-256 hash chains · Postgres row-level security · hashed tokens · gitleaks · egress allowlists" },
    Row { dt: "Web", dd: "TypeScript · React · Vite · Tailwind · Supabase" },
    Row { dt: "Integration", dd: "Salesforce REST API and data model · SOQL · import pipelines with record provenance" },
    Row { dt: "Also", dd: "Python · SQL (Postgres, MySQL) · WGSL" },
    Row { dt: "AI", dd: "Claude Code · Anthropic + OpenAI APIs · MCP · whisper.cpp · local models · tool-calling with grounding" },
    Row { dt: "Ops", dd: "GitHub admin · Actions (3-OS matrix) · Docker · Fly.io · Vercel · Cloudflare Pages + Access" },
];

/// Facts plotted along each job's track on the Experience sky, in track order. Each one
/// restates part of that job's own body text.
pub const JOB_FACTS: &[(&str, &[&str])] = &[
    ("Pursuit client capstone (NYC nonprofit)", &["Offline iPad check-in", "180 of 223 merged PRs",
        "Postgres row-level security", "Salesforce sync, stand-in org"]),
    ("Nisaba Capital Charting", &["~25k lines of native Rust", "132 tests · zero warnings", "Built solo in 30 days"]),
    ("Architechne", &["Python + AI scraper", "Manual research cut about 50%", "MySQL territory database"]),
    ("Airbnb", &["2.5 years, solo", "Listing · marketing · bookkeeping"]),
];

pub fn job_facts(org: &str) -> &'static [&'static str] {
    JOB_FACTS.iter().find(|f| f.0 == org).map_or(&[], |f| f.1)
}

/// One tool on the Experience ziggurat: what it is, in a plain line, and the builds that used
/// it. Each use names a phrase that appears on that build's page; a test holds it to that.
pub struct Tool {
    pub name: &'static str,
    pub what: &'static str,
    pub used: &'static [(&'static str, &'static str)],
}

pub const TOOLS: &[Tool] = &[
    Tool { name: "axum", what: "A Rust web framework for servers and APIs, from the Tokio project.",
        used: &[("housecheck", "axum"), ("nisaba", "axum"), ("ziqpu", "axum")] },
    Tool { name: "SQLx", what: "Rust's async SQL toolkit; it checks queries against the database at compile time.",
        used: &[("nisaba", "SQLx")] },
    Tool { name: "rusqlite", what: "Rust bindings for SQLite, the database that lives in a single file.", used: &[] },
    Tool { name: "Tauri 2", what: "Small desktop apps with a Rust core and a web front end.", used: &[("resona", "Tauri 2")] },
    Tool { name: "Dioxus", what: "A Rust UI framework in the style of React. This site is rendered with it, at build time.",
        used: &[("ziqpu", "Dioxus")] },
    Tool { name: "Iced", what: "A Rust GUI toolkit inspired by the Elm architecture.", used: &[("nisaba", "Iced")] },
    Tool { name: "whisper-rs", what: "Rust bindings for whisper.cpp, for speech-to-text.", used: &[("resona", "whisper-rs")] },
    Tool { name: "cpal", what: "Cross-platform audio input and output for Rust.", used: &[("resona", "cpal")] },
    Tool { name: "Ed25519", what: "A fast digital-signature scheme: a signature proves who made a file and that it hasn't changed since.",
        used: &[("housecheck", "Ed25519")] },
    Tool { name: "SHA-256 hash chains", what: "Each record carries the hash of the one before it, so changing any entry breaks every link after it.",
        used: &[("housecheck", "hash chain"), ("siteassure", "hash chain")] },
    Tool { name: "Postgres row-level security", what: "Rules inside Postgres that decide, row by row, what each user can see or change.",
        used: &[("gala-guest-list", "row-level security")] },
    Tool { name: "hashed tokens", what: "Only a hash of each access token is stored, so a leaked database holds no working links.",
        used: &[("gala-guest-list", "stored only as SHA-256 hashes")] },
    Tool { name: "gitleaks", what: "A scanner that stops passwords and keys from being committed to git.", used: &[] },
    Tool { name: "egress allowlists", what: "Outbound network access limited to a named list of hosts.",
        used: &[("siteassure", "allowlist")] },
    Tool { name: "TypeScript", what: "JavaScript with types, checked before the code runs.",
        used: &[("housecheck", "TypeScript"), ("gala-guest-list", "TypeScript"), ("resona", "TypeScript"), ("excalidraw", "TypeScript")] },
    Tool { name: "React", what: "A JavaScript library for building interfaces out of components.",
        used: &[("gala-guest-list", "React"), ("resona", "React"), ("excalidraw", "React")] },
    Tool { name: "Vite", what: "A fast dev server and build tool for web front ends.",
        used: &[("gala-guest-list", "Vite"), ("excalidraw", "Vite")] },
    Tool { name: "Tailwind", what: "A CSS framework of small utility classes, written straight into the markup.",
        used: &[("gala-guest-list", "Tailwind")] },
    Tool { name: "Supabase", what: "Hosted Postgres with sign-in, storage and edge functions on top.",
        used: &[("gala-guest-list", "Supabase")] },
    Tool { name: "Salesforce REST API and data model", what: "Salesforce's HTTP API, and the objects and fields its records live in.",
        used: &[("gala-guest-list", "Salesforce")] },
    Tool { name: "SOQL", what: "Salesforce Object Query Language, the SQL-like way to query Salesforce records.", used: &[] },
    Tool { name: "import pipelines with record provenance", what: "Imports that keep where every row came from, so any value traces back to its source.",
        used: &[] },
    Tool { name: "Python", what: "A general-purpose language, the usual choice for data work.", used: &[] },
    Tool { name: "SQL (Postgres, MySQL)", what: "The language of relational databases, in two of the most common ones.",
        used: &[("ziqpu", "Postgres"), ("gala-guest-list", "Postgres")] },
    Tool { name: "WGSL", what: "WebGPU's shading language, for programs that run on the graphics card.", used: &[("nisaba", "WGSL")] },
    Tool { name: "Claude Code", what: "Anthropic's coding agent, in the terminal and the editor.",
        used: &[("gala-guest-list", "Claude Code")] },
    Tool { name: "Anthropic + OpenAI APIs", what: "The HTTP APIs for Claude and GPT models.", used: &[] },
    Tool { name: "MCP", what: "Model Context Protocol: an open standard for giving AI models tools and data.", used: &[("ziqpu", "MCP")] },
    Tool { name: "whisper.cpp", what: "A fast C/C++ port of OpenAI's Whisper speech-recognition model.", used: &[("resona", "whisper.cpp")] },
    Tool { name: "local models", what: "Language models that run on your own machine, with no cloud call.", used: &[("ziqpu", "local models")] },
    Tool { name: "tool-calling with grounding", what: "A model calls functions for facts, and its answer sticks to what they returned.", used: &[] },
    Tool { name: "GitHub admin", what: "Running a team's repository: branch protection, reviews, permissions.",
        used: &[("gala-guest-list", "repo admin")] },
    Tool { name: "Actions (3-OS matrix)", what: "GitHub's built-in CI, here running every check on Windows, macOS and Linux.",
        used: &[("siteassure", "3-OS"), ("excalidraw", "GitHub Actions")] },
    Tool { name: "Docker", what: "Packs an app and everything it needs into a container that runs the same anywhere.",
        used: &[("housecheck", "Docker")] },
    Tool { name: "Fly.io", what: "A platform that runs app containers in regions close to users.", used: &[("housecheck", "Fly.io")] },
    Tool { name: "Vercel", what: "A host for web front ends, deployed straight from git.", used: &[("housecheck", "Vercel")] },
    Tool { name: "Cloudflare Pages + Access", what: "Static hosting on Cloudflare's network, with Access putting a sign-in in front.",
        used: &[("gala-guest-list", "Cloudflare Pages")] },
];

pub fn tool(name: &str) -> Option<&'static Tool> {
    TOOLS.iter().find(|t| t.name == name)
}

/// A tool's id on the page, for its card: lower case, runs of anything else become one dash.
pub fn tool_id(name: &str) -> String {
    let mut s = String::from("tool-");
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            s.push(c.to_ascii_lowercase());
        } else if !s.ends_with('-') {
            s.push('-');
        }
    }
    s.trim_end_matches('-').to_string()
}

pub struct Doc {
    pub name: &'static str,
    pub files: &'static [(&'static str, &'static str)],
}

pub const DOCS: &[Doc] = &[
    Doc {
        name: "Résumé",
        files: &[
            ("PDF · Letter", "assets/docs/resume/resume-letter.pdf"),
            ("PDF · Legal", "assets/docs/resume/resume-legal.pdf"),
        ],
    },
    Doc { name: "Berkshire redesign: design rationale", files: &[("PDF", "assets/docs/berkshire-rationale/berkshire-rationale.pdf")] },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn root() -> &'static Path {
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/.."))
    }

    /// The first section has to stand on its own: most readers stop after it.
    #[test]
    fn every_case_opens_with_a_standalone_section() {
        for b in BUILDS {
            let first = &b.sections[0];
            assert_eq!(first.title, "What it is", "{} must open with What it is", b.name);
            assert!(first.points.is_empty(), "{} opening section must carry its claim in prose", b.name);
            assert!(b.sections.iter().any(|s| s.title == "What it doesn't do"), "{} hides its limits", b.name);
        }
    }

    /// The laptop on the home desk holds four plates, one per project world Aisling named
    /// (Nov 11): a 2 x 2 screen. A fifth would wrap onto a lonely row.
    #[test]
    fn four_builds_on_the_laptop() {
        assert_eq!(BUILDS.iter().filter(|b| b.featured).count(), 4);
    }

    #[test]
    fn slugs_are_unique_and_url_safe() {
        for (i, a) in BUILDS.iter().enumerate() {
            assert!(a.slug.chars().all(|c| c.is_ascii_lowercase() || c == '-'), "{}", a.slug);
            for b in &BUILDS[i + 1..] {
                assert_ne!(a.slug, b.slug);
            }
        }
    }

    /// Every local file the site links to has to exist, or a reader gets a 404.
    #[test]
    fn every_local_asset_exists() {
        let mut paths: Vec<&str> = DOCS.iter().flat_map(|d| d.files.iter().map(|f| f.1)).collect();
        for b in BUILDS {
            if let Some(m) = &b.media {
                paths.push(m.video);
                paths.push(m.poster);
                paths.extend(m.captions);
            }
        }
        paths.extend(GALLERY.iter().map(|s| s.src));
        // The turntable strip plays on the page; the still is what the link-preview cards use.
        let models: Vec<String> = PLATE_MODELS.iter()
            .flat_map(|m| ["matte.webp", "turn.webp", "turn-lg.webp", "model.json"].map(|k| format!("brand/library/{}-{k}", m.1)))
            .collect();
        paths.extend(models.iter().map(String::as_str));
        for p in paths {
            assert!(root().join(p).is_file(), "missing asset: {p}");
        }
        // Link-preview cards, one per case study; regenerate with tools/og-cards.mjs.
        for b in BUILDS {
            let p = format!("assets/og/{}.png", b.slug);
            assert!(root().join(&p).is_file(), "missing asset: {p}");
        }
    }

    /// The client is unnamed on the public site until they confirm. A name slipping into
    /// this file would publish it.
    #[test]
    fn client_stays_unnamed() {
        let src = include_str!("content.rs").to_lowercase();
        let banned = [["aa", "fe"].concat(), ["asian ", "americans"].concat()];
        for b in banned {
            assert!(!src.contains(&b), "content.rs names the client");
        }
    }

    /// Every tool on the stack ziggurat has a card, and every job a set of facts for its track.
    #[test]
    fn every_tool_and_job_has_its_words() {
        for row in STACK {
            for item in row.dd.split(" · ") {
                assert!(tool(item).is_some(), "no card for stack tool {item:?}");
            }
        }
        for j in JOBS {
            assert!(!job_facts(j.org).is_empty(), "no sky facts for {:?}", j.org);
        }
    }

    /// A card says a build used a tool only where that build's own page says so.
    #[test]
    fn tool_uses_are_on_the_build_page() {
        for t in TOOLS {
            for (slug, phrase) in t.used {
                let b = BUILDS.iter().find(|b| b.slug == *slug).unwrap_or_else(|| panic!("{} cites unknown build {slug}", t.name));
                let mut text = format!("{} {} {} {}", b.line, b.stat, b.role, b.stack.join(" "));
                for s in b.sections {
                    text.push_str(s.title);
                    text.push_str(s.lead);
                    s.points.iter().for_each(|p| text.push_str(p));
                }
                assert!(text.to_lowercase().contains(&phrase.to_lowercase()), "{} says {slug} used it, but {phrase:?} isn't on that page", t.name);
            }
        }
    }
}
