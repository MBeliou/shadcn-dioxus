---
title: Avatar
description: An image element with a fallback for representing the user.
component: true
---

<ComponentPreview name="avatar-demo"/>

## Installation

```bash
npx shadcn-dioxus add avatar
```

## Usage

```rust
use ui::{Avatar, AvatarImage, AvatarFallback};

rsx! {
    Avatar {
        AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
        AvatarFallback { "CN" }
    }
}
```

## Structure

The Avatar component is a compound component with the following parts:

- `Avatar` - The main container with loading state management
- `AvatarImage` - Displays the avatar image
- `AvatarFallback` - Renders initials or placeholder when image fails to load

## Examples

### Default

<ComponentPreview name="avatar-demo"/>

### Fallback

The fallback is displayed when the image fails to load or while it's loading.

<ComponentPreview name="avatar-fallback"/>
