use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct InputOtpContext {
    pub value: Signal<String>,
    pub max_length: usize,
    pub set_value: Callback<String>,
}

#[derive(Clone, PartialEq, Props)]
pub struct InputOtpProps {
    #[props(default = 6)]
    pub max_length: usize,
    #[props(default)]
    pub value: String,
    #[props(default)]
    pub on_change: Option<Callback<String>>,
    pub children: Element,
}

#[component]
pub fn InputOtp(props: InputOtpProps) -> Element {
    let mut internal_value = use_signal(|| props.value);

    let set_value = use_callback(move |val: String| {
        internal_value.set(val.clone());
        if let Some(cb) = &props.on_change {
            cb.call(val);
        }
    });

    use_context_provider(|| InputOtpContext {
        value: internal_value,
        max_length: props.max_length,
        set_value,
    });

    rsx! {
        div { class: "flex items-center gap-2", {props.children} }
    }
}
