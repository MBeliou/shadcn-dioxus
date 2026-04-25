use crate::calendar::CalendarContext;
use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CalendarDayProps {
    pub day: u32,
}

#[component]
pub fn CalendarDay(props: CalendarDayProps) -> Element {
    let ctx = use_context::<CalendarContext>();
    let selected_date = ctx.selected_date;
    let set_selected_date = ctx.set_selected_date;
    let year = ctx.year;
    let month = ctx.month;
    let day = props.day;

    let is_selected = use_memo(move || {
        selected_date() == Some((year(), month(), day))
    });

    let classes = cn(
        "inline-flex h-8 w-8 items-center justify-center rounded-md text-sm font-medium font-medium",
        if is_selected() { "bg-primary text-primary-foreground hover:bg-primary hover:text-primary-foreground" } else { "hover:bg-accent hover:text-accent-foreground" },
    );

    rsx! {
        button {
            r#type: "button",
            class: "{classes}",
            onclick: move |_| set_selected_date.call(Some((year(), month(), day))),
            "{day}"
        }
    }
}
