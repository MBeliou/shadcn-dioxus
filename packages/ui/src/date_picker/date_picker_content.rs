use crate::date_picker::DatePickerContext;
use crate::popover::PopoverContext;
use dioxus::prelude::*;

#[component]
pub fn DatePickerContent() -> Element {
    let date_ctx = use_context::<DatePickerContext>();
    let popover_ctx = use_context::<PopoverContext>();

    let selected_date = date_ctx.selected_date;
    let set_selected_date = date_ctx.set_selected_date;
    let close_popover = popover_ctx.set_open;

    rsx! {
        crate::Calendar {
            default_date: selected_date(),
            on_change: move |date: Option<(i32, u32, u32)>| {
                set_selected_date.call(date);
                close_popover.call(false);
            },
        }
    }
}
