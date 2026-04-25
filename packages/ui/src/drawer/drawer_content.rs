use crate::cn;
use crate::drawer::DrawerContext;
use dioxus::prelude::*;
use lucide_dioxus::X;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum DrawerSide {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}

impl DrawerSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Right => "right",
            Self::Left => "left",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }

    pub fn classes(&self) -> &'static str {
        match self {
            Self::Right => "inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm data-[state=closed]:slide-out-to-right data-[state=open]:slide-in-from-right",
            Self::Left => "inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm data-[state=closed]:slide-out-to-left data-[state=open]:slide-in-from-left",
            Self::Top => "inset-x-0 top-0 h-auto w-full border-b data-[state=closed]:slide-out-to-top data-[state=open]:slide-in-from-top",
            Self::Bottom => "inset-x-0 bottom-0 h-auto w-full border-t data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-bottom",
        }
    }
}

const CONTENT_BASE: &str = "fixed z-50 gap-4 bg-background p-6 shadow-lg transition ease-in-out data-[state=closed]:duration-300 data-[state=open]:duration-500 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[state=closed]:slide-out-to-left data-[state=closed]:slide-out-to-right data-[state=closed]:slide-out-to-top data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-left data-[state=open]:slide-in-from-right data-[state=open]:slide-in-from-top data-[state=open]:slide-in-from-bottom";

const CLOSE_BASE: &str = "absolute right-4 top-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none";

#[derive(Clone, PartialEq, Props)]
pub struct DrawerContentProps {
    #[props(default)]
    pub side: DrawerSide,
    #[props(into, default)]
    pub class: String,
    #[props(default = true)]
    pub show_close_button: bool,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DrawerContent(props: DrawerContentProps) -> Element {
    let ctx = use_context::<DrawerContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    if !open() {
        return rsx! {};
    }

    let data_state = "open";
    let base = cn(CONTENT_BASE, props.side.classes());
    let classes = cn(&base, &props.class);

    rsx! {
        div {
            "data-slot": "drawer-content",
            "data-state": data_state,
            "data-side": props.side.as_str(),
            class: "{classes}",
            onclick: move |evt| evt.stop_propagation(),
            onkeydown: move |evt: KeyboardEvent| {
                if evt.key() == Key::Escape { set_open.call(false); }
            },
            ..props.attributes,
            {props.children}
            if props.show_close_button {
                button {
                    r#type: "button",
                    "data-slot": "drawer-close",
                    class: CLOSE_BASE,
                    onclick: move |_| set_open.call(false),
                    X { class: "h-4 w-4" }
                    span { class: "sr-only", "Close" }
                }
            }
        }
    }
}
