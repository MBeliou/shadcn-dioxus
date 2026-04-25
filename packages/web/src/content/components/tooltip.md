---
title: Tooltip
description: A small popup box that appears when hovering over an element, providing extra context.
component: true
---

<ComponentPreview name="tooltip-demo"/>

## Usage

```rust
use ui::{Tooltip, TooltipTrigger, TooltipContent, TooltipProvider};

rsx! {
    TooltipProvider {
        Tooltip {
            TooltipTrigger { rsx! { button { "Hover" } } }
            TooltipContent {
                p { "This is a tooltip" }
            }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="tooltip-default"/>

### With Arrow

Display a pointing arrow indicating the trigger element.

<ComponentPreview name="tooltip-with-arrow"/>

