---
title: Chart
description: A component for rendering data visualizations like bar charts.
component: true
---

<ComponentPreview name="chart-demo"/>

## Usage

```rust
use ui::Chart;

rsx! {
    Chart {}
}
```

## Examples

### Bar Chart

Display data using vertical bars.

<ComponentPreview name="chart-bar-chart"/>

### Custom Colors

Override the default color palette for bars.

<ComponentPreview name="chart-custom-colors"/>

