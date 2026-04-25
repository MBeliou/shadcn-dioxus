use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SidebarGroupContext {
    pub is_open: Signal<bool>,
    pub set_is_open: Callback<bool>,
}

#[derive(Clone, PartialEq, Props)]
pub struct SidebarGroupProps {
    #[props(default = true)]
    pub default_open: bool,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SidebarGroup(props: SidebarGroupProps) -> Element {
    let mut is_open = use_signal(|| props.default_open);
    let set_is_open = use_callback(move |val: bool| is_open.set(val));

    use_context_provider(|| SidebarGroupContext { is_open, set_is_open });

    rsx! {
        div {
            "data-slot": "sidebar-group",
            "data-state": if is_open() { "open" } else { "closed" },
            class: "relative flex flex-col gap-1 px-2 py-1.5 {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
