use dioxus::prelude::*;

use super::PortalContext;

/// Provider component that enables portal functionality.
/// Wrap your app with this component to allow dialogs, tooltips, and other
/// overlay components to render at the root level.
///
/// # Example
///
/// ```rust
/// use ui::PortalProvider;
///
/// fn App() -> Element {
///     rsx! {
///         PortalProvider {
///             // Your app content here
///             Router {}
///         }
///     }
/// }
/// ```
#[component]
pub fn PortalProvider(children: Element) -> Element {
    let ctx = PortalContext::new();
    use_context_provider(|| ctx);

    rsx! {
        {children}
        // Portal outlet renders at the end, at root level. You'll need to handle positioning. 
        PortalOutlet {}
    }
}

/// Internal component that renders all portal content.
/// This is automatically included by PortalProvider.
#[component]
fn PortalOutlet() -> Element {
    let ctx = use_context::<PortalContext>();
    let portals = ctx.get_portals();

    rsx! {
        for entry in portals {
            {entry.content}
        }
    }
}
