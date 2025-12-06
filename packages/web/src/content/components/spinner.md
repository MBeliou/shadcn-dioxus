---
title: Spinner
description: An indicator that can be used to show a loading state.
component: true
---

<ComponentPreview name="spinner-demo"/>

## Usage

```rust
use ui::Spinner;

rsx! {
    Spinner {}
}
```

## Examples

### Default

<ComponentPreview name="spinner-demo"/>

### Sizes

Apply Tailwind size utilities to adjust spinner dimensions:

```rust
use ui::Spinner;

rsx! {
    Spinner { class: "size-3" }
    Spinner { class: "size-4" }
    Spinner { class: "size-6" }
    Spinner { class: "size-8" }
}
```

### Colors

Use text color utilities to change appearance:

```rust
use ui::Spinner;

rsx! {
    Spinner { class: "text-red-500" }
    Spinner { class: "text-green-500" }
    Spinner { class: "text-blue-500" }
}
```

### With Button

Combine with Button component for loading states:

```rust
use ui::{Button, ButtonSize, Spinner};

rsx! {
    Button { disabled: true, size: ButtonSize::Sm,
        Spinner {}
        "Loading..."
    }
}
```
