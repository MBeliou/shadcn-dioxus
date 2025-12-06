---
title: Progress
description: Displays an indicator showing the completion progress of a task.
component: true
---

<ComponentPreview name="progress-demo"/>

## Usage

```rust
use ui::Progress;

rsx! {
    Progress { value: 33.0 }
}
```

## Examples

### Default

<ComponentPreview name="progress-demo"/>

### Indeterminate

When no value is provided, the progress bar is in an indeterminate state.

<ComponentPreview name="progress-indeterminate"/>
