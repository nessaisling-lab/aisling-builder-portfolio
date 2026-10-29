// Renders one 1200x630 link-preview card per case study into assets/og/{slug}.png.
// Reads the facts from site/src/content.rs, so a card never says something the page doesn't.
// Text only: the video posters are title cards, so cropping one beside the text repeats it.
// Run after changing a build's name, kicker, line or stat:
//   NODE_PATH=<dir with playwright> node tools/og-cards.mjs
// Commit the PNGs; the site build only links to them.
import { readFileSync, mkdirSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
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

// Inlined as data: URLs; a page loaded with setContent can't fetch file:// resources.
const font = (file) => `data:font/woff2;base64,${readFileSync(join(root, 'assets/fonts', file)).toString('base64')}`;
const esc = (s) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;');
const card = (b) => {
  return `<!doctype html><meta charset="utf-8"><style>
@font-face{font-family:N;font-weight:400 500;src:url(${font('newsreader.woff2')})}
@font-face{font-family:M;font-weight:400;src:url(${font('plex-mono-400.woff2')})}
@font-face{font-family:M;font-weight:500;src:url(${font('plex-mono-500.woff2')})}
*{margin:0;box-sizing:border-box}
body{width:1200px;height:630px;display:flex;background:#F4F1E9;color:#171A1F;overflow:hidden}
.t{flex:1;padding:64px 72px;display:flex;flex-direction:column}
.k{font:500 21px/1 M;letter-spacing:.06em;text-transform:uppercase;color:#9A4012}
h1{font:500 ${b.name.length > 18 ? 64 : 84}px/1.02 N;letter-spacing:-.015em;margin-top:34px}
.r{width:90px;height:3px;background:#9A4012;margin:30px 0 26px}
.l{font:400 27px/1.38 N;color:#585C63;max-width:960px;display:-webkit-box;-webkit-line-clamp:3;-webkit-box-orient:vertical;overflow:hidden}
.s{margin-top:auto;font:500 30px/1.2 M;color:#171A1F;border-left:4px solid #9A4012;padding-left:16px}
.u{margin-top:22px;font:400 19px/1 M;color:#9A4012}
</style><body><div class="t">
<p class="k">Case study · ${esc(b.kicker)}</p><h1>${esc(b.name)}</h1><div class="r"></div>
<p class="l">${esc(b.line)}</p><p class="s">${esc(b.stat)}</p>
<p class="u">nessaisling-lab.github.io/aisling-builder-portfolio</p></div></body>`;
};

mkdirSync(join(root, 'assets/og'), { recursive: true });
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1200, height: 630 } });
for (const b of builds) {
  if (!b.slug || !b.stat) throw new Error(`build missing slug or stat: ${JSON.stringify(b)}`);
  await page.setContent(card(b), { waitUntil: 'load' });
  await page.evaluate(() => document.fonts.ready);
  await page.screenshot({ path: join(root, 'assets/og', `${b.slug}.png`) });
  console.log(`assets/og/${b.slug}.png`);
}
await browser.close();
