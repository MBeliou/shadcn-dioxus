---
title: Sheet
description: Extends the Dialog component to display content that complements the main content of the screen.
component: true
---

<ComponentPreview name="sheet-demo"/>

## Usage

```rust
use ui::{
    Sheet, SheetTrigger, SheetPortal, SheetOverlay, SheetContent,
    SheetHeader, SheetTitle, SheetDescription, SheetFooter, SheetClose,
    Button, Side,
};

rsx! {
    Sheet {
        SheetTrigger {
            Button { "Open Sheet" }
        }
        SheetPortal {
            SheetOverlay {}
            SheetContent {
                side: Side::Right,
                SheetHeader {
                    SheetTitle { "Edit profile" }
                    SheetDescription {
                        "Make changes to your profile here."
                    }
                }
            }
        }
    }
}
```

## Examples

### Side

Use the `side` property to specify the edge of the screen where the sheet will appear. Options are `Top`, `Right`, `Bottom`, or `Left`.

<ComponentPreview name="sheet-side"/>
