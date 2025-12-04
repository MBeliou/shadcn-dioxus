# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

shadcn_dioxus is a cross-platform UI component library built with Dioxus 0.7.1, inspired by shadcn/ui. It's a monorepo workspace with shared components (`packages/ui/`) and platform-specific apps (`packages/web/`, `packages/desktop/`, `packages/mobile/`).

## Commands

```bash
# Development (run from platform directory, e.g., packages/web/)
dx serve                    # Start dev server with hot reload

# Build
dx build                    # Build for production
cargo build                 # Build all workspace packages
cargo check                 # Type check without building

# Formatting
dx fmt                      # Format Dioxus code
cargo fmt                   # Format Rust code
```

## Architecture

### Workspace Structure
- `packages/ui/` - Shared component library (platform-agnostic, no web_sys or platform APIs)
- `packages/web/` - Web app with documentation site
- `packages/desktop/` - Desktop app (Dioxus Desktop)
- `packages/mobile/` - Mobile app (Dioxus Mobile)

### Component Pattern
Components follow a consistent structure with variant enums and the `cn()` utility for class merging:

```rust
#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(into, default)] class: String,
    children: Element,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let classes = cn(&button_variants(variant, size), &class);
    rsx! {
        button { class: "{classes}", ..attributes, {children} }
    }
}
```

### Compound Components
Complex components use sub-components (e.g., Card > CardHeader > CardTitle, CardContent, CardFooter).

### Styling
- Tailwind CSS with oklch color space
- CSS variables defined in `tailwind.css` (--primary, --secondary, --background, etc.)
- Dark mode via `.dark` class
- All components use `data-slot="component-name"` attribute for styling hooks

### Routing
Each platform uses Dioxus Router with `#[derive(Routable)]` enum and `#[layout()]` for shared layouts.

## Dioxus 0.7 Notes

**IMPORTANT**: Dioxus 0.7 has breaking changes from earlier versions. See `AGENTS.md` for the complete API reference.

Key patterns:
- Use `use_signal()` for local state (not `use_state`)
- Use `use_memo()` for derived values
- Use `use_context_provider()`/`use_context()` for shared state
- Props must implement `Clone + PartialEq`
- Use `#[props(into, default)]` for optional string props
- Use `#[props(extends = GlobalAttributes)]` to capture HTML attributes
- RSX syntax: `rsx! { div { class: "...", "text" } }`

## Conventions

- Component names: PascalCase (Button, CardHeader)
- Variant enums: ComponentVariant::Default, Outline, Ghost, etc.
- Size enums: ComponentSize::Default, Sm, Lg, Icon
- Always use `cn()` from `utils.rs` to merge classes
- Props with children: `children: Element`
- Event handlers: `onclick: Option<EventHandler<MouseEvent>>`
