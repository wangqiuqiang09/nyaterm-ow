//! Fork-local product switches.
//!
//! This fork ships without AI surfaces. Code paths that read persisted user
//! state consult this constant so a profile written by an upstream build cannot
//! re-enable them.

/// Whether AI entry points are offered by this build.
pub const AI_VISIBLE: bool = false;
