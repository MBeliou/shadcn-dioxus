---
title: Switch
description: A control that allows the user to toggle between checked and not checked.
component: true
---

<ComponentPreview name="switch-demo"/>

## Usage

```rust
use ui::Switch;

rsx! {
    Switch {}
}
```

## Examples

### Default

<ComponentPreview name="switch-demo"/>

### With Label

Use the `Label` component alongside `Switch` to create an accessible switch with a label.

<ComponentPreview name="switch-with-label"/>

### Disabled

<ComponentPreview name="switch-disabled"/>
