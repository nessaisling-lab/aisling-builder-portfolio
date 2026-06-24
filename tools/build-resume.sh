#!/usr/bin/env bash
# Regenerate every résumé format for the Documents hub.
# Sources of truth:
#   assets/docs/resume/resume.md   -> docx, txt, html  (via pandoc)
#   assets/docs/resume/resume.tex  -> letter + legal PDF (via pdflatex)
# Requires: pandoc, pdflatex (MiKTeX/TeX Live). Installed globally on this machine.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
D="$ROOT/assets/docs/resume"

PANDOC="$(command -v pandoc || echo '/c/Program Files/Pandoc/pandoc.exe')"
PDFLATEX="$(command -v pdflatex || echo '/c/Program Files/MiKTeX/miktex/bin/x64/pdflatex.exe')"

echo "==> pandoc formats from resume.md"
"$PANDOC" "$D/resume.md" -t plain -o "$D/resume.txt"
"$PANDOC" "$D/resume.md" -o "$D/resume.docx"
"$PANDOC" "$D/resume.md" -s --embed-resources --metadata title="Stan (Aisling) Leiva-Davila — Resume" -o "$D/resume.html"

echo "==> PDFs from resume.tex (letter + legal)"
TMP="$(mktemp -d)"
cp "$D/resume.tex" "$TMP/resume-letter.tex"
sed 's/\[letterpaper,10pt\]/[legalpaper,10pt]/' "$D/resume.tex" > "$TMP/resume-legal.tex"
( cd "$TMP"
  for v in letter legal; do
    "$PDFLATEX" -enable-installer -interaction=nonstopmode "resume-$v.tex" >/dev/null 2>&1 || true
  done )
cp "$TMP/resume-letter.pdf" "$D/resume-letter.pdf"
cp "$TMP/resume-legal.pdf"  "$D/resume-legal.pdf"
rm -rf "$TMP"

echo "==> done. Formats in $D:"
ls -1 "$D"
