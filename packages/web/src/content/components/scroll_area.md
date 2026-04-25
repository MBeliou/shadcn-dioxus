---
title: Scroll Area
description: Enhances native scrollbars with custom styling for overflowing content.
component: true
---

<ComponentPreview name="scroll-area-demo"/>

## Usage

```rust
use ui::ScrollArea;

rsx! {
    ScrollArea {
        div { class: "h-[200px] w-[350px] rounded-md border p-4",
            (0..50).map(|i| rsx! { p { "Item {i}" } })
        }
    }
}
```

## Examples

### Horizontal

Enable horizontal scrolling.

<ComponentPreview name="scroll-area-horizontal"/>

### Vertical

Standard vertical scrolling with styled scrollbars.

<ComponentPreview name="scroll-area-vertical"/>

