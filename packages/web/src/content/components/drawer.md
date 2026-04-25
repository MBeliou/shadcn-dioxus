---
title: Drawer
description: A panel that slides in from the edge of the screen to show supplementary content.
component: true
---

<ComponentPreview name="drawer-demo"/>

## Usage

```rust
use ui::{Drawer, DrawerTrigger, DrawerContent, DrawerHeader, DrawerTitle, DrawerDescription, DrawerFooter, DrawerClose};

rsx! {
    Drawer {
        DrawerTrigger { rsx! { button { "Open Drawer" } } }
        DrawerContent {
            DrawerHeader {
                DrawerTitle { "Title" }
                DrawerDescription { "Description" }
            }
            DrawerFooter { DrawerClose { "Close" } }
        }
    }
}
```

## Examples

### Right

Slides in from the right side.

<ComponentPreview name="drawer-right"/>

### Left

Slides in from the left side.

<ComponentPreview name="drawer-left"/>

### Bottom

Slides in from the bottom edge.

<ComponentPreview name="drawer-bottom"/>

