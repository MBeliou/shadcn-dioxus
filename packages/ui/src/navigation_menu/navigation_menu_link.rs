use crate::cn;
use dioxus::prelude::*;

const LINK_BASE: &str = "group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus:outline-none disabled:pointer-events-none disabled:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct NavigationMenuLinkProps {
    #[props(into, default)]
    pub class: String,
    #[props(default)]
    pub href: Option<String>,
    #[props(default = false)]
    pub active: bool,
    #[props(default)]
    pub onselect: Option<Callback<()>>,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn NavigationMenuLink(props: NavigationMenuLinkProps) -> Element {
    let active_class = if props.active {
        "bg-accent text-accent-foreground"
    } else {
        ""
    };
    let base_classes = cn(LINK_BASE, active_class);
    let classes = cn(&base_classes, &props.class);

    if let Some(href) = props.href {
        rsx! {
            a {
                "data-slot": "navigation-menu-link",
                href: "{href}",
                class: "{classes}",
                onclick: move |_| {
                    if let Some(cb) = &props.onselect {
                        cb.call(());
                    }
                },
                ..props.attributes,
                {props.children}
            }
        }
    } else {
        rsx! {
            a {
                "data-slot": "navigation-menu-link",
                class: "{classes}",
                onclick: move |_| {
                    if let Some(cb) = &props.onselect {
                        cb.call(());
                    }
                },
                ..props.attributes,
                {props.children}
            }
        }
    }
}
