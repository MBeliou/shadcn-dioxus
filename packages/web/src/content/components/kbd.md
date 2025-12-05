---
title: Kbd
description: Used to display textual user input from keyboard.
component: true
---

<ComponentPreview name="kbd-demo"/>

## Installation

```bash
npx shadcn-dioxus add kbd
```

## Usage

```rust
use ui::Kbd;

rsx! {
    Kbd { "B" }
}
```

## Examples

### Default

<ComponentPreview name="kbd-demo"/>

### Group

Group multiple keyboard keys together:

```rust
use ui::{Kbd, KbdGroup};

rsx! {
    KbdGroup {
        Kbd { "Ctrl + B" }
        Kbd { "Ctrl + K" }
    }
}
```

### With Button

Embed keyboard indicators within button components:

```rust
use ui::{Button, ButtonVariant, ButtonSize, Kbd};

rsx! {
    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm,
        "Accept "
        Kbd { "⏎" }
    }
}
```
