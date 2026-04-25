---
title: Radio Group
description: A set of checkable buttons where only one can be selected at a time.
component: true
---

<ComponentPreview name="radio-group-demo"/>

## Usage

```rust
use ui::{RadioGroup, RadioGroupItem, RadioGroupLabel};

rsx! {
    RadioGroup {
        RadioGroupItem { value: "default", "Default" }
        RadioGroupItem { value: "comfortable", "Comfortable" }
        RadioGroupItem { value: "compact", "Compact" }
    }
}
```

## Examples

### Default

<ComponentPreview name="radio-group-default"/>

