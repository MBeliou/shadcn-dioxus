---
title: Button
description: Displays a button or a component that looks like a button.
component: true
---

<ComponentPreview name="button-demo"/>

## Usage

```rust
use ui::Button;

rsx! {
    Button { "Button" }
}
```

## Link

You can use the `href` prop to render the button as an `<a>` element.

```rust
use ui::Button;

rsx! {
    Button { href: "/dashboard", "Dashboard" }
}
```

Alternatively, you can use the `button_variants` helper to create a link that looks like a button.

```rust
use ui::{button_variants, ButtonVariant, ButtonSize};

rsx! {
    a {
        href: "/dashboard",
        class: button_variants(ButtonVariant::Outline, ButtonSize::Default),
        "Dashboard"
    }
}
```

## Examples

### Primary

<ComponentPreview name="button-primary"/>

### Secondary

<ComponentPreview name="button-secondary"/>

### Destructive

<ComponentPreview name="button-destructive"/>

### Outline

<ComponentPreview name="button-outline"/>

### Ghost

<ComponentPreview name="button-ghost"/>

### Link

<ComponentPreview name="button-link"/>
