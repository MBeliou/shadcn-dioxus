use dioxus::prelude::*;
use super::{calendar_header, calendar_grid};

#[derive(Clone, PartialEq)]
pub struct CalendarContext {
    pub year: Signal<i32>,
    pub month: Signal<u32>,
    pub selected_date: Signal<Option<(i32, u32, u32)>>,
    pub set_selected_date: Callback<Option<(i32, u32, u32)>>,
}

#[derive(Clone, PartialEq, Props)]
pub struct CalendarProps {
    #[props(default = 2024)]
    pub default_year: i32,
    #[props(default = 0)]
    pub default_month: u32,
    #[props(default)]
    pub default_date: Option<(i32, u32, u32)>,
    #[props(default)]
    pub on_change: Option<Callback<Option<(i32, u32, u32)>>>,
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    let year = use_signal(|| props.default_year);
    let month = use_signal(|| props.default_month);
    let mut selected_date = use_signal(|| props.default_date);

    let set_selected_date = use_callback(move |val: Option<(i32, u32, u32)>| {
        selected_date.set(val);
        if let Some(cb) = &props.on_change {
            cb.call(val);
        }
    });

    use_context_provider(|| CalendarContext { year, month, selected_date, set_selected_date });

    rsx! {
        div {
            "data-slot": "calendar",
            class: "p-3 {props.class}",
            calendar_header::CalendarHeader {}
            calendar_grid::CalendarGrid {}
        }
    }
}
