---
title: Aspect Ratio
description: Displays content within a desired ratio.
component: true
---

<ComponentPreview name="aspect-ratio-demo"/>

## Usage

```rust
use ui::AspectRatio;

rsx! {
    div { class: "w-[450px]",
        AspectRatio { ratio: 16.0 / 9.0, class: "bg-muted",
            img {
                src: "https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=800&dpr=2&q=80",
                alt: "Photo by Drew Beamer",
                class: "h-full w-full rounded-md object-cover"
            }
        }
    }
}
```
