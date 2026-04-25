---
title: Data Table
description: A robust table component with sorting, filtering, and pagination built-in.
component: true
---

<ComponentPreview name="data-table-demo"/>

## Usage

```rust
use ui::DataTable;

rsx! {
    DataTable {}
}
```

## Examples

### Basic

A simple table layout without controls.

<ComponentPreview name="data-table-basic"/>

### Sorting & Filtering

Enable column sorting and text filtering.

<ComponentPreview name="data-table-sorting-&-filtering"/>

### Pagination

Paginate large datasets client-side.

<ComponentPreview name="data-table-pagination"/>

