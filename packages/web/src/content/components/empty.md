---
title: Empty
description: Displays an empty state interface, useful for showing when no data or content is available.
component: true
---

<ComponentPreview name="empty-demo"/>

## Installation

```bash
npx shadcn-dioxus add empty
```

## Usage

```rust
use ui::{Empty, EmptyHeader, EmptyMedia, EmptyTitle, EmptyDescription, EmptyContent, Button};

rsx! {
    Empty {
        EmptyHeader {
            EmptyMedia {
                // Icon here
            }
            EmptyTitle { "No data" }
            EmptyDescription { "No data found" }
        }
        EmptyContent {
            Button { "Add data" }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="empty-demo"/>

### Structure

The Empty component consists of:

- **Empty** - Main container with flex layout
- **EmptyHeader** - Centered content area for icon, title, and description
- **EmptyMedia** - Displays icons or avatars
- **EmptyTitle** - Heading text
- **EmptyDescription** - Supporting text with muted styling
- **EmptyContent** - Action area for buttons or forms

### With Actions

```rust
use ui::{Empty, EmptyHeader, EmptyTitle, EmptyDescription, EmptyContent, Button, ButtonVariant};

rsx! {
    Empty { class: "border border-dashed",
        EmptyHeader {
            EmptyTitle { "No projects found" }
            EmptyDescription { "Get started by creating your first project." }
        }
        EmptyContent {
            Button { "Create Project" }
            Button { variant: ButtonVariant::Outline, "Learn More" }
        }
    }
}
```
