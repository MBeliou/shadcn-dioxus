use crate::cn;
use dioxus::prelude::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct InputGroupCtx {
    /// The mounted input or textarea element that should receive focus
    pub control: Signal<Option<Rc<MountedData>>>,
}

const BASE_CLASSES: &str = "group/input-group border-input dark:bg-input/30 shadow-xs relative flex w-full items-center rounded-md border outline-none transition-[color,box-shadow] h-9 has-[>textarea]:h-auto has-[>[data-align=inline-start]]:[&>input]:ps-2 has-[>[data-align=inline-end]]:[&>input]:pe-2 has-[>[data-align=block-start]]:h-auto has-[>[data-align=block-start]]:flex-col has-[>[data-align=block-start]]:[&>input]:pb-3 has-[>[data-align=block-end]]:h-auto has-[>[data-align=block-end]]:flex-col has-[>[data-align=block-end]]:[&>input]:pt-3 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot=input-group-control]:focus-visible]:ring-[3px] has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40";

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn InputGroup(props: InputGroupProps) -> Element {
    let control = use_signal(|| None);
    use_context_provider(|| InputGroupCtx { control });

    rsx! {
        div {
            "data-slot": "input-group",
            role: "group",
            class: cn(BASE_CLASSES, &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
