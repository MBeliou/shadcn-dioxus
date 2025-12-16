use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn InstallationView() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8",
            div { class: "flex flex-col gap-2",
                div { class: "flex items-start justify-between",
                    h1 { class: "scroll-m-20 text-4xl font-semibold tracking-tight sm:text-3xl xl:text-4xl",
                        "Installation"
                    }
                }
                p { class: "text-muted-foreground text-balance text-[1.05rem] sm:text-base",
                    "Install and configure shadcn for Dioxus."
                }
                
            }
            
        }
    }
}
