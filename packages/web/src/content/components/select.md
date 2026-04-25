---
title: Select
description: A dropdown control for picking a single value from a list of predefined options.
component: true
---

<ComponentPreview name="select-demo"/>

## Usage

```rust
use ui::{Select, SelectTrigger, SelectValue, SelectContent, SelectItem};

rsx! {
    Select {
        SelectTrigger {
            SelectValue { placeholder: "Select a fruit" }
        }
        SelectContent {
            SelectItem { value: "apple", "Apple" }
            SelectItem { value: "banana", "Banana" }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="select-default"/>

### With Label

Add an accessible label to the select trigger.

<ComponentPreview name="select-with-label"/>

### Disabled

Prevent interaction with specific items or the whole select.

<ComponentPreview name="select-disabled"/>

