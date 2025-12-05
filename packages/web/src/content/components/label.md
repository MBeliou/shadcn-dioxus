---
title: Label
description: Renders an accessible label associated with controls.
component: true
---

<ComponentPreview name="label-demo"/>

## Installation

```bash
No easy installation yet
```

## Usage

```rust
use ui::Label;

rsx! {
    Label { "Email" }
}
```

## With Input

```rust
use ui::{Label, Input};

rsx! {
    div { class: "grid w-full max-w-sm items-center gap-1.5",
        Label { r#for: "email", "Email" }
        Input { r#type: InputType::Email, id: "email", placeholder: "Email" }
    }
}
```
