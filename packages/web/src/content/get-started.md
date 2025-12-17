---
title: Installation
description: Install and configure shadcn for Dioxus.
---

### Create project

Use the Dioxus CLI to create a project.

<PmBlock command="dx new my_app"/>

Choose between the proposed templates.

If working with a workspace, remember to add a reference to your ui package from within your app's css.

```css
@source "./src/**/*.{rs,html,css}";

/* Add this line */
@source "../ui/src/**/*.{rs,html,css}";
```

### Add Components

You can now start adding components to your project. Use the Dioxus CLI with the desired output path.

<PmBlock command="dx components add --git {{git_url}} --path src/components/ui button"/>

The command above will add the <code>button</code> component to your project. You can then import it like this:

```rust
use crate::components::ui::Button;

pub fn MyComponent() -> Element {
    rsx! {
        Button {
            "Click me"
        }
    }
}
```
