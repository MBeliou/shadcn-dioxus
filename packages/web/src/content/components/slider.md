---
title: Slider
description: An input where the user selects a value from within a given range by dragging a handle.
component: true
---

<ComponentPreview name="slider-demo"/>

## Usage

```rust
use ui::Slider;

rsx! {
    Slider { default_value: vec![50.0] }
}
```

## Examples

### Default

<ComponentPreview name="slider-default"/>

### With Min/Max Steps

Define specific step increments and boundaries.

<ComponentPreview name="slider-with-min/max-steps"/>

