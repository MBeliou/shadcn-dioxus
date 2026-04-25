use dioxus::prelude::*;
use super::toast::Toast;
use super::toast_provider::get_toasts;

#[component]
pub fn ToastViewport() -> Element {
    let toasts = get_toasts();

    rsx! {
        div {
            "data-slot": "toast-viewport",
            class: "fixed bottom-0 right-0 z-[100] flex max-h-screen w-full flex-col-reverse p-4 sm:bottom-0 sm:right-0 sm:flex-col md:max-w-[420px]",
            tabindex: "-1",
            for toast_signal in toasts.iter() {
                Toast { key: "{toast_signal.id}", toast: Signal::new(toast_signal.clone()) }
            }
        }
    }
}
