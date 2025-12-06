---
title: Input
description: Displays a form input field or a component that looks like an input field.
component: true
---

<ComponentPreview name="input-demo"/>

## Usage

```rust
use ui::Input;

rsx! {
    Input { r#type: InputType::Email, placeholder: "Email" }
}
```

## Examples

### Default

<ComponentPreview name="input-demo"/>

### Disabled

<ComponentPreview name="input-disabled"/>

### With Label

```rust
use ui::{Input, Label};

rsx! {
    div { class: "grid w-full max-w-sm items-center gap-1.5",
        Label { r#for: "email", "Email" }
        Input { r#type: InputType::Email, id: "email", placeholder: "Email" }
    }
}
```

### File

```rust
use ui::{Input, Label};

rsx! {
    div { class: "grid w-full max-w-sm items-center gap-1.5",
        Label { r#for: "picture", "Picture" }
        Input { id: "picture", r#type: InputType::File }
    }
}
```
