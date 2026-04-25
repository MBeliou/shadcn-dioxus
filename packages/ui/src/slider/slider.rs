use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SliderProps {
    #[props(default = 0.0)]
    pub min: f64,
    #[props(default = 100.0)]
    pub max: f64,
    #[props(default = 1.0)]
    pub step: f64,
    #[props(default = 0.0)]
    pub value: f64,
    #[props(default)]
    pub on_change: Option<Callback<f64>>,
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Slider(props: SliderProps) -> Element {
    let mut internal_value = use_signal(|| props.value);

    let get_value = move || internal_value();
    let mut set_value = move |val: f64| {
        internal_value.set(val);
        if let Some(cb) = &props.on_change {
            cb.call(val);
        }
    };

    let percentage = (get_value() - props.min) / (props.max - props.min) * 100.0;

    let classes = cn(
        "relative flex w-full touch-none select-none items-center",
        &props.class
    );

    rsx! {
        div {
            "data-slot": "slider",
            class: "{classes}",
            ..props.attributes,
            input {
                r#type: "range",
                min: "{props.min}",
                max: "{props.max}",
                step: "{props.step}",
                value: "{get_value()}",
                disabled: props.disabled,
                class: "pointer-events-none absolute h-2 w-full cursor-foreground appearance-none bg-transparent opacity-0 [&::-webkit-slider-thumb]:pointer-events-auto",
                oninput: move |e| {
                    if let Ok(val) = e.value().parse::<f64>() {
                        set_value(val);
                    }
                },
            }
            div {
                class: "relative h-1.5 w-full grow overflow-hidden rounded-full bg-primary/20",
                div {
                    class: "h-full bg-primary",
                    style: "width: {percentage}%",
                }
            }
        }
    }
}
