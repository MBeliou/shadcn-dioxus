use crate::cn;
use crate::dialog::DialogContext;
use dioxus::prelude::*;
use lucide_dioxus::X;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Side {
    Top,
    Bottom,
    Left,
    #[default]
    Right,
}

impl Side {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    pub fn class(&self) -> &'static str {
        match self {
            Self::Top => "data-[state=closed]:slide-out-to-top data-[state=open]:slide-in-from-top inset-x-0 top-0 h-auto border-b",
            Self::Bottom => "data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-bottom inset-x-0 bottom-0 h-auto border-t",
            Self::Left => "data-[state=closed]:slide-out-to-start data-[state=open]:slide-in-from-start inset-y-0 start-0 h-full w-3/4 border-e sm:max-w-sm",
            Self::Right => "data-[state=closed]:slide-out-to-end data-[state=open]:slide-in-from-end inset-y-0 end-0 h-full w-3/4 border-s sm:max-w-sm",
        }
    }
}

const CONTENT_BASE: &str = "bg-background data-[state=open]:animate-in data-[state=closed]:animate-out fixed z-50 flex flex-col gap-4 p-6 shadow-lg transition ease-in-out data-[state=closed]:duration-300 data-[state=open]:duration-500";

const CLOSE_BUTTON_BASE: &str = "ring-offset-background focus-visible:ring-ring absolute end-4 top-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-hidden disabled:pointer-events-none";

#[derive(Props, Clone, PartialEq)]
pub struct SheetContentProps {
    #[props(default)]
    pub side: Side,

    #[props(into, default)]
    pub class: String,

    #[props(default = true)]
    pub show_close_button: bool,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SheetContent(props: SheetContentProps) -> Element {
    let ctx = use_context::<DialogContext>();
    let set_open = ctx.set_open;

    let data_state = "open";

    rsx! {
        div {
            "data-slot": "sheet-content",
            "data-state": data_state,
            "data-side": props.side.as_str(),
            id: ctx.content_id.clone(),
            role: "dialog",
            "aria-modal": "true",
            "aria-labelledby": ctx.title_id.clone(),
            "aria-describedby": ctx.description_id.clone(),
            tabindex: "-1",
            class: cn(&cn(CONTENT_BASE, props.side.class()), &props.class),
            onclick: move |evt| {
                evt.stop_propagation();
            },
            onkeydown: move |evt: KeyboardEvent| {
                if evt.key() == Key::Escape {
                    set_open.call(false);
                }
            },
            onmounted: move |data: MountedEvent| {
                spawn(async move {
                    let _ = data.data().set_focus(true).await;
                });
            },
            ..props.attributes,
            {props.children}
            if props.show_close_button {
                button {
                    r#type: "button",
                    "data-slot": "sheet-close-button",
                    "data-state": data_state,
                    class: CLOSE_BUTTON_BASE,
                    onclick: move |_| {
                        set_open.call(false);
                    },
                    X { class: "size-4" }
                    span {
                        class: "sr-only",
                        "Close"
                    }
                }
            }
        }
    }
}
