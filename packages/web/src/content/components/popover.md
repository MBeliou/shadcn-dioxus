---
title: Popover
description: A floating panel anchored to a target element, activated by click.
component: true
---

<ComponentPreview name="popover-demo"/>

## Usage

```rust
use ui::{Popover, PopoverTrigger, PopoverContent};

rsx! {
    Popover {
        PopoverTrigger { rsx! { button { "Open" } } }
        PopoverContent {
            div { class: "grid gap-4",
                div { class: "space-y-2",
                    h4 { class: "font-medium", "Dimensions" }
                    p { class: "text-sm text-muted-foreground", "Set the dimensions for the layer." }
                }
            }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="popover-default"/>

