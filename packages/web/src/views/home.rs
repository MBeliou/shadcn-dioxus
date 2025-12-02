use dioxus::prelude::*;
use lucide_dioxus::{BadgeCheck, ChevronRight};
use ui::{
    Button, Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle, Hero, Item,
    ItemActions, ItemContent, ItemDescription, ItemMedia, ItemTitle, RenderFn, Spinner,
};
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        div { class: "container mx-auto flex-1 pb-6",
            div { class: "container overflow-hidden",
                section { class: "theme-container",
                    div { class: "mx-auto grid gap-8 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 xl:gap-6 2xl:gap-8",
                        div { class: "flex flex-col gap-6 *:[div]:w-full *:[div]:max-w-full",
                            div {
                                class: "flex space-x-2",
                                Button {
                                    variant: ui::ButtonVariant::Ghost,
                                    "Examples"
                                }
                            }
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
                            Empty{
                                class: "border",
                                EmptyHeader{
                                    EmptyMedia{
                                        variant: ui::EmptyMediaVariant::Icon,
                                        Spinner{

                                        }
                                    }
                                    EmptyTitle{
                                        "Processing your request"
                                    }
                                    EmptyDescription{
                                        "Please wait while we process your request. Do not refresh the page."
                                    }
                                }
                                EmptyContent{
                                    Button {
                                        "Cancel"
                                    }
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
