use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DatePickerContext {
    pub selected_date: Signal<Option<(i32, u32, u32)>>,
    pub set_selected_date: Callback<Option<(i32, u32, u32)>>,
}

#[derive(Clone, PartialEq, Props)]
pub struct DatePickerProps {
    #[props(default)]
    pub default_date: Option<(i32, u32, u32)>,
    #[props(default = "Pick a date".to_string())]
    pub placeholder: String,
    #[props(into, default)]
    pub class: String,
    #[props(default)]
    pub on_change: Option<Callback<Option<(i32, u32, u32)>>>,
    #[props(default = false)]
    pub disabled: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let mut selected_date = use_signal(|| props.default_date);

    let set_selected_date = use_callback(move |val: Option<(i32, u32, u32)>| {
        selected_date.set(val.clone());
        if let Some(cb) = &props.on_change {
            cb.call(val);
        }
    });

    use_context_provider(|| DatePickerContext {
        selected_date,
        set_selected_date,
    });

    rsx! {
        div {
            "data-slot": "date-picker",
            class: "{props.class}",
            ..props.attributes,
            crate::Popover {
                crate::PopoverTrigger {
                    crate::date_picker::DatePickerTrigger {
                        placeholder: props.placeholder.clone(),
                        disabled: props.disabled,
                    }
                }
                crate::PopoverContent {
                    class: "min-w-[280px] p-0",
                    crate::date_picker::DatePickerContent {}
                }
            }
        }
    }
}
