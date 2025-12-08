---
title: Native Select
description: A styled native HTML select element with consistent design system integration.
component: true
---

<ComponentPreview name="native-select-demo"/>

## Usage

```rust
use ui::{NativeSelect, NativeSelectOption};

rsx! {
    NativeSelect {
        NativeSelectOption { value: "", "Select a fruit" }
        NativeSelectOption { value: "apple", "Apple" }
        NativeSelectOption { value: "banana", "Banana" }
        NativeSelectOption { value: "blueberry", "Blueberry" }
        NativeSelectOption { value: "grapes", disabled: true, "Grapes" }
        NativeSelectOption { value: "pineapple", "Pineapple" }
    }
}
```

## Examples

### Default

<ComponentPreview name="native-select-demo"/>

### With OptGroup

Use `NativeSelectOptGroup` to organize related options.

<ComponentPreview name="native-select-optgroup"/>

### Disabled

<ComponentPreview name="native-select-disabled"/>

### Invalid

Use `aria-invalid="true"` to indicate validation errors.

<ComponentPreview name="native-select-invalid"/>
