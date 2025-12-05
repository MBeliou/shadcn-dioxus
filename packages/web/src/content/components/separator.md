---
title: Separator
description: Visually or semantically separates content.
component: true
---

<ComponentPreview name="separator-demo"/>

## Installation

```bash
npx shadcn-dioxus add separator
```

## Usage

```rust
use ui::Separator;

rsx! {
    Separator {}
}
```

## Examples

### Default

<ComponentPreview name="separator-demo"/>

### Vertical

Use the `orientation` prop to create a vertical separator.

<ComponentPreview name="separator-vertical"/>
