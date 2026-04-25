use crate::cn;
use dioxus::prelude::*;

const RAIL_BASE: &str = "absolute inset-y-0 left-0 z-50 hidden w-[3px] bg-sidebar-border transition-all hover:w-[6px] data-[state=expanded]:block data-[side=right]:left-auto data-[side=right]:right-0";

#[derive(Clone, PartialEq, Props)]
pub struct SidebarRailProps {
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SidebarRail(props: SidebarRailProps) -> Element {
    rsx! {
        button {
            r#type: "button",
            "data-slot": "sidebar-rail",
            "aria-label": "Toggle sidebar",
            class: cn(RAIL_BASE, &props.class),
            ..props.attributes,
        }
    }
}
