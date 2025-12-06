use crate::cn;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    #[props(default)]
    pub checked: Option<Signal<bool>>,

    #[props(default = false)]
    pub default_checked: bool,

    #[props(default = false)]
    pub disabled: bool,

    #[props(into, default)]
    pub class: String,

    #[props(default)]
    pub on_checked_change: Option<Callback<bool>>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let mut internal_state = use_signal(|| props.default_checked);

    let get_checked = move || {
        props.checked.map(|s| s()).unwrap_or_else(|| internal_state())
    };

    let mut set_checked = move |new_state: bool| {
        if let Some(mut controlled) = props.checked {
            controlled.set(new_state);
        } else {
            internal_state.set(new_state);
        }
        if let Some(callback) = &props.on_checked_change {
            callback.call(new_state);
        }
    };

    let checked = get_checked();
    let data_state = if checked { "checked" } else { "unchecked" };

    rsx! {
        button {
            r#type: "button",
            role: "switch",
            "aria-checked": checked,
            "data-state": data_state,
            "data-slot": "switch",
            disabled: props.disabled,
            class: cn(
                "peer data-[state=checked]:bg-primary data-[state=unchecked]:bg-input focus-visible:border-ring focus-visible:ring-ring/50 dark:data-[state=unchecked]:bg-input/80 inline-flex h-[1.15rem] w-8 shrink-0 items-center rounded-full border border-transparent shadow-xs transition-all outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50",
                &props.class
            ),
            onclick: move |_| {
                if !props.disabled {
                    set_checked(!get_checked());
                }
            },
            ..props.attributes,

            span {
                "data-slot": "switch-thumb",
                "data-state": data_state,
                class: "bg-background dark:data-[state=unchecked]:bg-foreground dark:data-[state=checked]:bg-primary-foreground pointer-events-none block size-4 rounded-full ring-0 transition-transform data-[state=checked]:translate-x-[calc(100%-2px)] data-[state=unchecked]:translate-x-0",
            }
        }
    }
}
