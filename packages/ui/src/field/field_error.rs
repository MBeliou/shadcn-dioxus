use crate::cn;
use dioxus::prelude::*;
#[derive(Clone, PartialEq)]
pub struct FieldErrorMessage {
    pub message: Option<String>,
}
#[derive(Props, Clone, PartialEq)]
pub struct FieldErrorProps {
    #[props(into, default)]
    pub class: String,
    #[props(default)]
    pub children: Element,
    #[props(default)]
    pub errors: Vec<FieldErrorMessage>,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn FieldError(props: FieldErrorProps) -> Element {
    let has_children = props.children.is_ok();
    let errors_with_messages: Vec<_> = props
        .errors
        .iter()
        .filter_map(|e| e.message.clone())
        .collect();
    let has_content = has_children || errors_with_messages.len() > 1
        || (errors_with_messages.len() == 1 && !errors_with_messages[0].is_empty());
    if !has_content {
        return rsx! {};
    }
    let is_multiple = errors_with_messages.len() > 1;
    let single_message = if errors_with_messages.len() == 1 {
        Some(errors_with_messages[0].clone())
    } else {
        None
    };
    rsx! {
        div {
            role: "alert",
            "data-slot": "field-error",
            class: cn("text-destructive text-sm font-normal", &props.class),
            ..props.attributes,
            if has_children {
                {props.children}
            } else if let Some(msg) = single_message {
                "{msg}"
            } else if is_multiple {
                ul { class: "ms-4 flex list-disc flex-col gap-1",
                    for (index , msg) in errors_with_messages.iter().enumerate() {
                        li { key: "{index}", "{msg}" }
                    }
                }
            }
        }
    }
}
