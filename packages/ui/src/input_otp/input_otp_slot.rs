use crate::cn;
use crate::input_otp::InputOtpContext;
use dioxus::prelude::*;

const SLOT_BASE: &str = "flex h-10 w-10 items-center justify-center rounded-md border border-input bg-background text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct InputOtpSlotProps {
    #[props(default = 0)]
    pub index: usize,
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
}

#[component]
pub fn InputOtpSlot(props: InputOtpSlotProps) -> Element {
    let ctx = use_context::<InputOtpContext>();
    let value = ctx.value;
    let max_length = ctx.max_length;
    let set_value = ctx.set_value;

    let current_char = use_memo(move || {
        let val = value();
        if props.index < val.len() {
            val.chars().nth(props.index)
        } else {
            None
        }
    });

    let classes = cn(SLOT_BASE, &props.class);

    rsx! {
        input {
            r#type: "text",
            "data-slot": "input-otp-slot",
            maxlength: "1",
            value: current_char().map(|c| c.to_string()).unwrap_or_default(),
            disabled: props.disabled,
            class: "{classes}",
            oninput: move |e| {
                let mut new_val = value();
                let ch = e.value().chars().next();

                if let Some(c) = ch {
                    if props.index < new_val.len() {
                        let mut chars: Vec<char> = new_val.chars().collect();
                        chars[props.index] = c;
                        new_val = chars.into_iter().collect();
                    } else if props.index == new_val.len() {
                        new_val.push(c);
                    }
                } else {
                    // Handle backspace
                    if props.index < new_val.len() {
                        let mut chars: Vec<char> = new_val.chars().collect();
                        chars[props.index] = ' ';
                        new_val = chars.into_iter().collect();
                        new_val = new_val.trim_end().to_string();
                    }
                }

                if new_val.len() <= max_length {
                    set_value.call(new_val);
                }
            },
            // Auto-focus next logic would ideally go here with web-sys
        }
    }
}
