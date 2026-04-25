use crate::cn;
use dioxus::prelude::*;

const ITEM_BASE: &str = "relative flex cursor-default items-center gap-2 rounded-md px-2 py-1.5 text-sm outline-none ring-sidebar-ring transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2 data-[active=true]:bg-sidebar-accent data-[active=true]:text-sidebar-accent-foreground data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct SidebarMenuItemProps {
    #[props(default = false)]
    pub active: bool,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SidebarMenuItem(props: SidebarMenuItemProps) -> Element {
    let classes = cn(ITEM_BASE, &props.class);

    rsx! {
        button {
            r#type: "button",
            "data-slot": "sidebar-menu-item",
            role: "menuitem",
            "data-active": props.active,
            "data-disabled": props.disabled,
            disabled: props.disabled,
            class: "{classes}",
            onclick: move |e| {
                if let Some(cb) = &props.onclick {
                    cb.call(e);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}
