use dioxus::prelude::*;


#[derive(Clone, Copy, PartialEq, Default)]
pub enum ProgressState {
    #[default]
    Indeterminate,
    Loading,
    Loaded,
}

impl ProgressState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Indeterminate => "indeterminate",
            Self::Loading => "loading",
            Self::Loaded => "loaded",
        }
    }
}

pub fn get_progress_state(value: Option<f64>, max: f64) -> ProgressState {
    match value {
        None => ProgressState::Indeterminate,
        Some(v) if v >= max => ProgressState::Loaded,
        Some(_) => ProgressState::Loading,
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ProgressPrimitiveProps {
    /// Current progress value (None = indeterminate)
    #[props(default)]
    pub value: Option<f64>,

    #[props(default = 100.0)]
    pub max: f64,

    #[props(default = 0.0)]
    pub min: f64,

    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ProgressPrimitive(props: ProgressPrimitiveProps) -> Element {
    let state = get_progress_state(props.value, props.max);

    rsx! {
        div {
            role: "progressbar",
            "aria-valuemin": props.min,
            "aria-valuemax": props.max,
            "aria-valuenow": props.value,
            "data-state": state.as_str(),
            "data-value": props.value,
            "data-max": props.max,
            "data-min": props.min,
            class: "{props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
