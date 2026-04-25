use crate::cn;
use crate::date_picker::DatePickerContext;
use crate::popover::PopoverContext;
use dioxus::prelude::*;
use lucide_dioxus::Calendar as CalendarIcon;

const MONTHS: &[&str] = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

fn format_date(year: i32, month: u32, day: u32) -> String {
    let month_name = MONTHS
        .get((month - 1) as usize)
        .copied()
        .unwrap_or("");
    format!("{} {}, {}", month_name, day, year)
}

const TRIGGER_BASE: &str = "border-input ring-offset-background shadow-xs inline-flex h-9 w-full rounded-md border bg-transparent px-3 py-1 text-left text-sm items-center outline-none transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 dark:bg-input/30 md:text-sm";

#[derive(Clone, PartialEq, Props)]
pub struct DatePickerTriggerProps {
    #[props(default = "Pick a date".to_string())]
    pub placeholder: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DatePickerTrigger(props: DatePickerTriggerProps) -> Element {
    let date_ctx = use_context::<DatePickerContext>();
    let popover_ctx = use_context::<PopoverContext>();

    let selected_date = date_ctx.selected_date;
    let open = popover_ctx.open;

    let date_text = use_memo(move || {
        selected_date().map(|(y, m, d)| format_date(y, m, d))
    });

    let display_text = date_text().unwrap_or_else(|| props.placeholder.clone());
    let is_placeholder = date_text().is_none();

    let classes = cn(TRIGGER_BASE, &props.class);

    rsx! {
        button {
            r#type: "button",
            "data-slot": "date-picker-trigger",
            "aria-expanded": open(),
            disabled: props.disabled,
            class: "{classes}",
            ..props.attributes,
            span {
                class: if is_placeholder { "text-muted-foreground" } else { "" },
                "{display_text}"
            }
            CalendarIcon { class: "ml-2 h-4 w-4 opacity-50" }
        }
    }
}
