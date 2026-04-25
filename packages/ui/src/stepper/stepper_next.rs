use crate::cn;
use super::stepper::StepperContext;
use dioxus::prelude::*;
use lucide_dioxus::ChevronRight;

const BUTTON_BASE: &str = "inline-flex h-9 w-9 items-center justify-center rounded-md border border-input bg-background shadow-sm transition-colors hover:bg-accent hover:text-accent-foreground disabled:pointer-events-none disabled:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct StepperNextProps {
    #[props(default = "Next".to_string())]
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
pub fn StepperNext(props: StepperNextProps) -> Element {
    let ctx = use_context::<StepperContext>();
    let current_step = ctx.current_step;
    let set_step = ctx.set_step;
    let total_steps = ctx.total_steps;
    let is_last = use_memo(move || {
        let t = total_steps();
        t == 0 || current_step() >= t - 1
    });
    let is_disabled = props.disabled || is_last();
    let classes = cn(BUTTON_BASE, &props.class);

    rsx! {
        button {
            r#type: "button",
            "data-slot": "stepper-next",
            disabled: is_disabled,
            class: "{classes}",
            onclick: move |e| {
                if !is_last() { set_step.call(current_step() + 1); }
                if let Some(cb) = &props.onclick { cb.call(e); }
            },
            ..props.attributes,
            if props.children.is_ok() { {props.children} } else { span { "{props.label}" } }
            ChevronRight { class: "ml-1 h-4 w-4" }
        }
    }
}
