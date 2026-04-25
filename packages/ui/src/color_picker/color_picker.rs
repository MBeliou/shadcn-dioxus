use dioxus::prelude::*;
use crate::cn;

#[derive(Clone, PartialEq, Props)]
pub struct ColorPickerProps {
    #[props(default = "0.0".to_string())]
    pub default_hue: String,
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    let default_hue: f64 = props.default_hue.parse().unwrap_or(0.0);
    let mut hue = use_signal(|| default_hue);
    let mut saturation = use_signal(|| 0.0f64);
    let mut lightness = use_signal(|| 50.0f64);

    let hsl_string = use_memo(move || {
        format!("hsl({} {}% {}%)", hue(), saturation(), lightness())
    });

    rsx! {
        div {
            "data-slot": "color-picker",
            class: cn("flex flex-col gap-4 p-4 border rounded-md w-full max-w-[280px]", &props.class),
            ..props.attributes,
            div { class: "space-y-2",
                div { class: "flex items-center gap-2",
                    label { "Hue ({hue})" }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "360",
                        value: "{hue}",
                        disabled: props.disabled,
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<f64>() { hue.set(v); }
                        },
                    }
                }
                div { class: "flex items-center gap-2",
                    label { "Saturation ({saturation})" }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "100",
                        value: "{saturation}",
                        disabled: props.disabled,
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<f64>() { saturation.set(v); }
                        },
                    }
                }
                div { class: "flex items-center gap-2",
                    label { "Lightness ({lightness})" }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "100",
                        value: "{lightness}",
                        disabled: props.disabled,
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<f64>() { lightness.set(v); }
                        },
                    }
                }
            }
            div {
                class: "h-16 w-full rounded-md border border-border",
                style: "background: {hsl_string}",
            }
        }
    }
}
