---
title: Skeleton
description: Use to show a placeholder while content is loading.
component: true
---

<ComponentPreview name="skeleton-demo"/>

## Installation

```bash
No easy installation yet
```

## Usage

```rust
use ui::Skeleton;

rsx! {
    Skeleton { class: "h-4 w-[250px]" }
}
```

## Examples

### Card Skeleton

```rust
use ui::Skeleton;

rsx! {
    div { class: "flex items-center space-x-4",
        Skeleton { class: "h-12 w-12 rounded-full" }
        div { class: "space-y-2",
            Skeleton { class: "h-4 w-[250px]" }
            Skeleton { class: "h-4 w-[200px]" }
        }
    }
}
```
