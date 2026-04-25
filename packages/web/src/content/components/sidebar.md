---
title: Sidebar
description: A collapsible side navigation component commonly used in application layouts.
component: true
---

<ComponentPreview name="sidebar-demo"/>

## Usage

```rust
use ui::{Sidebar, SidebarContent, SidebarHeader, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarInset, SidebarProvider, SidebarTrigger};

rsx! {
    SidebarProvider {
        Sidebar {
            SidebarHeader { "Sidebar Title" }
            SidebarContent {
                SidebarGroup {
                    SidebarGroupLabel { "Navigation" }
                    SidebarGroupContent {}
                }
            }
            SidebarFooter {}
        }
        SidebarInset {}
    }
}
```

## Examples

### Default

<ComponentPreview name="sidebar-default"/>

### Collapsible

Allow the sidebar to collapse into a slim rail.

<ComponentPreview name="sidebar-collapsible"/>

