use dioxus::prelude::*;
use lucide_dioxus::{Bold, Italic, Plus, Underline};
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
        _ => None,
    }
}
