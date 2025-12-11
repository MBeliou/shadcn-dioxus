use crate::cn;
use crate::dialog::DialogContext;
use crate::portal::Portal;
use dioxus::prelude::*;

const PORTAL_BASE: &str = "fixed inset-0 z-50";

#[derive(Props, Clone, PartialEq)]
pub struct DialogPortalProps {
    #[props(into, default)]
    pub class: String,

    /// The portal content (typically DialogOverlay and DialogContent).
    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A container that renders its children at the app root level when the dialog is open.
/// Uses CSS fixed positioning to overlay the entire viewport.
///
/// The content is portaled to escape any CSS containment (transform, filter, etc.)
/// from ancestor elements, ensuring the dialog displays as a child of the PortalProvider.
///
/// Must be used within a `Dialog` component and the app must be wrapped with `PortalProvider`.
///
/// # Example
///
/// ```rust
/// use ui::{Dialog, DialogTrigger, DialogPortal, DialogOverlay, DialogContent};
///
/// rsx! {
///     Dialog {
///         DialogTrigger { "Open" }
///         DialogPortal {
///             DialogOverlay {}
///             DialogContent {
///                 // content
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn DialogPortal(props: DialogPortalProps) -> Element {
    // Capture the context BEFORE portaling
    let ctx = use_context::<DialogContext>();
    let open = ctx.open;

    // Lock body scroll when dialog is open.
    // NOTE: This uses JavaScript eval and only works on web platforms.
    // I think desktop and mobile would require a dedicated implementation
    use_effect(move || {
        if open() {
            document::eval("document.body.style.overflow = 'hidden'");
        } else {
            document::eval("document.body.style.overflow = ''");
        }
    });

    if !open() {
        return rsx! {};
    }

    let data_state = "open";

    rsx! {
        Portal {
            DialogContextProvider {
                context: ctx,
                div {
                    "data-slot": "dialog-portal",
                    "data-state": data_state,
                    class: cn(PORTAL_BASE, &props.class),
                    ..props.attributes,
                    {props.children}
                }
            }
        }
    }
}

/// Internal component to re-provide DialogContext in portaled content.
/// This is necessary because when content is rendered via Portal at the app root,
/// it loses access to the original Dialog component's context.
#[component]
fn DialogContextProvider(context: DialogContext, children: Element) -> Element {
    use_context_provider(|| context);
    rsx! { {children} }
}
