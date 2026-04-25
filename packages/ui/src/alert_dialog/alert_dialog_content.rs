use crate::cn;
use crate::alert_dialog::AlertDialogContext;
use dioxus::prelude::*;
use lucide_dioxus::X;

const CONTENT_BASE: &str = "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[state=closed]:slide-out-to-left-1/2 data-[state=closed]:slide-out-to-top-[48%] data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%] sm:rounded-lg";

const CLOSE_BASE: &str = "absolute right-4 top-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-accent data-[state=open]:text-muted-foreground";

#[derive(Clone, PartialEq, Props)]
pub struct AlertDialogContentProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = true)]
    pub show_close_button: bool,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    let ctx = use_context::<AlertDialogContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    if !open() {
        return rsx! {};
    }

    let data_state = if open() { "open" } else { "closed" };
    let classes = cn(CONTENT_BASE, &props.class);

    rsx! {
        div {
            class: "fixed inset-0 z-50 bg-black/80",
            onclick: move |_| set_open.call(false),
        }
        div {
            "data-slot": "alert-dialog-content",
            "data-state": data_state,
            role: "alertdialog",
            class: "{classes}",
            onclick: move |evt| evt.stop_propagation(),
            onkeydown: move |evt: KeyboardEvent| {
                if evt.key() == Key::Escape {
                    set_open.call(false);
                }
            },
            ..props.attributes,
            {props.children}
            if props.show_close_button {
                button {
                    r#type: "button",
                    "data-slot": "alert-dialog-close",
                    class: CLOSE_BASE,
                    onclick: move |_| set_open.call(false),
                    X { class: "h-4 w-4" }
                    span { class: "sr-only", "Close" }
                }
            }
        }
    }
}
