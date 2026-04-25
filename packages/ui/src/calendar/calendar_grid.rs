use crate::calendar::CalendarContext;
use super::calendar_day::CalendarDay;
use dioxus::prelude::*;

const WEEKDAYS: &[&str] = &["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
            if is_leap { 29 } else { 28 }
        }
        _ => 30,
    }
}

fn start_weekday(year: i32, month: u32) -> u32 {
    let _adjusted_month = if month < 3 { month + 12 } else { month };
    let adjusted_year = if month < 3 { year - 1 } else { year };
    let k = year % 100;
    let j = adjusted_year / 100;
    let day_of_week = (1 + (adjusted_year as i32 * 13 + 2) / 5 + (k as i32) / 4 + j / 4 + 5 * j + (adjusted_year as i32) / 400 * 5) % 7;
    ((day_of_week + 6) % 7) as u32
}

#[component]
pub fn CalendarGrid() -> Element {
    let ctx = use_context::<CalendarContext>();
    let year = ctx.year;
    let month = ctx.month;

    let days_weekday_memo = use_memo(move || {
        let y = year();
        let m = month();
        (days_in_month(y, m), start_weekday(y, m))
    });

    rsx! {
        div { class: "mt-2",
            div { class: "grid grid-cols-7 mb-1",
                for day in WEEKDAYS.iter() {
                    div {
                        class: "text-muted-foreground rounded-md w-8 h-8 text-xs font-medium flex items-center justify-center",
                        "{day}"
                    }
                }
            }
            div { class: "grid grid-cols-7",
                {
                    let (num_days, start) = days_weekday_memo();
                    rsx! {
                        for _ in 0..start {
                            div { class: "w-8 h-8" }
                        }
                        for day in 1..=num_days {
                            CalendarDay { key: "{day}", day }
                        }
                    }
                }
            }
        }
    }
}
