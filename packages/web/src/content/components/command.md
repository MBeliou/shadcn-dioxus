---
title: Command
description: A floating window for navigating and executing commands, often used in command palettes.
component: true
---

<ComponentPreview name="command-demo"/>

## Usage

```rust
use ui::{Command, CommandInput, CommandList, CommandGroup, CommandItem};

rsx! {
    Command {
        CommandInput { placeholder: "Type a command or search..." }
        CommandList {
            CommandGroup { heading: "Suggestions",
                CommandItem { value: "calendar", "Calendar" }
                CommandItem { value: "search", "Search" }
            }
        }
    }
}
```

## Examples

### Dialog

Use it inside a dialog for a Command Palette.

<ComponentPreview name="command-dialog"/>

### Combobox

Use it as a searchable combobox dropdown.

<ComponentPreview name="command-combobox"/>

