---
title: Calendar
description: A component that allows users to select a date from a monthly calendar grid.
component: true
---

<ComponentPreview name="calendar-demo"/>

## Usage

```rust
use ui::Calendar;

rsx! {
    Calendar { default_year: 2025, default_month: 5 }
}
```

## Examples

### Default

<ComponentPreview name="calendar-default"/>

### With Selected Date

Pre-select a date when the calendar mounts.

<ComponentPreview name="calendar-with-selected-date"/>

