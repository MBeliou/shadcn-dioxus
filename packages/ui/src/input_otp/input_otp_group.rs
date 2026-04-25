use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct InputOtpGroupProps {
    pub children: Element,
}

#[component]
pub fn InputOtpGroup(props: InputOtpGroupProps) -> Element {
    rsx! {
        div { class: "flex items-center gap-2", {props.children} }
    }
}
