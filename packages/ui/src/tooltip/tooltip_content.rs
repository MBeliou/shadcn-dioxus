use crate::cn;
use crate::tooltip::TooltipContext;
use dioxus::prelude::*;

const CONTENT_BASE: &str = "animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 overflow-hidden rounded-md bg-primary text-primary-foreground px-3 py-1.5 text-xs";

#[derive(Clone, PartialEq, Props)]
pub struct TooltipContentProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = "top".to_string())]
    pub side: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let ctx = use_context::<TooltipContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    if !open() {
        return rsx! {};
    }

    let side_class = format!("data-[side={}]", props.side);
    let position_class = match props.side.as_str() {
        "top" => "bottom-full left-1/2 -translate-x-1/2 mb-2",
        "bottom" => "top-full left-1/2 -translate-x-1/2 mt-2",
        "left" => "right-full top-1/2 -translate-y-1/2 mr-2",
        "right" => "left-full top-1/2 -translate-y-1/2 ml-2",
        _ => "bottom-full left-1/2 -translate-x-1/2 mb-2",
    };

    let base = cn(CONTENT_BASE, &side_class);
    let with_pos = cn(&base, position_class);
    let classes = cn(&with_pos, "absolute");
    let final_classes = cn(&classes, &props.class);

    rsx! {
        div {
            "data-slot": "tooltip-content",
            "data-state": "open",
            "data-side": "{props.side}",
            role: "tooltip",
            class: "{final_classes}",
            onmouseenter: move |_| set_open.call(true),
            onmouseleave: move |_| set_open.call(false),
            ..props.attributes,
            {props.children}
        }
    }
}
