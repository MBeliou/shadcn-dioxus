use std::sync::atomic::{AtomicUsize, Ordering};
use dioxus::prelude::*;
use super::toast::{ToastData, ToastVariant};

static TOAST_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Global signal holding all active toasts.
/// Safe because `GlobalSignal` does not require a component scope.
pub static TOASTS: GlobalSignal<Vec<ToastData>> = Signal::global(|| Vec::new());

pub fn get_toasts() -> Vec<ToastData> {
    TOASTS.cloned()
}

/// Add a toast notification
pub fn add_toast(title: String, description: Option<String>, variant: ToastVariant, duration: u64) {
    let id = TOAST_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let toast = ToastData {
        id,
        title,
        description,
        variant,
        action: None,
        duration,
    };
    TOASTS.with_mut(|v| v.push(toast));
}

/// Remove a toast by ID
pub fn remove_toast(id: usize) {
    TOASTS.with_mut(|v| v.retain(|t| t.id != id));
}

/// Clear all toasts
pub fn clear_toasts() {
    TOASTS.with_mut(|v| v.clear());
}

#[derive(Clone, PartialEq, Props)]
pub struct ToastProviderProps {
    pub children: Element,
}

#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    rsx! {
        {props.children}
        super::toast_viewport::ToastViewport {}
    }
}
