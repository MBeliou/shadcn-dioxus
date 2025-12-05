---
title: Checkbox
description: A control that allows the user to toggle between checked and not checked.
component: true
---

<ComponentPreview name="checkbox-demo"/>

## Installation

```bash
npx shadcn-dioxus add checkbox
```

## Usage

```rust
use ui::Checkbox;

rsx! {
    Checkbox {}
}
```

## Examples

### Default

<ComponentPreview name="checkbox-demo"/>

### With Label

Use the `Label` component alongside `Checkbox` to create an accessible checkbox with a label.

<ComponentPreview name="checkbox-with-label"/>

### Disabled

<ComponentPreview name="checkbox-disabled"/>
