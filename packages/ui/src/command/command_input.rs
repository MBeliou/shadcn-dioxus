use crate::cn;
use crate::command::CommandContext;
use dioxus::prelude::*;
use lucide_dioxus::Search;

const INPUT_BASE: &str = "flex h-11 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct CommandInputProps {
    #[props(default = "Type a command or search...".to_string())]
    pub placeholder: String,
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CommandInput(props: CommandInputProps) -> Element {
    let ctx = use_context::<CommandContext>();
    let value = ctx.value;
    let set_value = ctx.set_value;

    rsx! {
        div {
            class: "flex items-center border-b px-3",
            Search { class: "mr-2 h-4 w-4 shrink-0 opacity-50" }
            input {
                r#type: "text",
                "data-slot": "command-input",
                class: cn(INPUT_BASE, &props.class),
                placeholder: "{props.placeholder}",
                disabled: props.disabled,
                value: "{value}",
                oninput: move |e| set_value.call(e.value()),
                ..props.attributes,
            }
        }
    }
}
