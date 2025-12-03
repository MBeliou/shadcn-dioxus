---
title: Button
description: Displays a button or a component that looks like a button.
component: true
links:
  source: https://github.com/your-repo/components/button
  doc: https://your-docs.com/button
  api: https://your-docs.com/button#api
---

<ComponentPreview name="button-demo"/>

## Installation

```bash
No easy installation yet
```

## Usage

```rust
use crate::components::ui::{Button, ButtonVariant};

rsx! {
    Button { variant: ButtonVariant::Outline, "Button" }
}
```

## Link

You can convert the `<button>` into an `&lt;a&gt;` element by simply passing an `href` as a prop.

```rust
use crate::components::ui::Button;

rsx! {
    Button { href:"/dashboard", "Dashboard" }
}
```

Alternatively, you can use the `buttonVariants` helper to create a link that looks like a button.

```rust
use crate::components::ui::{Button, button_variants};

rsx! {
    Button { 
      href:"/dashboard", 
      class: button_variants({ variant: ButtonVariant::Outline }),
      "Dashboard"
      }
}
```

## Examples

### Primary

<ComponentPreview name="button-primary"/>

### Secondary

<ComponentPreview name="button-secondary"/>
