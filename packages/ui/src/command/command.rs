use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CommandContext {
    pub value: Signal<String>,
    pub set_value: Callback<String>,
    pub selected_id: Signal<Option<String>>,
    pub set_selected_id: Callback<Option<String>>,
    pub visible_count: Signal<usize>,
}

#[derive(Clone, PartialEq, Props)]
pub struct CommandProps {
    #[props(default)]
    pub default_value: String,
    #[props(default)]
    pub on_select: Option<Callback<String>>,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Command(props: CommandProps) -> Element {
    let mut value = use_signal(|| props.default_value);
    let mut selected_id = use_signal(|| None);
    let visible_count = use_signal(|| 0usize);

    let set_value = use_callback(move |val: String| value.set(val));
    let set_selected_id = use_callback(move |id: Option<String>| {
        selected_id.set(id.clone());
        if let Some(cb) = &props.on_select {
            if let Some(id_val) = id {
                cb.call(id_val);
            }
        }
    });

    use_context_provider(|| CommandContext {
        value,
        set_value,
        selected_id,
        set_selected_id,
        visible_count,
    });

    rsx! {
        div {
            "data-slot": "command",
            role: "combobox",
            "aria-expanded": "true",
            class: "flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
