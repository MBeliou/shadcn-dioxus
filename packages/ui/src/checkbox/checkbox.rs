use crate::cn;
use dioxus::prelude::*;
use lucide_dioxus::{Check, Minus};
use std::ops::Not;
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CheckboxState {
    Checked,
    Indeterminate,
    #[default]
    Unchecked,
}
impl CheckboxState {
    fn to_aria_checked(self) -> &'static str {
        match self {
            CheckboxState::Checked => "true",
            CheckboxState::Indeterminate => "mixed",
            CheckboxState::Unchecked => "false",
        }
    }
    fn to_data_state(self) -> &'static str {
        match self {
            CheckboxState::Checked => "checked",
            CheckboxState::Indeterminate => "indeterminate",
            CheckboxState::Unchecked => "unchecked",
        }
    }
}
impl From<CheckboxState> for bool {
    fn from(value: CheckboxState) -> Self {
        !matches!(value, CheckboxState::Unchecked)
    }
}
impl Not for CheckboxState {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Unchecked => Self::Checked,
            _ => Self::Unchecked,
        }
    }
}
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    #[props(default)]
    pub checked: Option<Signal<CheckboxState>>,
    #[props(default)]
    pub default_checked: CheckboxState,
    #[props(default = false)]
    pub disabled: bool,
    #[props(into, default)]
    pub class: String,
    /// Callback when state changes.
    #[props(default)]
    pub on_checked_change: Option<Callback<CheckboxState>>,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let mut internal_state = use_signal(|| props.default_checked);
    let get_checked = move || {
        props.checked.map(|s| s()).unwrap_or_else(|| internal_state())
    };
    let mut set_checked = move |new_state: CheckboxState| {
        if let Some(mut controlled) = props.checked {
            controlled.set(new_state);
        } else {
            internal_state.set(new_state);
        }
        if let Some(callback) = &props.on_checked_change {
            callback.call(new_state);
        }
    };
    let current_state = get_checked();
    rsx! {
        button {
            r#type: "button",
            role: "checkbox",
            "aria-checked": current_state.to_aria_checked(),
            "data-state": current_state.to_data_state(),
            "data-slot": "checkbox",
            "data-disabled": if props.disabled { "true" } else { "false" },
            disabled: props.disabled,
            class: cn(
                "border-input dark:bg-input/30 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground dark:data-[state=checked]:bg-primary data-[state=checked]:border-primary focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive shadow-xs peer flex size-4 shrink-0 items-center justify-center rounded-[4px] border outline-none transition-shadow focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50",
                &props.class,
            ),
            onclick: move |_| {
                if !props.disabled {
                    set_checked(!get_checked());
                }
            },
            onkeydown: move |e: KeyboardEvent| {
                if e.key() == Key::Enter {
                    e.prevent_default();
                }
            },
            ..props.attributes,
            div {
                "data-slot": "checkbox-indicator",
                class: "text-current transition-none",
                match current_state {
                    CheckboxState::Checked => rsx! {
                        Check { class: "size-3.5" }
                    },
                    CheckboxState::Indeterminate => rsx! {
                        Minus { class: "size-3.5" }
                    },
                    CheckboxState::Unchecked => rsx! {},
                }
            }
        }
    }
}
