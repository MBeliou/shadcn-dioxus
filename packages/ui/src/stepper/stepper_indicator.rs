use crate::cn;
use super::stepper::StepperContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct StepperIndicatorProps {
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn StepperIndicator(props: StepperIndicatorProps) -> Element {
    let ctx = use_context::<StepperContext>();
    let current_step = ctx.current_step;
    let total_steps = ctx.total_steps;
    let dots = (0..total_steps()).map(move |i| {
        let active = i == current_step();
        let completed = i < current_step();
        let cls = if active { "h-2.5 w-2.5 rounded-full bg-primary" } else if completed { "h-2.5 w-2.5 rounded-full bg-primary" } else { "h-2.5 w-2.5 rounded-full bg-muted-foreground/30" };
        rsx! { span { key: "{i}", class: "{cls}" } }
    });
    rsx! {
        div { "data-slot": "stepper-indicator",
            class: cn("flex items-center gap-1.5", &props.class),
            ..props.attributes,
            {dots.into_iter()}
        }
    }
}
