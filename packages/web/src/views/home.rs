use dioxus::prelude::*;
use lucide_dioxus::{BadgeCheck, ChevronRight};
use ui::{
    Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle, Hero, Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemTitle, RenderFn, Separator, Spinner
};
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        div { class: "container mx-auto flex-1 pb-6",
            div { class: "container overflow-hidden",
                section { class: "theme-container",
                    div { class: "flex space-x-2",
                        Button { variant: ui::ButtonVariant::Ghost, "Examples" }
                    }
                    div { class: "mx-auto grid gap-8 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 xl:gap-6 2xl:gap-8",
                        div { class: "flex flex-col gap-6 *:[div]:w-full *:[div]:max-w-full",
                            Card { class: "border-border bg-background!",
                                CardHeader {
                                    CardTitle { "Payment Methods" }
                                    CardDescription { "All transactions are secure and encrypted" }
                                }
                                Separator {
                                    orientation: ui::separator::SeparatorOrientation::Horizontal,
                                }
                                CardHeader {
                                    CardTitle { "Billing Address" }
                                    CardDescription { "The billing address associated with your payment method" }
                                }
                                Separator {
                                    orientation: ui::separator::SeparatorOrientation::Horizontal,
                                }
                                CardContent {

                                    div {
                                        class: "mb-3 font-medium data-[variant=legend]:text-base data-[variant=label]:text-sm",
                                        "Comments"
                                    }
                                    textarea {  
                                        class: "border-input placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 field-sizing-content shadow-xs flex min-h-16 w-full rounded-md border bg-transparent px-3 py-2 text-base outline-none transition-[color,box-shadow] focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
                                        placeholder: "Add any additional comments"
                                    }
                                }
                                CardFooter {
                                    class: "space-x-2",
                                    Button {
                                        "Submit"
                                    }
                                    Button {
                                        variant: ui::ButtonVariant::Secondary,
                                        "Cancel"
                                    }
                                }
                            }
                        }
                        div { class: "flex flex-col gap-6 *:[div]:w-full *:[div]:max-w-full",
                            Item { variant: ui::ItemVariant::Outline,
                                ItemContent {
                                    ItemTitle { "Two-factor authentication" }
                                    ItemDescription { "Verify via email or phone number." }
                                }
                                ItemActions {
                                    Button {
                                        variant: ui::ButtonVariant::Default,
                                        size: ui::ButtonSize::Sm,
                                        "Enable"
                                    }
                                }
                            }
                            Empty { class: "border",
                                EmptyHeader {
                                    EmptyMedia { variant: ui::EmptyMediaVariant::Icon, Spinner {} }
                                    EmptyTitle { "Processing your request" }
                                    EmptyDescription {
                                        "Please wait while we process your request. Do not refresh the page."
                                    }
                                }
                                EmptyContent {
                                    Button { "Cancel" }
                                }
                            }
                            Item {
                                variant: ui::ItemVariant::Outline,
                                size: ui::ItemSize::Sm,
                                as_child: RenderFn::new(|p, children| rsx! {
                                    a { class: "{p.class}", href: "#", {children} }
                                }),
                                ItemMedia {
                                    BadgeCheck { class: "size-5" }
                                }
                                ItemContent {
                                    ItemTitle { "Your profile has been verified." }
                                }
                                ItemActions {
                                    ChevronRight { class: "size-4" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
