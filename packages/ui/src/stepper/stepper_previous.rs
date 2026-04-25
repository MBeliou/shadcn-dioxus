use crate::cn;
use super::stepper::StepperContext;
use dioxus::prelude::*;
use lucide_dioxus::ChevronLeft;

const BUTTON_BASE: &str = "inline-flex h-9 w-9 items-center justify-center rounded-md border border-input bg-background shadow-sm transition-colors hover:bg-accent hover:text-accent-foreground disabled:pointer-events-none disabled:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct StepperPreviousProps {
    #[props(default = "Back".to_string())]
    pub label: String,
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn StepperPrevious(props: StepperPreviousProps) -> Element {
    let ctx = use_context::<StepperContext>();
    let current_step = ctx.current_step;
    let set_step = ctx.set_step;
    let is_first = use_memo(move || current_step() == 0);
    let is_disabled = props.disabled || is_first();
    let classes = cn(BUTTON_BASE, &props.class);

    rsx! {
        button {
            r#type: "button",
            "data-slot": "stepper-previous",
            disabled: is_disabled,
            class: "{classes}",
            onclick: move |e| {
                if !is_first() { set_step.call(current_step() - 1); }
                if let Some(cb) = &props.onclick { cb.call(e); }
            },
            ..props.attributes,
            ChevronLeft { class: "h-4 w-4" }
            if props.children.is_ok() { {props.children} } else { span { "{props.label}" } }
        }
    }
}
