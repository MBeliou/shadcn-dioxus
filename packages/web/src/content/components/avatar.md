---
title: Avatar
description: An image element with a fallback for representing the user.
component: true
---

<ComponentPreview name="avatar-demo"/>

## Installation

```bash
No easy installation yet
```

## Usage

```rust
use ui::{Avatar, AvatarImage, AvatarFallback};

rsx! {
    Avatar {
        AvatarImage { src: "https://github.com/shadcn.png", alt: "User" }
        AvatarFallback { "CN" }
    }
}
```

## Examples

### Fallback

The fallback is displayed when the image fails to load or while it's loading.

<ComponentPreview name="avatar-fallback"/>
