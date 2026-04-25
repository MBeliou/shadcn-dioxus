---
title: Alert
description: Displays a callout for important messages to the user.
component: true
---

<ComponentPreview name="alert-demo"/>

## Usage

```rust
use ui::{Alert, AlertTitle, AlertDescription};

rsx! {
    Alert {
        AlertTitle { "Heads up!" }
        AlertDescription { "You can add components to your app using the cli." }
    }
}
```

## Examples

### Default

<ComponentPreview name="alert-default"/>

### Destructive

Alerts with a destructive variant for dangerous actions.

<ComponentPreview name="alert-destructive"/>

### With Icon

Include an icon alongside your alert text.

<ComponentPreview name="alert-with-icon"/>

