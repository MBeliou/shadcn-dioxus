---
title: Resizable
description: Interactive container panels that can be resized by dragging the divider.
component: true
---

<ComponentPreview name="resizable-demo"/>

## Usage

```rust
use ui::{Resizable, ResizablePanel, ResizableHandle};

rsx! {
    Resizable {
        ResizablePanel { default_size: 50 }
        ResizableHandle {}
        ResizablePanel { default_size: 50 }
    }
}
```

## Examples

### Horizontal

Resize horizontally side-by-side.

<ComponentPreview name="resizable-horizontal"/>

### Vertical

Resize vertically stacked panels.

<ComponentPreview name="resizable-vertical"/>

### With Handle

Customize the appearance of the drag handle.

<ComponentPreview name="resizable-with-handle"/>

