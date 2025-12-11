use dioxus::prelude::*;

use super::{PortalContext, PortalId};

/// A portal component that renders its children at the app root level,
/// outside of the current component hierarchy.
///
///
/// # Example
///
/// ```rust
/// use ui::Portal;
///
/// rsx! {
///     Portal {
///         div { class: "fixed inset-0 z-50 bg-black/50",
///             // This will render at the root level
///         }
///     }
/// }
/// ```
#[component]
pub fn Portal(children: Element) -> Element {
    let ctx = use_context::<PortalContext>();
    let mut portal_id: Signal<Option<PortalId>> = use_signal(|| None);

    // Register portal on mount, update on content change, cleanup on unmount
    use_effect(move || {
        let content = children.clone();
        if let Some(id) = portal_id() {
            ctx.update(id, content);
        } else {
            let id = ctx.register(content);
            portal_id.set(Some(id));
        }
    });

    use_drop(move || {
        if let Some(id) = portal_id() {
            ctx.unregister(id);
        }
    });

    // All the content is located within PortalOutlet
    rsx! {}
}
