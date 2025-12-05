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

Use the `KbdGroup` component to group keyboard keys together.

<ComponentPreview name="kbd-group"/>

### Button

Use the `Kbd` component inside a `Button` component to display a keyboard key inside a button.

<ComponentPreview name="kbd-button"/>
