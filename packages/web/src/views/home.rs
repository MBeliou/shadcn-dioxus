use dioxus::prelude::*;
use ui::{Hero, Item, ItemContent, ItemTitle};
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        div { class: "container mx-auto flex-1 pb-6",
            div { class: "container overflow-hidden",
                section { class: "theme-container",
                    div { class: "mx-auto grid gap-8 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 xl:gap-6 2xl:gap-8",
                        div { class: "flex flex-col gap-6 *:[div]:w-full *:[div]:max-w-full",
                            "Demo coming soon",
                            Item {
                                variant: ui::ItemVariant::Outline,
                                size: ui::ItemSize::Sm,
                                // media
                                // content -> title
                                ItemContent{
                                    ItemTitle{
                                        "Your profile has been verified."
                                    }
                                }
                                // actions
                            }
                        }
                    }
                }
            }
        }
    }
}
