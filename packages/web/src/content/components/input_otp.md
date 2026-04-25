---
title: Input OTP
description: A specialized input for One-Time Passwords, separating characters into distinct slots.
component: true
---

<ComponentPreview name="input-otp-demo"/>

## Usage

```rust
use ui::InputOtp;

rsx! {
    InputOtp { length: 6 }
}
```

## Examples

### Default

<ComponentPreview name="input-otp-default"/>

### With Separator

Add a visual separator between groups of characters.

<ComponentPreview name="input-otp-with-separator"/>

### Controlled

Manage the OTP state programmatically.

<ComponentPreview name="input-otp-controlled"/>

