use super::stepper::StepperContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct StepperItemProps {
    #[props(default = 0)]
    pub step: usize,
    #[props(default)]
    pub completed: bool,
    #[props(default = false)]
    pub disabled: bool,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn StepperItem(props: StepperItemProps) -> Element {
    let ctx = use_context::<StepperContext>();
    let current_step = ctx.current_step;
    let set_step = ctx.set_step;
    let step = props.step;

    let is_active = use_memo(move || current_step() == step);
    let is_completed = use_memo(move || props.completed || current_step() > step);

    let active = is_active();
    let completed = is_completed();
    let disabled = props.disabled;

    let mut class_str = String::from("group flex items-center gap-3 rounded-md border p-3 transition-colors");
    if active { class_str.push_str(" border-primary bg-primary/5"); } else { class_str.push_str(" border-border"); }
    if completed && !active { class_str.push_str(" opacity-50"); }
    if disabled { class_str.push_str(" pointer-events-none opacity-50"); }
    if !props.class.is_empty() { class_str.push(' '); class_str.push_str(&props.class); }

    let mut circle_str = String::from("flex size-8 shrink-0 items-center justify-center rounded-full border-2 text-xs font-medium transition-colors");
    if completed && !active { circle_str.push_str(" border-primary bg-primary text-primary-foreground"); } else { circle_str.push_str(" border-muted-foreground/30"); }
    if active { circle_str.push_str(" border-primary text-primary"); }

    let step_label = if completed { "OK" } else { &step.to_string() };
    let state_str = if active { "active" } else if completed { "completed" } else { "inactive" };

    rsx! {
        div {
            "data-slot": "stepper-item",
            "data-state": state_str,
            class: "{class_str}",
            onclick: move |_| { if !disabled { set_step.call(step); } },
            ..props.attributes,
            div { class: "{circle_str}", "{step_label}" }
            div { class: "flex-1", {props.children} }
        }
    }
}
