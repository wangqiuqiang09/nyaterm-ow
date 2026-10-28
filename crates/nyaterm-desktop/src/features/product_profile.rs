//! Fork-local product profile.
//!
//! The fork hides the left/right activity rails and every AI surface. Each
//! deviation from upstream is routed through the switches here so the touched
//! upstream files keep one-line hooks and merges stay small.

use crate::models::NavItem;

/// Draw the left and right activity rails.
pub(crate) const SHOW_ACTIVITY_BARS: bool = false;

/// Offer AI entry points: panel, settings tabs, shortcuts, and context menus.
pub(crate) const SHOW_AI: bool = nyaterm_core::product::AI_VISIBLE;

/// Whether a panel may be listed in menus or restored from a saved layout.
pub(crate) fn panel_offered(item: NavItem) -> bool {
    SHOW_AI || item != NavItem::AiAssistant
}
