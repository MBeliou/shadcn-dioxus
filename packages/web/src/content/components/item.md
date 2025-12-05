---
title: Item
description: A versatile list item component for displaying content with titles, descriptions, and actions.
component: true
---

<ComponentPreview name="item-demo"/>

## Installation

```bash
No easy installation yet
```

## Usage

```rust
use ui::{ItemGroup, Item, ItemContent, ItemTitle, ItemDescription};

rsx! {
    ItemGroup {
        Item {
            ItemContent {
                ItemTitle { "Item Title" }
                ItemDescription { "Item description." }
            }
        }
    }
}
```

## Structure

The Item component is a compound component with the following parts:

- `ItemGroup` - Container for multiple items
- `Item` - Individual item container
- `ItemContent` - Content wrapper
- `ItemTitle` - Title text
- `ItemDescription` - Description text
- `ItemMedia` - Media/icon area
- `ItemActions` - Action buttons area
- `ItemSeparator` - Separator between items
