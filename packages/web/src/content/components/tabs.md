---
title: Tabs
description: A set of layered sections of content, known as tab panels, displayed one at a time.
component: true
---

<ComponentPreview name="tabs-demo"/>

## Usage

```rust
use ui::{Tabs, TabsList, TabsTrigger, TabsContent};

rsx! {
    Tabs { default_value: "account",
        TabsList {
            TabsTrigger { value: "account", "Account" }
            TabsTrigger { value: "password", "Password" }
        }
        TabsContent { value: "account",
            p { "Make changes to your account here." }
        }
    }
}
```

## Examples

### Default

<ComponentPreview name="tabs-default"/>

