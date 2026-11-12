// Renders one 1200x630 link-preview card per case study into assets/og/{slug}.png, and the
// site-wide card, assets/og-image.png. Both are drawn with the site's own built CSS (brand kit
// plus assets/site.css), so a card can never drift from the page's colours, fonts or desk.
// Reads the facts from site/src/content.rs, so a card never says something the page doesn't.
// Run after `cargo run` whenever a build's name, kicker, line, stat or plate model changes,
// or the portrait:
//   NODE_PATH=<dir with playwright> node tools/og-cards.mjs
// Commit the PNGs; the site build only links to them.
import { readFileSync, mkdirSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { createRequire } from 'node:module';

const { chromium } = createRequire(import.meta.url)('playwright');
const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const src = readFileSync(join(root, 'site/src/content.rs'), 'utf8');

// Rust string literal: unescape \" and drop `\`-newline continuations with their indent.
const str = (s) => s.replace(/\\\r?\n\s*/g, '').replace(/\\"/g, '"');
// ponytail: regex over content.rs, not a parser. Fine while fields stay `key: "literal"`.
const builds = src.split('pub const BUILDS: &[Build] = &[')[1].split(/\n    Build \{/).slice(1).map((b) => {
  const f = {};
  for (const [, k, v] of b.matchAll(/\b(slug|name|kicker|line|stat): "((?:[^"\\]|\\.)*)"/gs)) {
    f[k] ??= str(v);
  }
  return f;
});
const models = Object.fromEntries([...src.matchAll(/\("([a-z0-9-]+)", "([a-z0-9-]+--[a-z0-9-]+)", "/g)].map((m) => [m[1], m[2]]));

const esc = (s) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;');
const URL_TEXT = 'nessaisling-lab.github.io/aisling-builder-portfolio';
// Day world, matte, on the page's own classes. The desk band runs along the bottom like home.
const shell = (body) => `<!doctype html><html lang="en" data-theme="light" data-finish="matte"><meta charset="utf-8">
<link rel="stylesheet" href="brand/ness.css"><link rel="stylesheet" href="assets/site.css"><style>
html,body{width:1200px;height:630px;overflow:hidden}
body{margin:0;background:var(--surface);position:relative}
.og{position:absolute;inset:44px 56px 104px;display:grid;gap:44px}
.og .card{grid-area:auto;align-self:stretch;margin:0;padding:34px 38px;display:flex;flex-direction:column}
.og .window{grid-area:auto;align-self:stretch;margin:10px 10px 30px;height:auto}
.band{position:absolute;left:0;right:0;bottom:0;height:72px;background:var(--desk-front);border-top:16px solid var(--desk-top);
  box-shadow:inset 0 1px 0 var(--desk-cut);display:flex;align-items:center;padding:0 56px}
.u{font:600 22px/1 var(--display);font-stretch:106%;color:var(--ink-2)}
</style><body class="nb">${body}<div class="band"><p class="u">${URL_TEXT}</p></div></body></html>`;

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1200, height: 630 } });
// Load the real home page first: setContent keeps its URL, so the relative CSS, fonts and
// images below resolve against the repo root. The skyline comes from the built page too.
await page.goto(pathToFileURL(join(root, 'index.html')).href, { waitUntil: 'load' });
const skyline = await page.evaluate(() => document.querySelector('.window')?.innerHTML ?? '');
if (!skyline) throw new Error('index.html has no .window skyline; run `cargo run` in site/ first');
const shoot = async (html, out) => {
  await page.setContent(html, { waitUntil: 'load' });
  await page.evaluate(() => document.fonts.ready);
  await page.screenshot({ path: join(root, out) });
  console.log(out);
};

mkdirSync(join(root, 'assets/og'), { recursive: true });
for (const b of builds) {
  if (!b.slug || !b.stat) throw new Error(`build missing slug or stat: ${JSON.stringify(b)}`);
  const m = models[b.slug];
  await shoot(shell(`<div class="og" style="grid-template-columns:${m ? '1fr 210px' : '1fr'}">
<section class="card"><span class="pin"></span>
<p style="font:600 22px/1.2 var(--display);color:var(--muted)">${esc(b.kicker)}</p>
<h1 style="font-size:${b.name.length > 18 ? 58 : 72}px;line-height:1.02;margin-top:14px;max-width:none;text-wrap:balance">${esc(b.name)}</h1>
<p style="font-size:26px;line-height:1.38;color:var(--ink-2);margin-top:16px;display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical;overflow:hidden">${esc(b.line)}</p>
<p style="margin-top:auto;font:800 28px/1.2 var(--display);font-stretch:106%;color:var(--ink);border-left:5px solid var(--brass-deep);padding-left:16px">${esc(b.stat)}</p>
</section>${m ? `<img src="brand/library/${m}-matte.webp" alt="" style="align-self:center;width:210px;height:210px;object-fit:contain">` : ''}
</div>`), `assets/og/${b.slug}.png`);
}

// The site-wide card (every page that isn't a case study): her wall card beside the window.
await shoot(shell(`<div class="og" style="grid-template-columns:600px 1fr">
<section class="card"><span class="pin"></span>
<div style="display:grid;grid-template-columns:120px 1fr;gap:22px;align-items:center">
<img class="id-photo" src="assets/portrait@2x.jpg" alt="" style="width:120px">
<div><p style="font:800 32px/1.1 var(--display);font-stretch:108%;text-wrap:balance">Aisling (Stan) Leiva-Davila</p>
<p class="nb-place" style="font-size:22px;color:var(--ink-2);margin-top:8px">New York</p>
<p style="font:600 21px/1.3 var(--body);color:var(--ink-2);margin-top:6px">AI-native builder, Rust first</p></div></div>
<h1 style="font-size:44px;line-height:1.05;margin-top:auto">I build with AI the way a smith works a forge.</h1>
</section>
<div class="window" aria-hidden="true">${skyline}</div>
</div>`), 'assets/og-image.png');
await browser.close();
