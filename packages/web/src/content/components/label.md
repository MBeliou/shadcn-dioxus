---
title: Label
description: Renders an accessible label associated with controls.
component: true
---

<ComponentPreview name="label-demo"/>

## Installation

```bash
npx shadcn-dioxus add label
```

## Usage

```rust
use ui::Label;

rsx! {
    Label { r#for: "email", "Your email address" }
}
```

## Examples

### Default

<ComponentPreview name="label-demo"/>
