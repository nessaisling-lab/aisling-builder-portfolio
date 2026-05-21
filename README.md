# aisling-builder-portfolio

Portfolio v2 for **Stan (Aisling) Leiva-Davila** — Founder & AI Builder, NYC.

Single-file static HTML site. No build step. Editorial Tech Hybrid design (Mockup C). Founder section transitions into full Nisaba grimoire palette.

## What's in here

```
index.html                   — single-page portfolio (v2)
assets/                      — drop photos here
  portrait-placeholder.jpg   — replace with your portrait
README.md                    — this file
```

## Local preview

```powershell
# Just open the file in a browser. No build needed.
start index.html
```

Or via a local HTTP server (cleaner for OG meta tags):
```powershell
python -m http.server 8080
# visit http://localhost:8080/
```

## Add your portrait

Drop a portrait photo into `assets/` named `portrait-placeholder.jpg` (or update the path in `index.html`). 4:5 aspect ratio recommended. The hero photo area is sized for portrait orientation.

## Deploy to BOTH GitHubs (one push, two destinations)

### Step 1 — Create the two repos on GitHub

Create empty public repos:
1. https://github.com/new → `aisling-builder-portfolio` under `nessaisling-lab`
2. https://github.com/new → `aisling-builder-portfolio` under `aislingld-pursuit`

Do NOT initialize either with README, .gitignore, or license (this folder already has them).

### Step 2 — Initialize git locally + add both remotes

From this folder in PowerShell:

```powershell
git init
git add .
git commit -m "Initial portfolio v2"
git branch -M main

# Add personal remote
git remote add personal https://github.com/nessaisling-lab/aisling-builder-portfolio.git

# Add school remote
git remote add school https://github.com/aislingld-pursuit/aisling-builder-portfolio.git

# Push to both
git push personal main
git push school main
```

### Step 3 — Enable GitHub Pages on each

For each repo:
1. Go to repo → **Settings** → **Pages**.
2. Source: **Deploy from a branch**.
3. Branch: `main` / `(root)`.
4. Save.

Wait ~1 minute. Live at:
- https://nessaisling-lab.github.io/aisling-builder-portfolio/
- https://aislingld-pursuit.github.io/aisling-builder-portfolio/

### Step 4 — Future updates (one command, both sites)

```powershell
# After editing index.html or assets:
git add .
git commit -m "describe the change"
git push personal main
git push school main
```

Or push to both simultaneously by adding both URLs to a single remote:
```powershell
git remote set-url --add --push personal https://github.com/aislingld-pursuit/aisling-builder-portfolio.git
git push personal main   # now pushes to BOTH GitHubs
```

## Design system reference

Built on **Mockup C — Editorial Tech Hybrid** from the design exploration:
- **Type:** Source Serif 4 (display) · Inter (UI) · JetBrains Mono (code/labels) · Fraunces (founder section only)
- **Palette:** Off-white `#f5f2eb` · Deep navy `#0d1828` · Brass gold `#b8893c`
- **Founder section:** transitions into full Nisaba grimoire palette — navy + brass + radial-gradient atmosphere

If you want to revisit the original 3 design mockups (A · Editorial Magazine, B · Minimalist Tech, C · Editorial Tech Hybrid), they live in `Pursuit Ai Professional Portfolio/design-mockups/`.

## License

Marketing copy + design assets are © 2026 Stan (Aisling) Leiva-Davila. Personal portfolio site.
