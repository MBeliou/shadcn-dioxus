---
title: Toggle Group
description: A set of two-button toggles that can be switched on or off individually or as a group.
component: true
---

<ComponentPreview name="toggle-group-demo"/>

## Usage

```rust
use ui::{ToggleGroup, ToggleGroupItem};

rsx! {
    ToggleGroup { type_: "single".to_string(),
        ToggleGroupItem { value: "a", "A" }
        ToggleGroupItem { value: "b", "B" }
    }
}
```

## Examples

### Single

Only one item can be active at a time.

<ComponentPreview name="toggle-group-single"/>

### Multiple

Allow multiple items to be toggled on simultaneously.

<ComponentPreview name="toggle-group-multiple"/>

### Outline

Styling variant for toggle items.

<ComponentPreview name="toggle-group-outline"/>

