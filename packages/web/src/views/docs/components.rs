use dioxus::prelude::*;

#[component]
pub fn ComponentView() -> Element {
    rsx! {
        div { class: "flex flex-col  gap-2",
            div { class: "flex flex-col gap-2",
                div { class: "flex items-start justify-between",

                    h1 { class: "scroll-m-20 text-4xl font-semibold tracking-tight sm:text-3xl xl:text-4xl",
                        "Components"
                    }
                
                }
                p { class: "text-muted-foreground text-balance text-[1.05rem] sm:text-base",
                    "Here you can find all the components available in the library. We are working on adding more components."
                }
            
            }

            div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 md:gap-x-8 lg:gap-x-16 lg:gap-y-6 xl:gap-x-20",

                for i in 1..3 {
                    ui::Button { href: "/", variant: ui::ButtonVariant::Link, "{i}" }
                }
            }
        }

    }
}
