use dioxus::prelude::*;
use lucide_dioxus::{
    ArrowUp, Bold, Check, Copy, CornerDownLeft, CreditCard, FileCode, Info, Italic, Link2, Loader,
    Mail, Plus, RefreshCw, Search, Star, Underline,
};
use ui::*;
pub fn get_demo(name: &str) -> Option<Element> {
    match name {
        "button-demo" => {
            Some(
                rsx! {
                    Button { "Button" }
                },
            )
        }
        "button-primary" => {
            Some(
                rsx! {
                    Button { variant: ButtonVariant::Default, "Primary" }
                },
            )
        }
        "button-secondary" => {
            Some(
                rsx! {
                    Button { variant: ButtonVariant::Secondary, "Secondary" }
                },
            )
        }
        "button-destructive" => {
            Some(
                rsx! {
                    Button { variant: ButtonVariant::Destructive, "Destructive" }
                },
            )
        }
        "button-outline" => {
            Some(
                rsx! {
                    Button { variant: ButtonVariant::Outline, "Outline" }
                },
            )
        }
        "button-ghost" => {
            Some(
                rsx! {
                    Button { variant: ButtonVariant::Ghost, "Ghost" }
                },
            )
        }
        "button-link" => {
            Some(
                rsx! {
                    Button { variant: ButtonVariant::Link, "Link" }
                },
            )
        }
        "badge-demo" => {
            Some(
                rsx! {
                    Badge { "Badge" }
                },
            )
        }
        "badge-secondary" => {
            Some(
                rsx! {
                    Badge { variant: BadgeVariant::Secondary, "Secondary" }
                },
            )
        }
        "badge-destructive" => {
            Some(
                rsx! {
                    Badge { variant: BadgeVariant::Destructive, "Destructive" }
                },
            )
        }
        "badge-outline" => {
            Some(
                rsx! {
                    Badge { variant: BadgeVariant::Outline, "Outline" }
                },
            )
        }
        "card-demo" => {
            Some(
                rsx! {
                    Card { class: "w-[350px]",
                        CardHeader {
                            CardTitle { "Card Title" }
                            CardDescription { "Card description goes here." }
                        }
                        CardContent {
                            p { "Card content goes here." }
                        }
                        CardFooter {
                            Button { "Action" }
                        }
                    }
                },
            )
        }
        "avatar-demo" => {
            Some(
                rsx! {
                    Avatar {
                        AvatarImage {
                            src: "https://github.com/shadcn.png",
                            alt: "shadcn",
                        }
                        AvatarFallback { "CN" }
                    }
                },
            )
        }
        "avatar-fallback" => {
            Some(
                rsx! {
                    Avatar {
                        AvatarFallback { "JD" }
                    }
                },
            )
        }
        "input-demo" => {
            Some(
                rsx! {
                    Input { placeholder: "Email" }
                },
            )
        }
        "input-disabled" => {
            Some(
                rsx! {
                    Input { placeholder: "Disabled", disabled: true }
                },
            )
        }
        "label-demo" => {
            Some(
                rsx! {
                    Label { "Label" }
                },
            )
        }
        "progress-demo" => {
            Some(
                rsx! {
                    Progress { value: 60.0, class: "w-[60%]" }
                },
            )
        }
        "progress-indeterminate" => {
            Some(
                rsx! {
                    Progress { class: "w-[60%]" }
                },
            )
        }
        "separator-demo" => {
            Some(
                rsx! {
                    div {
                        div { class: "space-y-1",
                            h4 { class: "text-sm font-medium leading-none", "Radix Primitives" }
                            p { class: "text-sm text-muted-foreground",
                                "An open-source UI component library."
                            }
                        }
                        Separator {
                            class: "my-4",
                            orientation: separator::SeparatorOrientation::Horizontal,
                        }
                        div { class: "flex h-5 items-center space-x-4 text-sm",
                            div { "Blog" }
                            Separator { orientation: separator::SeparatorOrientation::Vertical }
                            div { "Docs" }
                            Separator { orientation: separator::SeparatorOrientation::Vertical }
                            div { "Source" }
                        }
                    }
                },
            )
        }
        "separator-vertical" => {
            Some(
                rsx! {
                    div { class: "flex h-5 items-center space-x-4 text-sm",
                        div { "Blog" }
                        Separator { orientation: separator::SeparatorOrientation::Vertical }
                        div { "Docs" }
                    }
                },
            )
        }
        "skeleton-demo" => {
            Some(
                rsx! {
                    div { class: "flex items-center space-x-4",
                        Skeleton { class: "h-12 w-12 rounded-full" }
                        div { class: "space-y-2",
                            Skeleton { class: "h-4 w-[250px]" }
                            Skeleton { class: "h-4 w-[200px]" }
                        }
                    }
                },
            )
        }
        "spinner-demo" => {
            Some(
                rsx! {
                    Spinner {}
                },
            )
        }
        "kbd-demo" => {
            Some(
                rsx! {
                    div { class: "flex items-center gap-1",
                        Kbd { "⌘" }
                        Kbd { "K" }
                    }
                },
            )
        }
        "kbd-group" => {
            Some(
                rsx! {
                    p { class: "text-muted-foreground text-sm",
                        "Use"
                        KbdGroup {
                            Kbd { "Ctrl + B" }
                            Kbd { "Ctrl + K" }
                        }
                        "to open the command palette"
                    }
                },
            )
        }
        "kbd-button" => {
            Some(
                rsx! {
                    div { class: "flex items-center gap-2",
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            class: "gap-2",
                            "Accept"
                            Kbd { "⏎" }
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            class: "gap-2",
                            "Cancel"
                            Kbd { "Esc" }
                        }
                    }
                },
            )
        }
        "item-demo" => {
            Some(
                rsx! {
                    div { class: "flex w-full max-w-md flex-col gap-6",
                        ItemGroup {
                            Item {
                                ItemMedia {
                                    Avatar {
                                        AvatarImage { src: "https://github.com/shadcn.png" }
                                        AvatarFallback { "S" }
                                    }
                                }
                                ItemContent {
                                    ItemTitle { "Item Title" }
                                    ItemDescription { "Item description goes here." }
                                }
                                ItemActions {
                                    Button {
                                        class: "rounded-full",
                                        size: ButtonSize::Icon,
                                        variant: ButtonVariant::Ghost,
                                        Plus {}
                                    }
                                }
                            }
                            ItemSeparator {}
                            Item {
                                ItemMedia {
                                    Avatar {
                                        AvatarImage { src: "https://github.com/shadcn.png" }
                                        AvatarFallback { "S" }
                                    }
                                }
                                ItemContent {
                                    ItemTitle { "Another Item" }
                                    ItemDescription { "Another description." }
                                }
                                ItemActions {
                                    Button {
                                        class: "rounded-full",
                                        size: ButtonSize::Icon,
                                        variant: ButtonVariant::Ghost,
                                        Plus {}
                                    }
                                }
                            }
                        }
                    }
                },
            )
        }
        "empty-demo" => {
            Some(
                rsx! {
                    Empty {
                        EmptyHeader {
                            EmptyTitle { "No results found" }
                            EmptyDescription { "Try adjusting your search or filters." }
                        }
                    }
                },
            )
        }
        "button-group-demo" => {
            Some(
                rsx! {
                    ButtonGroup::Root {
                        Button { variant: ButtonVariant::Outline, "Left" }
                        Button { variant: ButtonVariant::Outline, "Center" }
                        Button { variant: ButtonVariant::Outline, "Right" }
                    }
                },
            )
        }
        "checkbox-demo" => {
            Some(
                rsx! {
                    Checkbox { default_checked: CheckboxState::Checked }
                },
            )
        }
        "checkbox-with-label" => {
            Some(
                rsx! {
                    div { class: "flex items-center space-x-2",
                        Checkbox { id: "terms" }
                        Label { "for": "terms", "Accept terms and conditions" }
                    }
                },
            )
        }
        "checkbox-disabled" => {
            Some(
                rsx! {
                    div { class: "flex items-center space-x-2",
                        Checkbox { id: "disabled", disabled: true, default_checked: CheckboxState::Checked }
                        Label { "for": "disabled", "Disabled" }
                    }
                },
            )
        }
        "textarea-demo" => {
            Some(
                rsx! {
                    Textarea { placeholder: "Type your message here." }
                },
            )
        }
        "textarea-disabled" => {
            Some(
                rsx! {
                    Textarea { placeholder: "Type your message here.", disabled: true }
                },
            )
        }
        "textarea-with-label" => {
            Some(
                rsx! {
                    div { class: "grid w-full gap-1.5",
                        Label { "for": "message", "Your message" }
                        Textarea { placeholder: "Type your message here.", id: "message" }
                    }
                },
            )
        }
        "textarea-with-button" => {
            Some(
                rsx! {
                    div { class: "grid w-full gap-2",
                        Textarea { placeholder: "Type your message here." }
                        Button { "Send message" }
                    }
                },
            )
        }
        "switch-demo" => {
            Some(
                rsx! {
                    Switch {}
                },
            )
        }
        "switch-disabled" => {
            Some(
                rsx! {
                    div { class: "flex items-center space-x-2",
                        Switch { id: "disabled", disabled: true }
                        Label { "for": "disabled", "Airplane Mode" }
                    }
                },
            )
        }
        "switch-with-label" => {
            Some(
                rsx! {
                    div { class: "flex items-center space-x-2",
                        Switch { id: "airplane-mode" }
                        Label { "for": "airplane-mode", "Airplane Mode" }
                    }
                },
            )
        }
        "field-demo" => {
            Some(
                rsx! {
                    Field { class: "max-w-sm",
                        FieldLabel { "for": "email", "Email" }
                        Input { id: "email", placeholder: "Enter your email" }
                        FieldDescription { "We'll never share your email with anyone." }
                    }
                },
            )
        }
        "field-textarea" => {
            Some(
                rsx! {
                    Field { class: "max-w-sm",
                        FieldLabel { "for": "bio", "Bio" }
                        Textarea { id: "bio", placeholder: "Tell us about yourself" }
                        FieldDescription { "You can use markdown for formatting." }
                    }
                },
            )
        }
        "field-set-demo" => {
            Some(
                rsx! {
                    FieldSet { class: "max-w-md",
                        FieldLegend { "Address" }
                        Field {
                            FieldLabel { "for": "street", "Street" }
                            Input { id: "street", placeholder: "123 Main St" }
                        }
                        Field {
                            FieldLabel { "for": "city", "City" }
                            Input { id: "city", placeholder: "New York" }
                        }
                    }
                },
            )
        }
        "field-checkbox" => {
            Some(
                rsx! {
                    Field { orientation: FieldOrientation::Horizontal,
                        Checkbox { id: "terms" }
                        FieldContent {
                            FieldLabel { "for": "terms", "Accept terms and conditions" }
                            FieldDescription { "You agree to our Terms of Service and Privacy Policy." }
                        }
                    }
                },
            )
        }
        "field-switch" => {
            Some(
                rsx! {
                    Field { orientation: FieldOrientation::Horizontal, class: "max-w-sm",
                        Switch { id: "mfa" }
                        FieldContent {
                            FieldLabel { "for": "mfa", "Multi-factor Authentication" }
                            FieldDescription { "Add an extra layer of security to your account." }
                        }
                    }
                },
            )
        }
        "toggle-demo" => {
            Some(
                rsx! {
                    Toggle { aria_label: "Toggle bold",
                        Bold {}
                    }
                },
            )
        }
        "toggle-outline" => {
            Some(
                rsx! {
                    Toggle { variant: ToggleVariant::Outline, aria_label: "Toggle italic",
                        Italic {}
                    }
                },
            )
        }
        "toggle-with-text" => {
            Some(
                rsx! {
                    Toggle { aria_label: "Toggle italic",
                        Italic { class: "me-2" }
                        "Italic"
                    }
                },
            )
        }
        "toggle-sm" => {
            Some(
                rsx! {
                    Toggle { size: ToggleSize::Sm, aria_label: "Toggle bold",
                        Bold {}
                    }
                },
            )
        }
        "toggle-lg" => {
            Some(
                rsx! {
                    Toggle { size: ToggleSize::Lg, aria_label: "Toggle bold",
                        Bold {}
                    }
                },
            )
        }
        "toggle-disabled" => {
            Some(
                rsx! {
                    Toggle { disabled: true, aria_label: "Toggle underline",
                        Underline {}
                    }
                },
            )
        }
        "aspect-ratio-demo" => {
            Some(
                rsx! {
                    div { class: "w-[450px]",
                        AspectRatio { ratio: 16.0 / 9.0, class: "bg-muted",
                            img {
                                src: "https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=800&dpr=2&q=80",
                                alt: "Photo by Drew Beamer",
                                class: "h-full w-full rounded-md object-cover"
                            }
                        }
                    }
                },
            )
        }
        "native-select-demo" => {
            Some(
                rsx! {
                    NativeSelect {
                        NativeSelectOption { value: "", "Select a fruit" }
                        NativeSelectOption { value: "apple", "Apple" }
                        NativeSelectOption { value: "banana", "Banana" }
                        NativeSelectOption { value: "blueberry", "Blueberry" }
                        NativeSelectOption { value: "grapes", disabled: true, "Grapes" }
                        NativeSelectOption { value: "pineapple", "Pineapple" }
                    }
                },
            )
        }
        "native-select-optgroup" => {
            Some(
                rsx! {
                    NativeSelect {
                        NativeSelectOption { value: "", "Select a food" }
                        NativeSelectOptGroup { label: "Fruits",
                            NativeSelectOption { value: "apple", "Apple" }
                            NativeSelectOption { value: "banana", "Banana" }
                            NativeSelectOption { value: "blueberry", "Blueberry" }
                        }
                        NativeSelectOptGroup { label: "Vegetables",
                            NativeSelectOption { value: "carrot", "Carrot" }
                            NativeSelectOption { value: "broccoli", "Broccoli" }
                            NativeSelectOption { value: "spinach", "Spinach" }
                        }
                    }
                },
            )
        }
        "native-select-disabled" => {
            Some(
                rsx! {
                    NativeSelect { disabled: true,
                        NativeSelectOption { value: "", "Select a fruit" }
                        NativeSelectOption { value: "apple", "Apple" }
                        NativeSelectOption { value: "banana", "Banana" }
                    }
                },
            )
        }
        "native-select-invalid" => {
            Some(
                rsx! {
                    NativeSelect { aria_invalid: "true",
                        NativeSelectOption { value: "", "Select a fruit" }
                        NativeSelectOption { value: "apple", "Apple" }
                        NativeSelectOption { value: "banana", "Banana" }
                    }
                },
            )
        }
        "input-group-demo" => {
            Some(
                rsx! {
                    div { class: "grid w-full max-w-sm gap-6",
                        InputGroup {
                            InputGroupInput { placeholder: "Search..." }
                            InputGroupAddon {
                                Search {}
                            }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                "12 results"
                            }
                        }
                        InputGroup {
                            InputGroupInput { placeholder: "example.com", class: "!ps-1" }
                            InputGroupAddon {
                                InputGroupText { "https://" }
                            }
                        }
                        InputGroup {
                            InputGroupTextarea { placeholder: "Ask, Search or Chat..." }
                            InputGroupAddon { align: InputGroupAddonAlign::BlockEnd,
                                InputGroupButton { variant: ButtonVariant::Outline, class: "rounded-full", size: InputGroupButtonSize::IconXs,
                                    Plus {}
                                }
                                InputGroupText { class: "ms-auto", "52% used" }
                                Separator { orientation: separator::SeparatorOrientation::Vertical, class: "!h-4" }
                                InputGroupButton { variant: ButtonVariant::Default, class: "rounded-full", size: InputGroupButtonSize::IconXs, disabled: true,
                                    ArrowUp {}
                                }
                            }
                        }
                        InputGroup {
                            InputGroupInput { placeholder: "@shadcn" }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                div { class: "bg-primary text-primary-foreground flex size-4 items-center justify-center rounded-full",
                                    Check { class: "size-3" }
                                }
                            }
                        }
                    }
                },
            )
        }
        "input-group-icon" => {
            Some(
                rsx! {
                    div { class: "grid w-full max-w-sm gap-6",
                        InputGroup {
                            InputGroupInput { placeholder: "Search..." }
                            InputGroupAddon {
                                Search {}
                            }
                        }
                        InputGroup {
                            InputGroupInput { r#type: "email", placeholder: "Enter your email" }
                            InputGroupAddon {
                                Mail {}
                            }
                        }
                        InputGroup {
                            InputGroupInput { placeholder: "Card number" }
                            InputGroupAddon {
                                CreditCard {}
                            }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                Check {}
                            }
                        }
                        InputGroup {
                            InputGroupInput { placeholder: "Card number" }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                Star {}
                                Info {}
                            }
                        }
                    }
                },
            )
        }
        "input-group-text" => {
            Some(
                rsx! {
                    div { class: "grid w-full max-w-sm gap-6",
                        InputGroup {
                            InputGroupAddon {
                                InputGroupText { "$" }
                            }
                            InputGroupInput { placeholder: "0.00" }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                InputGroupText { "USD" }
                            }
                        }
                        InputGroup {
                            InputGroupAddon {
                                InputGroupText { "https://" }
                            }
                            InputGroupInput { placeholder: "example.com", class: "!ps-0.5" }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                InputGroupText { ".com" }
                            }
                        }
                        InputGroup {
                            InputGroupInput { placeholder: "Enter your username" }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                InputGroupText { "@company.com" }
                            }
                        }
                        InputGroup {
                            InputGroupTextarea { placeholder: "Enter your message" }
                            InputGroupAddon { align: InputGroupAddonAlign::BlockEnd,
                                InputGroupText { class: "text-muted-foreground text-xs", "120 characters left" }
                            }
                        }
                    }
                },
            )
        }
        "input-group-button" => {
            Some(
                rsx! {
                    div { class: "grid w-full max-w-sm gap-6",
                        InputGroup {
                            InputGroupInput { readonly: true, placeholder: "https://x.com/shadcn" }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                InputGroupButton { size: InputGroupButtonSize::IconXs,
                                    Copy {}
                                }
                            }
                        }
                        InputGroup { class: "[--radius:9999px]",
                            InputGroupAddon { class: "text-muted-foreground ps-1.5",
                                InputGroupText { "https://" }
                            }
                            InputGroupInput {}
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                InputGroupButton { size: InputGroupButtonSize::IconXs,
                                    Star {}
                                }
                            }
                        }
                        InputGroup {
                            InputGroupInput { placeholder: "Type to search..." }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                InputGroupButton { variant: ButtonVariant::Secondary,
                                    "Search"
                                }
                            }
                        }
                    }
                },
            )
        }
        "input-group-textarea" => {
            Some(
                rsx! {
                    InputGroup { class: "w-full max-w-md",
                        InputGroupAddon { align: InputGroupAddonAlign::BlockStart, class: "border-b",
                            InputGroupText { class: "font-mono font-medium",
                                FileCode { class: "size-4" }
                                "script.js"
                            }
                            InputGroupButton { class: "ms-auto", size: InputGroupButtonSize::IconXs,
                                RefreshCw {}
                            }
                            InputGroupButton { variant: ButtonVariant::Ghost, size: InputGroupButtonSize::IconXs,
                                Copy {}
                            }
                        }
                        InputGroupTextarea { placeholder: "console.log('Hello, world!');", class: "min-h-[200px]" }
                        InputGroupAddon { align: InputGroupAddonAlign::BlockEnd, class: "border-t",
                            InputGroupText { "Line 1, Column 1" }
                            InputGroupButton { size: InputGroupButtonSize::Sm, class: "ms-auto", variant: ButtonVariant::Default,
                                "Run"
                                CornerDownLeft {}
                            }
                        }
                    }
                },
            )
        }
        "input-group-spinner" => {
            Some(
                rsx! {
                    div { class: "grid w-full max-w-sm gap-4",
                        InputGroup { "data-disabled": "true",
                            InputGroupInput { placeholder: "Searching...", disabled: true }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                Spinner {}
                            }
                        }
                        InputGroup { "data-disabled": "true",
                            InputGroupInput { placeholder: "Processing...", disabled: true }
                            InputGroupAddon {
                                Spinner {}
                            }
                        }
                        InputGroup { "data-disabled": "true",
                            InputGroupInput { placeholder: "Saving changes...", disabled: true }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                InputGroupText { "Saving..." }
                                Spinner {}
                            }
                        }
                        InputGroup { "data-disabled": "true",
                            InputGroupInput { placeholder: "Refreshing data...", disabled: true }
                            InputGroupAddon {
                                Loader { class: "animate-spin" }
                            }
                            InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                InputGroupText { class: "text-muted-foreground", "Please wait..." }
                            }
                        }
                    }
                },
            )
        }
        "input-group-label" => {
            Some(
                rsx! {
                    div { class: "grid w-full max-w-sm gap-4",
                        InputGroup {
                            InputGroupInput { id: "email", placeholder: "shadcn" }
                            InputGroupAddon {
                                Label { "for": "email", "@" }
                            }
                        }
                        InputGroup {
                            InputGroupInput { id: "email-2", placeholder: "shadcn@vercel.com" }
                            InputGroupAddon { align: InputGroupAddonAlign::BlockStart,
                                Label { "for": "email-2", class: "text-foreground", "Email" }
                            }
                        }
                    }
                },
            )
        }
        "input-group-button-group" => {
            Some(
                rsx! {
                    div { class: "grid w-full max-w-sm gap-6",
                        ButtonGroup::Root {
                            ButtonGroup::Text {
                                Label { "for": "url", "https://" }
                            }
                            InputGroup {
                                InputGroupInput { id: "url" }
                                InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                                    Link2 {}
                                }
                            }
                            ButtonGroup::Text { ".com" }
                        }
                    }
                },
            )
        }
        "input-group-custom-input" => {
            Some(
                rsx! {
                    div { class: "grid w-full max-w-sm gap-6",
                        InputGroup {
                            textarea {
                                "data-slot": "input-group-control",
                                class: "field-sizing-content flex min-h-16 w-full resize-none rounded-md bg-transparent px-3 py-2.5 text-base outline-none transition-[color,box-shadow] md:text-sm",
                                placeholder: "Autoresize textarea...",
                            }
                            InputGroupAddon { align: InputGroupAddonAlign::BlockEnd,
                                InputGroupButton { class: "ms-auto", size: InputGroupButtonSize::Sm, variant: ButtonVariant::Default,
                                    "Submit"
                                }
                            }
                        }
                    }
                },
            )
        }
        _ => None,
    }
}
