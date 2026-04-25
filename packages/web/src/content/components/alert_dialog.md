---
title: Alert Dialog
description: A modal dialog that interrupts the user with important content and expects a response.
component: true
---

<ComponentPreview name="alert-dialog-demo"/>

## Usage

```rust
use ui::{AlertDialog, AlertDialogTrigger, AlertDialogContent, AlertDialogHeader, AlertDialogTitle, AlertDialogDescription, AlertDialogFooter, AlertDialogAction, AlertDialogCancel};

rsx! {
    AlertDialog {
        AlertDialogTrigger { rsx! { button { "Open" } } }
        AlertDialogContent {
            AlertDialogHeader {
                AlertDialogTitle { "Are you sure?" }
                AlertDialogDescription { "This action cannot be undone." }
            }
            AlertDialogFooter {
                AlertDialogCancel { "Cancel" }
                AlertDialogAction { "Continue" }
            }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="alert-dialog-default"/>

### With Row

Place actions in a row inside the footer.

<ComponentPreview name="alert-dialog-with-row"/>

