---
title: Breadcrumb
description: Displays the path to the current resource using a hierarchy of links.
component: true
---

<ComponentPreview name="breadcrumb-default"/>

## Usage

```rust
use ui::{Breadcrumb, BreadcrumbList, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator};

rsx! {
    Breadcrumb {
        BreadcrumbList {
            BreadcrumbItem { BreadcrumbLink { href: "/", "Home" } }
            BreadcrumbSeparator {}
            BreadcrumbItem { BreadcrumbPage { "Components" } }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="breadcrumb-default"/>

### Separator

Customize the separator between items.

<ComponentPreview name="breadcrumb-separator"/>

### Collapsible

Collapse breadcrumb items when they overflow.

<ComponentPreview name="breadcrumb-collapsible"/>
