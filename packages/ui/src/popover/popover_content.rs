use crate::cn;
use crate::popover::PopoverContext;
use dioxus::prelude::*;

const CONTENT_BASE: &str = "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 origin-(--radix-popover-content-transform-origin) rounded-md border p-4 shadow-md outline-none";

#[derive(Clone, PartialEq, Props)]
pub struct PopoverContentProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = "bottom".to_string())]
    pub side: String,
    #[props(default = false)]
    pub force_mount: bool,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    let ctx = use_context::<PopoverContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    if !props.force_mount && !open() {
        return rsx! {};
    }

    let data_state = if open() { "open" } else { "closed" };
    let side_class = format!("data-[side={}]", props.side);
    let position_class = match props.side.as_str() {
        "top" => "bottom-full left-1/2 -translate-x-1/2 mb-2",
        "bottom" => "top-full left-1/2 -translate-x-1/2 mt-2",
        "left" => "right-full top-1/2 -translate-y-1/2 mr-2",
        "right" => "left-full top-1/2 -translate-y-1/2 ml-2",
        _ => "top-full left-1/2 -translate-x-1/2 mt-2",
    };

    let base = cn(CONTENT_BASE, &side_class);
    let with_pos = cn(&base, position_class);
    let classes = cn(&with_pos, "absolute");
    let final_classes = cn(&classes, &props.class);

    rsx! {
        div {
            class: "fixed inset-0 z-40",
            onclick: move |_| set_open.call(false),
        }
        div {
            "data-slot": "popover-content",
            "data-state": data_state,
            "data-side": "{props.side}",
            class: "{final_classes}",
            onclick: move |evt| evt.stop_propagation(),
            onkeydown: move |evt: KeyboardEvent| {
                if evt.key() == Key::Escape {
                    set_open.call(false);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}
