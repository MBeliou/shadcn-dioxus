---
title: Field
description: Combine labels, controls, and help text to compose accessible form fields and grouped inputs.
component: true
---

<ComponentPreview name="field-demo"/>

## Usage

```rust
use ui::{Field, FieldLabel, FieldDescription, FieldError, Input};

rsx! {
    Field {
        FieldLabel { "for": "email", "Email" }
        Input { id: "email", r#type: InputType::Email, placeholder: "Enter your email" }
        FieldDescription { "We'll never share your email." }
    }
}
```

## Structure

The Field component family provides building blocks for accessible form fields:

- `Field` - Core wrapper with orientation support (vertical, horizontal, responsive)
- `FieldLabel` - Label associated with the control
- `FieldDescription` - Optional helper text
- `FieldError` - Validation error messages
- `FieldContent` - Flex container for label and description
- `FieldGroup` - Container for multiple fields
- `FieldSet` - Semantic fieldset element
- `FieldLegend` - Legend for fieldsets
- `FieldSeparator` - Visual divider between fields
- `FieldTitle` - Title text within fields

## Examples

### Input Field

A basic input field with label and description.

<ComponentPreview name="field-demo"/>

### Textarea

Multi-line text input with description.

<ComponentPreview name="field-textarea"/>

### FieldSet

Group related fields with a legend.

<ComponentPreview name="field-set-demo"/>

### Checkbox

Checkbox control with Field wrapper for proper layout.

<ComponentPreview name="field-checkbox"/>

### Switch

Toggle switch with description text.

<ComponentPreview name="field-switch"/>

## Orientation

The `Field` component supports three orientations:

```rust
use ui::{Field, FieldOrientation, FieldLabel, Input};

rsx! {
    // Vertical (default)
    Field {
        FieldLabel { "Name" }
        Input { placeholder: "Enter name" }
    }

    // Horizontal
    Field { orientation: FieldOrientation::Horizontal,
        FieldLabel { "Name" }
        Input { placeholder: "Enter name" }
    }

    // Responsive (vertical on mobile, horizontal on larger screens)
    Field { orientation: FieldOrientation::Responsive,
        FieldLabel { "Name" }
        Input { placeholder: "Enter name" }
    }
}
```

## Validation

Display error messages using `FieldError`:

```rust
use ui::{Field, FieldLabel, FieldError, FieldErrorMessage, Input};

rsx! {
    Field {
        FieldLabel { "for": "username", "Username" }
        Input { id: "username", "aria-invalid": "true" }
        FieldError {
            errors: vec![
                FieldErrorMessage { message: Some("Username is required".into()) }
            ]
        }
    }
}
```

Multiple errors are automatically rendered as a list.
