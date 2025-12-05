---
title: Card
description: Displays a card with header, content, and footer.
component: true
---

<ComponentPreview name="card-demo"/>

## Installation

```bash
No easy installation yet
```

## Usage

```rust
use ui::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};

rsx! {
    Card {
        CardHeader {
            CardTitle { "Card Title" }
            CardDescription { "Card description." }
        }
        CardContent {
            p { "Card content goes here." }
        }
        CardFooter {
            Button { "Action" }
        }
    }
}
```

## Structure

The Card component is a compound component with the following parts:

- `Card` - The container
- `CardHeader` - Header section
- `CardTitle` - Title within header
- `CardDescription` - Description within header
- `CardContent` - Main content area
- `CardFooter` - Footer section
- `CardAction` - Action buttons area
