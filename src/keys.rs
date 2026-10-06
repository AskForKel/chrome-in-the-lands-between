//! The only keys the chrome listens to. The game's own bindings are never remapped.

/// How many frames must pass between two bench actions, so a held key counts once.
pub const DEBOUNCE_FRAMES: u64 = 14;

pub const BENCH: i32 = 0x46; // F
pub const CONFIRM: i32 = 0x0D; // Enter
pub const REMOVE: i32 = 0x2E; // Delete
pub const UP: i32 = 0x26;
pub const DOWN: i32 = 0x28;
pub const LEFT: i32 = 0x25;
pub const RIGHT: i32 = 0x27;
