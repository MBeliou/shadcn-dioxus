use dioxus::prelude::*;

#[component]
pub fn InputOtpSeparator() -> Element {
    rsx! {
        div { "data-slot": "input-otp-separator", class: "flex items-center justify-center", "-" }
    }
}
