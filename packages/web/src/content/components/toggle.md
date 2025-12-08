---
title: Toggle
description: A two-state button that can be either on or off.
component: true
---

<ComponentPreview name="toggle-demo"/>

## Usage

```rust
use ui::Toggle;

rsx! {
    Toggle { aria_label: "Toggle bold",
        Bold {}
    }
}
```

## Examples

### Default

<ComponentPreview name="toggle-demo"/>

### Outline

Use the `variant` prop to set the outline style.

<ComponentPreview name="toggle-outline"/>

### With Text

<ComponentPreview name="toggle-with-text"/>

### Small

<ComponentPreview name="toggle-sm"/>

### Large

<ComponentPreview name="toggle-lg"/>

### Disabled

<ComponentPreview name="toggle-disabled"/>
