use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CommandGroupProps {
    #[props(into, default)]
    pub heading: Option<String>,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CommandGroup(props: CommandGroupProps) -> Element {
    rsx! {
        div {
            "data-slot": "command-group",
            role: "group",
            class: "overflow-hidden p-1 text-foreground {props.class}",
            ..props.attributes,
            if let Some(heading) = &props.heading {
                div {
                    "data-slot": "command-group-heading",
                    class: "px-2 py-1.5 text-xs font-medium text-muted-foreground",
                    "{heading}"
                }
            }
            {props.children}
        }
    }
}
