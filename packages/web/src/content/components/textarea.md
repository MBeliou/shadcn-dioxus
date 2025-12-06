---
title: Textarea
description: Displays a form textarea or a component that looks like a textarea.
component: true
---

<ComponentPreview name="textarea-demo"/>

## Usage

```rust
use ui::Textarea;

rsx! {
    Textarea { placeholder: "Type your message here." }
}
```

## Examples

### Default

<ComponentPreview name="textarea-demo"/>

### Disabled

A textarea with the `disabled` attribute applied.

<ComponentPreview name="textarea-disabled"/>

### With Label

<ComponentPreview name="textarea-with-label"/>

### With Button

<ComponentPreview name="textarea-with-button"/>
