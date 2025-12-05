---
title: Empty
description: Displays an empty state with a message and optional actions.
component: true
---

<ComponentPreview name="empty-demo"/>

## Installation

```bash
No easy installation yet
```

## Usage

```rust
use ui::{Empty, EmptyHeader, EmptyTitle, EmptyDescription};

rsx! {
    Empty {
        EmptyHeader {
            EmptyTitle { "No results found" }
            EmptyDescription { "Try adjusting your search or filters." }
        }
    }
}
```

## Structure

The Empty component is a compound component with the following parts:

- `Empty` - The container
- `EmptyHeader` - Header section
- `EmptyTitle` - Title text
- `EmptyDescription` - Description text
- `EmptyMedia` - Media/icon area
- `EmptyContent` - Content area for actions
