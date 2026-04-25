---
title: Pagination
description: Navigation controls for splitting content across multiple pages.
component: true
---

<ComponentPreview name="pagination-demo"/>

## Usage

```rust
use ui::{Pagination, PaginationContent, PaginationItem, PaginationPrevious, PaginationNext, PaginationLink};

rsx! {
    Pagination {
        PaginationContent {
            PaginationPrevious {}
            PaginationItem { PaginationLink { href: "#", "1" } }
            PaginationNext {}
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="pagination-default"/>

