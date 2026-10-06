//! The only keys the chrome listens to. The game's own bindings are never remapped.

/// How many frames must pass between two bench actions, so a held key counts once.
pub const DEBOUNCE_FRAMES: u64 = 14;

pub const BENCH: u32 = 0x46; // F
pub const CONFIRM: u32 = 0x0D; // Enter
pub const REMOVE: u32 = 0x2E; // Delete
pub const UP: u32 = 0x26;
pub const DOWN: u32 = 0x28;
pub const LEFT: u32 = 0x25;
pub const RIGHT: u32 = 0x27;
