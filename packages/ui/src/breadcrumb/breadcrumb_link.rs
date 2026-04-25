use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbLinkProps {
    #[props(into, default)]
    pub class: String,
    #[props(default)]
    pub href: Option<String>,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn BreadcrumbLink(props: BreadcrumbLinkProps) -> Element {
    rsx! {
        a {
            class: "transition-colors hover:text-foreground {props.class}",
            href: props.href.as_deref().unwrap_or("#"),
            ..props.attributes,
            {props.children}
        }
    }
}
