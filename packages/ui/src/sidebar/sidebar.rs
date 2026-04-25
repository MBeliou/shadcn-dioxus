use crate::cn;
use super::sidebar_context::SidebarContext;
use dioxus::prelude::*;

const SIDEBAR_BASE: &str = "relative flex w-64 flex-col border-r border-border bg-sidebar text-sidebar-foreground";

const SIDEBAR_FLOATING: &str = "relative flex w-64 flex-col border border-border bg-sidebar text-sidebar-foreground rounded-lg";

const SIDEBAR_INSET: &str = "relative flex w-64 flex-col border-r border-border bg-background text-foreground";

#[derive(Clone, PartialEq, Props)]
pub struct SidebarProps {
    #[props(default)]
    pub collapsible: bool,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    let ctx = use_context::<SidebarContext>();
    let open = ctx.open;
    let variant = ctx.variant;

    let base = match variant {
        super::sidebar_context::SidebarVariant::Sidebar => SIDEBAR_BASE,
        super::sidebar_context::SidebarVariant::Floating => SIDEBAR_FLOATING,
        super::sidebar_context::SidebarVariant::Inset => SIDEBAR_INSET,
    };

    let width_class = if props.collapsible && !open() {
        "w-16"
    } else {
        ""
    };

    let classes = cn(&format!("{} {}", base, width_class), &props.class);

    rsx! {
        aside {
            "data-slot": "sidebar",
            "data-variant": variant.as_str(),
            "data-collapsible": props.collapsible,
            "data-state": if open() { "expanded" } else { "collapsed" },
            class: "{classes}",
            ..props.attributes,
            {props.children}
        }
    }
}
