use crate::cn;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct TextareaProps {
    #[props(into, default)]
    pub class: String,
    #[props(into, default = "textarea".to_string())]
    pub data_slot: String,
    pub onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = textarea)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    rsx! {
        textarea {
            "data-slot": "{props.data_slot}",
            class: cn(
                "border-input placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 flex min-h-16 w-full rounded-md border bg-transparent px-3 py-2 text-base shadow-xs outline-none transition-[color,box-shadow] focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
                &props.class,
            ),
            onmounted: move |e| _ = props.onmounted.map(|cb| cb(e)),
            ..props.attributes,
        }
    }
}
