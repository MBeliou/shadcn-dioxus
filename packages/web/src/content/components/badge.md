---
title: Badge
description: Displays a badge or a component that looks like a badge.
component: true
---

<ComponentPreview name="badge-demo"/>

## Usage

```rust
use ui::{Badge, BadgeVariant};

rsx! {
    Badge { variant: BadgeVariant::Outline, "Badge" }
}
```

## Link

You can use the `href` prop to render the badge as an `<a>` element.

```rust
use ui::Badge;

rsx! {
    Badge { href: "/dashboard", "Badge" }
}
```

## Examples

### Default

<ComponentPreview name="badge-demo"/>

### Secondary

<ComponentPreview name="badge-secondary"/>

### Destructive

<ComponentPreview name="badge-destructive"/>

### Outline

<ComponentPreview name="badge-outline"/>
