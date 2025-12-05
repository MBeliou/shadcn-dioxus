---
title: Kbd
description: Displays keyboard input or shortcuts.
component: true
---

<ComponentPreview name="kbd-demo"/>

## Installation

```bash
No easy installation yet
```

## Usage

```rust
use ui::Kbd;

rsx! {
    Kbd { "⌘" }
    Kbd { "K" }
}
```

## Examples

### Keyboard Shortcut

```rust
use ui::Kbd;

rsx! {
    div { class: "flex items-center gap-1",
        Kbd { "⌘" }
        Kbd { "Shift" }
        Kbd { "P" }
    }
}
```
