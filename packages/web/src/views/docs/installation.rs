use crate::{components::PmBlock, docs::parse_doc, Route};
use dioxus::prelude::*;
use dioxus_markdown::{CustomComponents, Markdown};

#[component]
pub fn InstallationView() -> Element {
    let markdown = include_str!("../../content/get-started.md");

    let parsed = parse_doc(markdown).unwrap();

    let mut custom_components = CustomComponents::new();
    custom_components.register("PmBlock", |props| {
        
        let command = props.get("command").map(|v| v.as_str().to_string())
                            .unwrap_or_default();

        Ok(rsx! {
            PmBlock {  command:command.to_string()}
        })
    });

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
                Markdown {
                        src: parsed.content.clone(),
                        components: custom_components,
                        theme: "base16-ocean.dark",
                    }

            }

        }
    }
}
