use crate::calendar::CalendarContext;
use dioxus::prelude::*;
use lucide_dioxus::{ChevronLeft, ChevronRight};

const MONTHS: &[&str] = &[
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
];

#[component]
pub fn CalendarHeader() -> Element {
    let ctx = use_context::<CalendarContext>();
    let mut year = ctx.year;
    let mut month = ctx.month;

    let prev_month = move |_| {
        let m = (month)();
        let y = (year)();
        if m == 0 {
            month.set(11);
            year.set(y - 1);
        } else {
            month.set(m - 1);
        }
    };

    let next_month = move |_| {
        let m = (month)();
        let y = (year)();
        if m == 11 {
            month.set(0);
            year.set(y + 1);
        } else {
            month.set(m + 1);
        }
    };

    let month_name = use_memo(move || MONTHS[(month)() as usize].to_string());

    rsx! {
        div {
            class: "flex items-center justify-between pt-1 relative",
            div { class: "space-y-1",
                h2 { class: "text-sm font-medium", "{month_name} {year}" }
            }
            div { class: "flex items-center space-x-1",
                button {
                    r#type: "button",
                    class: "inline-flex h-7 w-7 items-center justify-center rounded-md text-sm font-medium opacity-50 hover:opacity-100",
                    onclick: prev_month,
                    ChevronLeft { class: "h-4 w-4" }
                }
                button {
                    r#type: "button",
                    class: "inline-flex h-7 w-7 items-center justify-center rounded-md text-sm font-medium opacity-50 hover:opacity-100",
                    onclick: next_month,
                    ChevronRight { class: "h-4 w-4" }
                }
            }
        }
    }
}
