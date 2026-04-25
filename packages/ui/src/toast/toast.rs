use dioxus::prelude::*;
use super::toast_close::ToastClose;

#[derive(Clone, PartialEq)]
pub enum ToastVariant {
    Default,
    Destructive,
    Success,
}

impl ToastVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
            Self::Success => "success",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ToastData {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    pub action: Option<String>,
    pub duration: u64,
}

#[derive(Clone, PartialEq, Props)]
pub struct ToastProps {
    pub toast: Signal<ToastData>,
}

#[component]
pub fn Toast(props: ToastProps) -> Element {
    let toast = props.toast;
    let id = toast.with(|t| t.id);
    let title = toast.with(|t| t.title.clone());
    let description = toast.with(|t| t.description.clone());
    let variant = toast.with(|t| t.variant.clone());
    let action = toast.with(|t| t.action.clone());
    let variant_class = variant.as_str();

    let bg_class = match variant {
        ToastVariant::Default => "bg-background border",
        ToastVariant::Destructive => "bg-destructive text-white border-destructive",
        ToastVariant::Success => "bg-green-600 text-white border-green-600",
    };

    rsx! {
        div {
            key: "{id}",
            "data-slot": "toast",
            "data-variant": "{variant_class}",
            class: "{bg_class} group pointer-events-auto relative flex w-full items-center justify-between space-x-2 overflow-hidden rounded-md p-4 pr-6 shadow-lg transition-all animate-in slide-in-from-bottom-full data-[state=open]:sm:slide-in-from-bottom-full data-[state=closed]:fade-out-80 data-[swipe=cancel]:slide-out-to-right-full data-[swipe=end]:slide-out-to-right-full data-[swipe=move]:translate-x-[var(--radix-toast-swipe-move-x)] data-[swipe=move]:transition-none data-[swipe=move]:[transition-duration:200ms]",
            div {
                class: "grid gap-1",
                if let Some(desc) = &description {
                    div { class: "text-sm font-semibold", "{title}" }
                    div { class: "text-sm opacity-90", "{desc}" }
                } else {
                    div { class: "text-sm font-semibold", "{title}" }
                }
                if let Some(act) = &action {
                    div { class: "text-xs opacity-90", "{act}" }
                }
            }
            ToastClose { id: id }
        }
    }
}
