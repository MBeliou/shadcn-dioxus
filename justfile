# =============================================================================
# shadcn-dioxus — justfile
# =============================================================================
#
# Usage:
#   just              — run default recipe (test)
#   just build          — debug build all workspace members
#   just build-release  — release build all workspace members
#   just test           — run tests with cargo-nextest
#   just test-verbose   — run tests with full output
#   just test-ui         — run only ui package tests
#   just check          — cargo check all
#   just clippy         — cargo clippy all
# just fmt            — format all Rust code
# just fmt-check      — check formatting without writing
# just clean          — remove target directory
#   just css            — install deps + build Tailwind CSS
#   css-watch          — install deps + watch Tailwind CSS on change
#   just serve          — build CSS + serve web app in dev mode
#   just serve-no-css   — serve web app without rebuilding CSS
#   just run-desktop    — build + run desktop app
#   just run-mobile     — build + run mobile app
#   just watch          — watch CSS + restart dx serve on change
#   just wr             — watchexec wrapper (original)
# =============================================================================
# ---------------------------------------------------------------------------
# Default

# ---------------------------------------------------------------------------
default: test

# ---------------------------------------------------------------------------
# Build

# ---------------------------------------------------------------------------
build:
    cargo build

build-release:
    cargo build --release

build-web:
    cargo build -p web

build-desktop:
    cargo build -p desktop

build-mobile:
    cargo build -p mobile

build-ui:
    cargo build -p ui

# ---------------------------------------------------------------------------
# Test

# ---------------------------------------------------------------------------
test:
    cargo nextest run

test-verbose:
    cargo nextest run --no-capture

test-ui:
    cargo nextest run -p ui

test-web:
    cargo nextest run -p web

test-desktop:
    cargo nextest run -p desktop

test-mobile:
    cargo nextest run -p mobile

test-no-fail:
    cargo nextest run --no-fail-fast

# ---------------------------------------------------------------------------
# Lint & Format

# ---------------------------------------------------------------------------
check:
    cargo check

clippy:
    cargo clippy -W clippy::all

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all --check

lint: fmt clippy

# ---------------------------------------------------------------------------
# CSS (uses pnpm — see package.json "packageManager": "pnpm@10.13.1")

# ---------------------------------------------------------------------------
css:
    cd packages/web && \
    pnpm install && \
    pnpm exec tailwindcss -i tailwind.css -o assets/tailwind.css

css-watch:
    cd packages/web && \
    pnpm install && \
    pnpm exec tailwindcss -i tailwind.css -o assets/tailwind.css --watch

# ---------------------------------------------------------------------------
# Serve / Dev

# ---------------------------------------------------------------------------
serve: css
    cd packages/web && \
    dx serve

serve-no-css:
    cd packages/web && \
    dx serve

# ---------------------------------------------------------------------------
# Run apps

# ---------------------------------------------------------------------------
run-desktop: build-desktop
    cd packages/desktop && \
    cargo run

run-mobile: build-mobile
    cd packages/mobile && \
    cargo run

# ---------------------------------------------------------------------------
# Watch (auto-rebuild CSS + restart dx serve on file change)

# ---------------------------------------------------------------------------
watch:
    watchexec -w "packages/web/tailwind.css" -w "packages/web/src/**/*.rs" -r "just css && just serve-no-css"

wr:
    watchexec -w ./wr.sh --clear -r "sh ./wr.sh"

# ---------------------------------------------------------------------------
# Clean

# ---------------------------------------------------------------------------
clean:
    cargo clean

clean-css:
    rm -f packages/web/assets/tailwind.css

distclean: clean clean-css
    rm -rf target

# ---------------------------------------------------------------------------
# Utilities

# ---------------------------------------------------------------------------
loc:
    find packages -name "*.rs" | xargs wc -l

deps:
    cargo tree --depth 1

deps-full:
    cargo tree

outdated:
    cargo outdated

audit:
    cargo audit

# ---------------------------------------------------------------------------
# Install tools (run once)

# ---------------------------------------------------------------------------
install-tools:
    cargo install cargo-nextest
    cargo install dioxus-cli
    cargo install cargo-audit
    cargo install cargo-outdated
