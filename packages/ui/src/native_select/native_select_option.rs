use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NativeSelectOptionProps {
    pub children: Element,

    #[props(extends = GlobalAttributes)]
    #[props(extends = option)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn NativeSelectOption(props: NativeSelectOptionProps) -> Element {
    rsx! {
        option {
            "data-slot": "native-select-option",
            ..props.attributes,
            {props.children}
        }
    }
}
