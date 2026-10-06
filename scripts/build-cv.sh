#!/usr/bin/env bash
# Render the English CV (typst) to PDF for the /cv sub-page, and regenerate
# the wrapper page that embeds it. Output is committed, so the Zola deploy
# action needs no typst.
set -euo pipefail

cd "$(dirname "$0")/.."

typst compile \
    --root curriculum-vitae \
    --font-path curriculum-vitae/fonts \
    curriculum-vitae/cv-rafael-bachmann/cv.en.typ \
    static/cv/cv.en.pdf

cat > static/cv/index.html <<'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>CV — Rafael Bachmann</title>
<style>
  html, body { margin: 0; height: 100%; }
  embed { width: 100%; height: 100%; }
</style>
</head>
<body>
<embed src="cv.en.pdf" type="application/pdf">
<noscript><p>See <a href="cv.en.pdf">cv.en.pdf</a>.</p></noscript>
</body>
</html>
EOF

echo "Wrote static/cv/cv.en.pdf and static/cv/index.html"
