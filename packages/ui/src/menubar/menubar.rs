use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MenubarContext {
    pub open_menu: Signal<Option<usize>>,
    pub set_open_menu: Callback<Option<usize>>,
}

#[derive(Clone, PartialEq, Props)]
pub struct MenubarProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    let mut open_menu = use_signal(|| None);
    let set_open_menu = use_callback(move |val: Option<usize>| open_menu.set(val));

    use_context_provider(|| MenubarContext { open_menu, set_open_menu });

    rsx! {
        div {
            "data-slot": "menubar",
            class: "flex h-9 items-center space-x-0.5 rounded-md border bg-background p-0.5 shadow-sm {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
