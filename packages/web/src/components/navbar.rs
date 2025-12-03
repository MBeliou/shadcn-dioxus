use crate::components::GithubIcon;
use dioxus::{core::IntoAttributeValue, prelude::*};
use ui::Button;
const ICON_SVG: Asset = asset!("/assets/shadcn-dioxus.svg");
#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        header { id: "navbar", class: "bg-background sticky top-0 z-50 w-full",
            div { class: "container-wrapper 3xl:fixed:px-0 px-5",
                div { class: "3xl:fixed:container h-(--header-height) **:data-[slot=separator]:h-4! flex items-center gap-2",
                    Button {
                        size: ui::ButtonSize::IconSm,
                        variant: ui::ButtonVariant::Ghost,
                        href: "/",
                        img { class: "size-5", src: ICON_SVG }
                    }
                    div { class: "grow", {children} }
                    Button {
                        variant: ui::ButtonVariant::Ghost,
                        size: ui::ButtonSize::Sm,
                        href: "https://github.com/MBeliou/shadcn-dioxus",
                        attributes: vec![
                            Attribute {
                                name: "target",
                                namespace: None,
                                volatile: false,
                                value: "blank".into_value(),
                            },
                        ],
                        GithubIcon {}
                    }
                }
            }
        }
    }
}
