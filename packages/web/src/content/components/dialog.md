---
title: Dialog
description: A window overlaid on either the primary window or another dialog window, rendering the content underneath inert.
component: true
---

<ComponentPreview name="dialog-demo"/>

## Usage

```rust
use ui::{
    Dialog, DialogTrigger, DialogPortal, DialogOverlay, DialogContent,
    DialogHeader, DialogTitle, DialogDescription, DialogFooter, DialogClose,
    Button,
};

rsx! {
    Dialog {
        DialogTrigger {
            Button { "Open Dialog" }
        }
        DialogPortal {
            DialogOverlay {}
            DialogContent {
                DialogHeader {
                    DialogTitle { "Are you absolutely sure?" }
                    DialogDescription {
                        "This action cannot be undone."
                    }
                }
            }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="dialog-demo"/>

### Confirmation

Use the `DialogClose` component to wrap buttons that should close the dialog when clicked.

<ComponentPreview name="dialog-form"/>
