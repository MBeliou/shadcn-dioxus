---
title: Menubar
description: A horizontal bar displaying menus and their associated actions, akin to desktop apps.
component: true
---

<ComponentPreview name="menubar-demo"/>

## Usage

```rust
use ui::{Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem, MenubarSeparator};

rsx! {
    Menubar {
        MenubarMenu {
            MenubarTrigger { "File" }
            MenubarContent {
                MenubarItem { "New Tab" }
                MenubarSeparator {}
                MenubarItem { "Save" }
            }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="menubar-default"/>

