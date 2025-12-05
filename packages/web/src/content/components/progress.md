---
title: Progress
description: Displays an indicator showing the completion progress of a task.
component: true
---

<ComponentPreview name="progress-demo"/>

## Installation

```bash
No easy installation yet
```

## Usage

```rust
use ui::Progress;

rsx! {
    Progress { value: 60.0 }
}
```

## Examples

### Indeterminate

When no value is provided, the progress bar is in an indeterminate state.

<ComponentPreview name="progress-indeterminate"/>
