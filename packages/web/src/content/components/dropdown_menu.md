---
title: Dropdown Menu
description: Displays a menu triggered by a button, offering a list of actions.
component: true
---

<ComponentPreview name="dropdown-menu-demo"/>

## Usage

```rust
use ui::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator};

rsx! {
    DropdownMenu {
        DropdownMenuTrigger { rsx! { button { "Open" } } }
        DropdownMenuContent {
            DropdownMenuLabel { "Actions" }
            DropdownMenuItem { "Edit" }
            DropdownMenuSeparator {}
            DropdownMenuItem { "Delete" }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="dropdown-menu-default"/>

### With Checkboxes

Allow multiple items to be checked within the menu.

<ComponentPreview name="dropdown-menu-with-checkboxes"/>

