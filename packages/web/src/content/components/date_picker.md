---
title: Date Picker
description: A dropdown input that opens a calendar for selecting a specific date.
component: true
---

<ComponentPreview name="date-picker-demo"/>

## Usage

```rust
use ui::DatePicker;

rsx! {
    DatePicker { placeholder: "Pick a date".to_string() }
}
```

## Examples

### Default

<ComponentPreview name="date-picker-default"/>

### With Presets

Provide quick date range preset buttons.

<ComponentPreview name="date-picker-with-presets"/>

