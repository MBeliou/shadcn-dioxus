use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static DIALOG_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn generate_dialog_id() -> String {
    let id = DIALOG_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("dialog-{}", id)
}

#[derive(Clone, PartialEq)]
pub struct DialogContext {
    pub open: Memo<bool>,

    pub set_open: Callback<bool>,

    pub content_id: String,

    pub title_id: String,

    pub description_id: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    #[props(default)]
    pub open: Option<Signal<bool>>,

    #[props(default = false)]
    pub default_open: bool,

    #[props(default)]
    pub on_open_change: Option<Callback<bool>>,

    pub children: Element,
}

/// The root Dialog component that manages state and provides context.
///
/// # Example
///
/// ```rust
/// use ui::{Dialog, DialogTrigger, DialogPortal, DialogOverlay, DialogContent, DialogTitle};
///
/// rsx! {
///     Dialog {
///         DialogTrigger { "Open Dialog" }
///         DialogPortal {
///             DialogOverlay {}
///             DialogContent {
///                 DialogTitle { "Hello" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Dialog(props: DialogProps) -> Element {
    // Internal state for uncontrolled mode
    let mut internal_open = use_signal(|| props.default_open);

    let on_open_change = props.on_open_change;

    // Respect controlled vs uncontrolled
    let get_open = use_memo(move || {
        props.open.map(|s| s()).unwrap_or_else(|| internal_open())
    });

    let set_open = use_callback(move |new_state: bool| {
        if let Some(mut controlled) = props.open {
            controlled.set(new_state);
        } else {
            internal_open.set(new_state);
        }
        if let Some(callback) = on_open_change {
            callback.call(new_state);
        }
    });

    // Unique per dialog ID. Required for accessibility
    let base_id = use_hook(generate_dialog_id);
    let content_id = use_hook(|| format!("{}-content", base_id));
    let title_id = use_hook(|| format!("{}-title", base_id));
    let description_id = use_hook(|| format!("{}-description", base_id));

    use_context_provider(|| DialogContext {
        open: get_open,
        set_open,
        content_id: content_id.clone(),
        title_id: title_id.clone(),
        description_id: description_id.clone(),
    });

    rsx! {
        {props.children}
    }
}
