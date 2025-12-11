mod portal;
mod portal_provider;

pub use portal::*;
pub use portal_provider::*;

use dioxus::prelude::*;

/// Unique ID for a portal instance
pub type PortalId = usize;

/// A single portal entry
#[derive(Clone)]
pub struct PortalEntry {
    pub id: PortalId,
    pub content: Element,
}

/// Context for managing portals.
/// This allows components to render content at the app root level,
/// escaping any containing blocks in the component hierarchy.
#[derive(Clone, Copy)]
pub struct PortalContext {
    portals: Signal<Vec<PortalEntry>>,
    next_id: Signal<PortalId>,
}

impl PortalContext {
    /// Create a new portal context
    pub fn new() -> Self {
        Self {
            portals: Signal::new(vec![]),
            next_id: Signal::new(0),
        }
    }

    /// Register a portal and return its ID
    pub fn register(&self, content: Element) -> PortalId {
        let mut next_id = self.next_id;
        let mut portals = self.portals;
        let id = next_id();
        next_id.set(id + 1);
        portals.write().push(PortalEntry { id, content });
        id
    }

    /// Update an existing portal's content
    pub fn update(&self, id: PortalId, content: Element) {
        let mut portals = self.portals;
        let mut portals_guard = portals.write();
        if let Some(entry) = portals_guard.iter_mut().find(|e| e.id == id) {
            entry.content = content;
        }
    }

    /// Unregister a portal
    pub fn unregister(&self, id: PortalId) {
        let mut portals = self.portals;
        portals.write().retain(|e| e.id != id);
    }

    /// Get all portal contents for rendering
    pub fn get_portals(&self) -> Vec<PortalEntry> {
        let portals = self.portals;
        portals()
    }
}

impl Default for PortalContext {
    fn default() -> Self {
        Self::new()
    }
}
