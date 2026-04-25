---
title: Hover Card
description: A floating card anchored to a trigger element, revealed on hover.
component: true
---

<ComponentPreview name="hover-card-demo"/>

## Usage

```rust
use ui::{HoverCard, HoverCardTrigger, HoverCardContent};

rsx! {
    HoverCard {
        HoverCardTrigger { rsx! { span { "@shadcn" } } }
        HoverCardContent {
            div { class: "flex gap-4",
                div { class: "font-bold", "Shadcn UI" }
            }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="hover-card-default"/>

