---
title: Input Group
description: Display additional information or actions to an input or textarea.
component: true
---

<ComponentPreview name="input-group-demo"/>

## Usage

```rust
use ui::{InputGroup, InputGroupInput, InputGroupAddon, InputGroupButton};

rsx! {
    InputGroup {
        InputGroupInput { placeholder: "Search..." }
        InputGroupAddon {
            SearchIcon {}
        }
        InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
            InputGroupButton { "Search" }
        }
    }
}
```

## Examples

### Icon

<ComponentPreview name="input-group-icon"/>

### Text

Display additional text information alongside inputs.

<ComponentPreview name="input-group-text"/>

### Button

Add buttons to perform actions within the input group.

<ComponentPreview name="input-group-button"/>

### Textarea

Input groups also work with textarea components. Use `block_start` or `block_end` for alignment.

<ComponentPreview name="input-group-textarea"/>

### Spinner

Show loading indicators while processing input.

<ComponentPreview name="input-group-spinner"/>

### Label

Add labels within input groups to improve accessibility.

<ComponentPreview name="input-group-label"/>

### Button Group

Wrap input groups with button groups to create prefixes and suffixes.

<ComponentPreview name="input-group-button-group"/>

### Custom Input

Add the `data-slot="input-group-control"` attribute to your custom input for automatic behavior and focus state handling.

<ComponentPreview name="input-group-custom-input"/>
