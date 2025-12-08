use crate::{Button, ButtonVariant};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum InputGroupButtonSize {
    #[default]
    Xs,
    Sm,
    IconXs,
    IconSm,
}

impl InputGroupButtonSize {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Xs => "h-6 gap-1 rounded-[calc(var(--radius)-5px)] px-2 has-[>svg]:px-2 [&>svg:not([class*='size-'])]:size-3.5",
            Self::Sm => "h-8 gap-1.5 rounded-md px-2.5 has-[>svg]:px-2.5",
            Self::IconXs => "size-6 rounded-[calc(var(--radius)-5px)] p-0 has-[>svg]:p-0",
            Self::IconSm => "size-8 p-0 has-[>svg]:p-0",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupButtonProps {
    #[props(default)]
    pub size: InputGroupButtonSize,

    #[props(default = ButtonVariant::Ghost)]
    pub variant: ButtonVariant,

    #[props(into, default)]
    pub class: String,

    #[props(default = false)]
    pub disabled: bool,

    pub children: Element,
}

#[component]
pub fn InputGroupButton(props: InputGroupButtonProps) -> Element {
    let size_class = props.size.class();
    let class = format!("shadow-none {} {}", size_class, props.class);

    rsx! {
        Button {
            variant: props.variant,
            disabled: props.disabled,
            class: class,
            children: {props.children}
        }
    }
}
