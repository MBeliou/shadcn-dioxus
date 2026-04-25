use crate::cn;
use super::sidebar_group::SidebarGroupContext;
use dioxus::prelude::*;
use lucide_dioxus::ChevronRight;

#[derive(Clone, PartialEq, Props)]
pub struct SidebarGroupLabelProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(default = true)]
    pub show_caret: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SidebarGroupLabel(props: SidebarGroupLabelProps) -> Element {
    let ctx = use_context::<SidebarGroupContext>();
    let is_open = ctx.is_open;
    let set_is_open = ctx.set_is_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "sidebar-group-label",
            class: cn(
                "flex w-full items-center gap-1 rounded-md px-1 py-1.5 text-xs font-medium text-sidebar-foreground/70 outline-none ring-sidebar-ring transition-opacity hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2",
                &props.class,
            ),
            onclick: move |_| set_is_open.call(!is_open()),
            ..props.attributes,
            {props.children}
            if props.show_caret {
                ChevronRight {
                    class: "ml-auto h-3.5 w-3.5 transition-transform duration-200",
                    style: if is_open() { "transform: rotate(90deg)" } else { "" },
                }
            }
        }
    }
}
