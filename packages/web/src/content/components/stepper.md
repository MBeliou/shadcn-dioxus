---
title: Stepper
description: A multi-step sequence UI guiding users through a linear process.
component: true
---

<ComponentPreview name="stepper-demo"/>

## Usage

```rust
use ui::{Stepper, StepperItem, StepperTitle, StepperDescription, StepperSeparator, StepperFooter, StepperPrevious, StepperNext};

rsx! {
    Stepper {
        StepperItem { step: 1,
            StepperTitle { "Step 1" }
            StepperDescription { "Details" }
        }
        StepperSeparator {}
        StepperItem { step: 2,
            StepperTitle { "Step 2" }
            StepperDescription { "Details" }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="stepper-default"/>

### Vertical

Display the stepper in a vertical orientation.

<ComponentPreview name="stepper-vertical"/>

