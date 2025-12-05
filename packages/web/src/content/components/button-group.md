---
title: ButtonGroup
description: Groups multiple buttons together.
component: true
---

<ComponentPreview name="button-group-demo"/>

## Installation

```bash
No easy installation yet
```

## Usage

```rust
use ui::{ButtonGroup, Button, ButtonVariant};

rsx! {
    ButtonGroup::Root {
        Button { variant: ButtonVariant::Outline, "Left" }
        Button { variant: ButtonVariant::Outline, "Center" }
        Button { variant: ButtonVariant::Outline, "Right" }
    }
}
```

## Structure

The ButtonGroup component is a compound component with the following parts:

- `ButtonGroup::Root` - The container
- `ButtonGroup::Text` - Text element within the group
- `ButtonGroup::Separator` - Separator between buttons
