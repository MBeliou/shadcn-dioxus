use dioxus::prelude::*;


#[derive(Props, Clone, PartialEq)]
pub struct NativeSelectOptionProps {
    pub children: Element,

    #[props(extends = GlobalAttributes)]
    #[props(extends = option)]
    pub attributes: Vec<Attribute>,
}


pub fn NativeSelectOptGroup(props: NativeSelectOptGroupProps) -> Element {
    rsx! {
        optgroup{
            "data-slot": "native-select-opt-group",
            ..props.attributes,
            {props.children}
        }
    }
}