use dioxus::prelude::*;

use crate::Button;
const ICON_SVG: Asset = asset!("/assets/shadcn-dioxus.svg");

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        header {
            id: "navbar",
            class: "bg-background sticky top-0 z-50 w-full",

            div {
                class: "container-wrapper 3xl:fixed:px-0 px-5",
                div {
                    class: "3xl:fixed:container h-(--header-height) **:data-[slot=separator]:h-4! flex items-center gap-2",
                    Button {  
                        size: crate::ButtonSize::IconSm,
                        variant: crate::ButtonVariant::Ghost,
                        href: "/",
                        img {
                            src: ICON_SVG
                        }
                    },
                    {children}
                }
            }
        }
    }
}
