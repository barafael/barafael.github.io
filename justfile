default := "build"

# Build the site into public/
build:
    zola build

# Local dev server
serve:
    zola serve

# Regenerate static/cv/ from the curriculum-vitae submodule (requires typst)
cv:
    ./scripts/build-cv.sh
