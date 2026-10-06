#!/usr/bin/env bash
# Render the English CV (typst) to static HTML for the /cv sub-page.
# The generated file is committed, so the Zola deploy action needs no typst.
set -euo pipefail

cd "$(dirname "$0")/.."

typst compile \
    --features html \
    --root curriculum-vitae \
    --format html \
    --font-path curriculum-vitae/fonts \
    curriculum-vitae/cv-rafael-bachmann/cv.en.typ \
    static/cv/index.html

echo "Wrote static/cv/index.html"
