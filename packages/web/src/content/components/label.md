---
title: Label
description: Renders an accessible label associated with controls.
component: true
---

<ComponentPreview name="label-demo"/>

## Usage

```rust
use ui::Label;

rsx! {
    Label { "for": "email", "Your email address" }
}
```

## Examples

### Default

<ComponentPreview name="label-demo"/>

### With Checkbox

Use the `Label` component alongside `Checkbox` to create an accessible checkbox with a label.

<ComponentPreview name="checkbox-with-label"/>
