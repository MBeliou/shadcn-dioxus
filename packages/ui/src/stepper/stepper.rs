use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct StepperContext {
    pub current_step: Signal<usize>,
    pub total_steps: Signal<usize>,
    pub set_step: Callback<usize>,
}

#[derive(Clone, PartialEq, Props)]
pub struct StepperProps {
    #[props(default = 0)]
    pub default_step: usize,
    /// Total number of steps (required for proper navigation)
    pub total_steps: usize,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Stepper(props: StepperProps) -> Element {
    let mut current_step = use_signal(|| props.default_step);
    let total_steps = use_signal(|| props.total_steps);
    let set_step = use_callback(move |step: usize| {
        if step < total_steps() { current_step.set(step); }
    });
    use_context_provider(|| StepperContext { current_step, total_steps, set_step });
    rsx! {
        div { "data-slot": "stepper", class: "flex flex-col gap-2", ..props.attributes, {props.children} }
    }
}
