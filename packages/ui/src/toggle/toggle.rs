use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ToggleSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl ToggleVariant {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => "bg-transparent",
            Self::Outline => {
                "border-input shadow-xs hover:bg-accent hover:text-accent-foreground border bg-transparent"
            }
        }
    }
}

impl ToggleSize {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => "h-9 min-w-9 px-2",
            Self::Sm => "h-8 min-w-8 px-1.5",
            Self::Lg => "h-10 min-w-10 px-2.5",
        }
    }
}

const BASE_CLASSES: &str = "hover:bg-muted hover:text-muted-foreground data-[state=on]:bg-accent data-[state=on]:text-accent-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium outline-none transition-[color,box-shadow] focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&_svg:not([class*='size-'])]:size-4 [&_svg]:pointer-events-none [&_svg]:shrink-0";

pub fn toggle_variants(variant: ToggleVariant, size: ToggleSize) -> String {
    format!("{} {} {}", BASE_CLASSES, variant.class(), size.class())
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    #[props(default)]
    pub pressed: Option<Signal<bool>>,

    #[props(default = false)]
    pub default_pressed: bool,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default)]
    pub variant: ToggleVariant,

    #[props(default)]
    pub size: ToggleSize,

    #[props(into, default)]
    pub class: String,

    #[props(default)]
    pub on_pressed_change: Option<Callback<bool>>,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let mut internal_state = use_signal(|| props.default_pressed);

    let get_pressed = move || {
        props
            .pressed
            .map(|s| s())
            .unwrap_or_else(|| internal_state())
    };

    let mut set_pressed = move |new_state: bool| {
        if let Some(mut controlled) = props.pressed {
            controlled.set(new_state);
        } else {
            internal_state.set(new_state);
        }
        if let Some(callback) = &props.on_pressed_change {
            callback.call(new_state);
        }
    };

    let pressed = get_pressed();
    let data_state = if pressed { "on" } else { "off" };
    let base_classes = toggle_variants(props.variant, props.size);

    rsx! {
        button {
            r#type: "button",
            "aria-pressed": pressed,
            "data-state": data_state,
            "data-slot": "toggle",
            disabled: props.disabled,
            class: cn(&base_classes, &props.class),
            onclick: move |_| {
                if !props.disabled {
                    set_pressed(!get_pressed());
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}
