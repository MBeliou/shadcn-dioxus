---
title: Button Group
description: A container that groups related buttons together with consistent styling.
component: true
---

<ComponentPreview name="button-group-demo"/>

## Installation

```bash
npx shadcn-dioxus add button-group
```

## Usage

```rust
use ui::{ButtonGroup, Button};

rsx! {
    ButtonGroup {
        Button { "Button 1" }
        Button { "Button 2" }
    }
}
```

## Examples

### Default

<ComponentPreview name="button-group-demo"/>

### Vertical

Groups buttons in a column layout:

```rust
use ui::{ButtonGroup, ButtonGroupOrientation, Button, ButtonVariant};

rsx! {
    ButtonGroup { orientation: ButtonGroupOrientation::Vertical,
        Button { variant: ButtonVariant::Outline, "Top" }
        Button { variant: ButtonVariant::Outline, "Middle" }
        Button { variant: ButtonVariant::Outline, "Bottom" }
    }
}
```

### With Separator

Use `ButtonGroupSeparator` to visually divide buttons within a group:

```rust
use ui::{ButtonGroup, ButtonGroupSeparator, Button, ButtonVariant};

rsx! {
    ButtonGroup {
        Button { variant: ButtonVariant::Outline, "Action" }
        ButtonGroupSeparator {}
        Button { variant: ButtonVariant::Outline, "▼" }
    }
}
```

## Accessibility

- Component includes `role="group"` attribute
- Use `aria-label` for proper labeling
