---
title: Table
description: A semantic HTML table component for displaying structured tabular data.
component: true
---

<ComponentPreview name="table-demo"/>

## Usage

```rust
use ui::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell, TableCaption};

rsx! {
    Table {
        TableCaption { "Invoice Details" }
        TableHeader {
            TableRow {
                TableHead { "Invoice" }
                TableHead { "Status" }
            }
        }
        TableBody {
            TableRow {
                TableCell { "#INV001" }
                TableCell { "Paid" }
            }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="table-default"/>

### With Sorting

Add interactive sorting controls to table headers.

<ComponentPreview name="table-with-sorting"/>

