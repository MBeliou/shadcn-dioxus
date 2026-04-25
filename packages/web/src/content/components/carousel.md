---
title: Carousel
description: A slideshow component for cycling through images or content cards.
component: true
---

<ComponentPreview name="carousel-demo"/>

## Usage

```rust
use ui::{Carousel, CarouselContent, CarouselItem, CarouselPrevious, CarouselNext};

rsx! {
    Carousel {
        CarouselContent {
            CarouselItem { img { src: "https://picsum.photos/1", class: "w-full" } }
            CarouselItem { img { src: "https://picsum.photos/2", class: "w-full" } }
        }
        CarouselPrevious {}
        CarouselNext {}
    }
}
```

## Examples

### Default

<ComponentPreview name="carousel-default"/>

### Autoplay

Automatically advance slides after a set interval.

<ComponentPreview name="carousel-autoplay"/>

### Orientation

Switch between horizontal and vertical slide directions.

<ComponentPreview name="carousel-orientation"/>

