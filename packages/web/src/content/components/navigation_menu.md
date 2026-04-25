---
title: Navigation Menu
description: A collection of links for navigating websites, supporting dropdown sub-menus.
component: true
---

<ComponentPreview name="navigation-menu-demo"/>

## Usage

```rust
use ui::{NavigationMenu, NavigationMenuList, NavigationMenuItem, NavigationMenuTrigger, NavigationMenuContent, NavigationMenuLink};

rsx! {
    NavigationMenu {
        NavigationMenuList {
            NavigationMenuItem {
                NavigationMenuTrigger { "Getting Started" }
                NavigationMenuContent {
                    NavigationMenuLink { href: "/docs", "Introduction" }
                }
            }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="navigation-menu-default"/>

### Horizontal

Orient the navigation menu horizontally.

<ComponentPreview name="navigation-menu-horizontal"/>

