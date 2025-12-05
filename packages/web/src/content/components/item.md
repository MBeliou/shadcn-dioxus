---
title: Item
description: A straightforward flex container that can house nearly any type of content.
component: true
---

<ComponentPreview name="item-demo"/>

## Installation

```bash
npx shadcn-dioxus add item
```

## Usage

```rust
use ui::{Item, ItemContent, ItemTitle, ItemDescription};

rsx! {
    Item {
        ItemContent {
            ItemTitle { "Item" }
            ItemDescription { "Item description" }
        }
    }
}
```

## Examples

### Basic Item

A simple item with title and description.

<ComponentPreview name="item-demo"/>

### Variants

The Item component supports three styling options.

**Default** - Standard styling with subtle background and borders.

**Outline** - Outlined style with clear borders and transparent background.

**Muted** - Subdued appearance with muted colors for secondary content.

```rust
use ui::{Item, ItemVariant, ItemContent, ItemTitle, ItemDescription};

rsx! {
    Item { variant: ItemVariant::Default,
        ItemContent {
            ItemTitle { "Default" }
            ItemDescription { "Standard styling" }
        }
    }
    Item { variant: ItemVariant::Outline,
        ItemContent {
            ItemTitle { "Outline" }
            ItemDescription { "Outlined style" }
        }
    }
    Item { variant: ItemVariant::Muted,
        ItemContent {
            ItemTitle { "Muted" }
            ItemDescription { "Subdued appearance" }
        }
    }
}
```

### Size

The Item component has different sizes for different use cases.

```rust
use ui::{Item, ItemSize, ItemContent, ItemTitle, ItemDescription};

rsx! {
    Item {
        ItemContent {
            ItemTitle { "Default Size" }
            ItemDescription { "Standard item size" }
        }
    }
    Item { size: ItemSize::Sm,
        ItemContent {
            ItemTitle { "Small Size" }
            ItemDescription { "Compact item size" }
        }
    }
}
```

### Icon

Display items with icon media.

```rust
use ui::{Item, ItemMedia, ItemMediaVariant, ItemContent, ItemTitle, ItemDescription, ItemActions, Button};

rsx! {
    Item {
        ItemMedia { variant: ItemMediaVariant::Icon,
            // Icon here
        }
        ItemContent {
            ItemTitle { "Security Alert" }
            ItemDescription { "Your account security needs attention" }
        }
        ItemActions {
            Button { "Review" }
        }
    }
}
```

### Avatar

Display items with avatar media.

```rust
use ui::{Item, ItemMedia, ItemContent, ItemTitle, ItemDescription, Avatar, AvatarImage, AvatarFallback};

rsx! {
    Item {
        ItemMedia {
            Avatar {
                AvatarImage { src: "/avatar.png" }
                AvatarFallback { "JD" }
            }
        }
        ItemContent {
            ItemTitle { "John Doe" }
            ItemDescription { "john@example.com" }
        }
    }
}
```

### Image

Display items with image media.

```rust
use ui::{Item, ItemMedia, ItemMediaVariant, ItemContent, ItemTitle, ItemDescription};

rsx! {
    Item {
        ItemMedia { variant: ItemMediaVariant::Image,
            img { src: "/album-cover.jpg", alt: "Album" }
        }
        ItemContent {
            ItemTitle { "Song Title" }
            ItemDescription { "Artist Name" }
        }
    }
}
```

### Group

Use `ItemGroup` to create lists of items.

```rust
use ui::{ItemGroup, Item, ItemSeparator, ItemContent, ItemTitle, ItemDescription};

rsx! {
    ItemGroup {
        Item {
            ItemContent {
                ItemTitle { "First Person" }
                ItemDescription { "first@example.com" }
            }
        }
        ItemSeparator {}
        Item {
            ItemContent {
                ItemTitle { "Second Person" }
                ItemDescription { "second@example.com" }
            }
        }
        ItemSeparator {}
        Item {
            ItemContent {
                ItemTitle { "Third Person" }
                ItemDescription { "third@example.com" }
            }
        }
    }
}
```

### Header

Use `ItemHeader` for grid layouts with images.

```rust
use ui::{ItemGroup, Item, ItemHeader, ItemContent, ItemTitle, ItemDescription};

rsx! {
    ItemGroup { class: "grid grid-cols-3 gap-4",
        Item {
            ItemHeader {
                img { src: "/image1.jpg", class: "rounded-lg" }
            }
            ItemContent {
                ItemTitle { "Card Title" }
                ItemDescription { "Card description" }
            }
        }
    }
}
```

### Link

To render an item as a link, wrap the content with an anchor element.

```rust
use ui::{Item, ItemContent, ItemTitle, ItemDescription};

rsx! {
    a { href: "/dashboard",
        Item {
            ItemContent {
                ItemTitle { "Dashboard" }
                ItemDescription { "View your dashboard" }
            }
        }
    }
}
```

