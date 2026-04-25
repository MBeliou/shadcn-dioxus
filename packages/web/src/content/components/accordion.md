---
title: Accordion
description: A vertically stacked set of interactive headings that reveal or hide associated content panels.
component: true
---

<ComponentPreview name="accordion-demo"/>

## Usage

```rust
use ui::{Accordion, AccordionItem, AccordionContent, AccordionTrigger};

rsx! {
    Accordion { type_: "single".to_string(), collapsible: true,
        AccordionItem { value: "item-1",
            AccordionTrigger { "Is it accessible?" }
            AccordionContent { "Yes. It adheres to the WAI-ARIA design pattern." }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="accordion-default"/>

### Multiple

Allow multiple items to open simultaneously.

<ComponentPreview name="accordion-multiple"/>

