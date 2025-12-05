---
title: Card
description: Displays a card with header, content, and footer.
component: true
---

<ComponentPreview name="card-demo"/>

## Installation

```bash
npx shadcn-dioxus add card
```

## Usage

```rust
use ui::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};

rsx! {
    Card {
        CardHeader {
            CardTitle { "Card Title" }
            CardDescription { "Card Description" }
        }
        CardContent {
            p { "Card Content" }
        }
        CardFooter {
            p { "Card Footer" }
        }
    }
}
```

## Structure

The Card component is a compound component with the following parts:

- `Card` - The main container
- `CardHeader` - Contains the title and description
- `CardTitle` - The card title
- `CardDescription` - A muted description below the title
- `CardContent` - The main content area
- `CardFooter` - The footer area for actions
- `CardAction` - Optional action element in the header

## Examples

### Default

<ComponentPreview name="card-demo"/>
