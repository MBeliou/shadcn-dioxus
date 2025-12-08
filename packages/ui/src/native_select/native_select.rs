use crate::cn;
use dioxus::prelude::*;
use lucide_dioxus::ChevronDown;
#[derive(Props, Clone, PartialEq)]
pub struct NativeSelectProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    #[props(extends = select)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn NativeSelect(props: NativeSelectProps) -> Element {
    rsx! {
        div {
            class: "group/native-select relative w-fit has-[select:disabled]:opacity-50",
            "data-slot": "native-select-wrapper",
            select {
                "data-slot": "native-select",
                class: cn(
                    "border-input placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 dark:hover:bg-input/50 shadow-xs h-9 w-full min-w-0 appearance-none rounded-md border bg-transparent px-3 py-2 pe-9 text-sm outline-none transition-[color,box-shadow] disabled:pointer-events-none disabled:cursor-not-allowed focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
                    &props.class,
                ),
                ..props.attributes,
                {props.children}
            }
            ChevronDown { class: "text-muted-foreground pointer-events-none absolute end-3.5 top-1/2 size-4 -translate-y-1/2 select-none opacity-50" }
        }
    }
}
