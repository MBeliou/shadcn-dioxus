---
title: Context Menu
description: A floating menu that appears on right-click, providing contextual actions.
component: true
---

<ComponentPreview name="context-menu-demo"/>

## Usage

```rust
use ui::{ContextMenu, ContextMenuTrigger, ContextMenuContent, ContextMenuItem};

rsx! {
    ContextMenu {
        ContextMenuTrigger { rsx! { div { class: "p-4 border", "Right Click Me" } } }
        ContextMenuContent {
            ContextMenuItem { "Back" }
            ContextMenuItem { "Forward" }
            ContextMenuItem { "Reload" }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="context-menu-default"/>

### With Submenus

Nest context menus to create hierarchical actions.

<ComponentPreview name="context-menu-with-submenus"/>

