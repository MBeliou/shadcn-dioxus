---
title: Combobox
description: A combination of a text input and a dropdown list for filtering options.
component: true
---

<ComponentPreview name="combobox-demo"/>

## Usage

```rust
use ui::Combobox;

rsx! {
    Combobox { placeholder: "Select a framework...".to_string() }
}
```

## Examples

### Default

<ComponentPreview name="combobox-default"/>

### With Group

Group related options together in the dropdown list.

<ComponentPreview name="combobox-with-group"/>

