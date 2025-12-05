use dioxus::prelude::*;
use lucide_dioxus::Plus;
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
        _ => None,
    }
}
