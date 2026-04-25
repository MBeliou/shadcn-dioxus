---
title: Collapsible
description: An interactive component which expands/collapses a content panel.
component: true
---

<ComponentPreview name="collapsible-demo"/>

## Usage

```rust
use ui::{Collapsible, CollapsibleTrigger, CollapsibleContent};

rsx! {
    Collapsible {
        CollapsibleTrigger { "Open Panel" }
        CollapsibleContent {
            div { class: "p-4", "This content is collapsible." }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="collapsible-default"/>

### Animated

Collapsible panels with smooth height transitions.

<ComponentPreview name="collapsible-animated"/>

