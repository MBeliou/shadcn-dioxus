use crate::components::ComponentPreview;
use crate::docs::{component_exists, loader::parse_doc, registry::get_component_doc};
use dioxus::prelude::*;
use dioxus_markdown::{CustomComponents, Markdown};
#[component]
pub fn ComponentDoc(name: String) -> Element {
    let doc = get_component_doc(&name).and_then(parse_doc);
    let exists = component_exists(&name);
    match (doc, exists) {
        (Some(parsed), _) => {
            let mut custom_components = CustomComponents::new();
            custom_components
                .register(
                    "ComponentPreview",
                    |props| {
                        let name = props
                            .get("name")
                            .map(|v| v.as_str().to_string())
                            .unwrap_or_default();
                        Ok(
                            rsx! {
                                ComponentPreview { name: name.to_string() }
                            },
                        )
                    },
                );
            rsx! {
                article { class: "prose dark:prose-invert max-w-none *>pre:bg-red-500",
                    header { class: "mb-8",
                        h1 { class: "scroll-m-20 text-4xl font-semibold tracking-tight sm:text-3xl xl:text-4xld",
                            "{parsed.frontmatter.title}"
                        }
                        p { class: "text-muted-foreground text-balance text-[1.05rem] sm:text-base",
                            "{parsed.frontmatter.description}"
                        }
                        if let Some(links) = &parsed.frontmatter.links {
                            div { class: "flex gap-4 mt-4",
                                if let Some(source) = &links.source {
                                    a {
                                        href: "{source}",
                                        class: "text-sm text-muted-foreground hover:underline",
                                        "Source"
                                    }
                                }
                                if let Some(doc) = &links.doc {
                                    a {
                                        href: "{doc}",
                                        class: "text-sm text-muted-foreground hover:underline",
                                        "Docs"
                                    }
                                }
                                if let Some(api) = &links.api {
                                    a {
                                        href: "{api}",
                                        class: "text-sm text-muted-foreground hover:underline",
                                        "API Reference"
                                    }
                                }
                            }
                        }
                    }
                    Markdown {
                        src: parsed.content.clone(),
                        components: custom_components,
                        theme: "base16-ocean.dark",
                    }
                }
            }
        }
        (None, true) => {
            rsx! {
                div { class: "py-12",
                    h1 { class: "scroll-m-20 capitalize text-4xl font-semibold tracking-tight sm:text-3xl xl:text-4xl mb-4",
                        "{name}"
                    }
                    div { class: "rounded-lg border border-border bg-muted/50 p-8 text-center",
                        p { class: "text-muted-foreground text-lg", "Documentation coming soon." }
                        p { class: "text-muted-foreground text-sm mt-2",
                            "This component is available but documentation is still being written."
                        }
                    }
                }
            }
        }
        (None, false) => {
            rsx! {
                div { class: "text-center py-12",
                    h1 { class: "text-2xl font-bold", "Component not found" }
                    p { class: "text-muted-foreground", "The component \"{name}\" doesn't exist." }
                }
            }
        }
    }
}
