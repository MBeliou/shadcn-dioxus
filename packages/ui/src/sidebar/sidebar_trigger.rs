use crate::cn;
use super::sidebar_context::SidebarContext;
use dioxus::prelude::*;
use lucide_dioxus::PanelLeftClose;

#[derive(Clone, PartialEq, Props)]
pub struct SidebarTriggerProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub close: bool,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SidebarTrigger(props: SidebarTriggerProps) -> Element {
    let ctx = use_context::<SidebarContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    let classes = cn(
        "inline-flex h-7 w-7 items-center justify-center rounded-md text-sidebar-foreground outline-none ring-sidebar-ring transition-opacity hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2",
        &props.class,
    );

    rsx! {
        button {
            r#type: "button",
            "data-slot": "sidebar-trigger",
            "aria-label": if props.close { "Close sidebar" } else { "Toggle sidebar" },
            class: "{classes}",
            onclick: move |_| set_open.call(!open()),
            ..props.attributes,
            if props.children.is_ok() {
                {props.children}
            } else if props.close {
                PanelLeftClose { class: "h-4 w-4" }
            }
        }
    }
}
