//! The bench: which chrome sits in which slot, which row the cursor is on, the charge meter.
//! Slice 1 only holds this state; slice 2 makes it act on the game.

use crate::keys;
use crate::log;
use crate::table::{self, CHROME, CHROME_COUNT, Fidelity};
use std::sync::{Mutex, OnceLock};

pub const MAX_TIERS: usize = 4;

#[derive(Clone, Copy)]
pub struct Fitted {
    pub row: usize,
    pub tier: usize,
}

pub struct Bench {
    pub frames: u64,
    pub open: bool,
    pub cursor: usize,
    pub tier: usize,
    pub fitted: Vec<Fitted>,
    pub charge: f32,
    last_action: u64,
}

static BENCH: OnceLock<Mutex<Bench>> = OnceLock::new();

pub fn bench() -> &'static Mutex<Bench> {
    BENCH.get_or_init(|| {
        Mutex::new(Bench {
            frames: 0,
            open: false,
            cursor: 0,
            tier: 0,
            fitted: Vec::new(),
            charge: 0.0,
            last_action: 0,
        })
    })
}

/// A read-only copy for the overlay, so the game's frame is never blocked by drawing.
pub struct View {
    pub frames: u64,
    pub open: bool,
    pub cursor: usize,
    pub tier: usize,
    pub charge: f32,
    pub fitted: Vec<Fitted>,
}

pub fn view() -> Option<View> {
    let b = bench().lock().ok()?;
    Some(View {
        frames: b.frames,
        open: b.open,
        cursor: b.cursor,
        tier: b.tier,
        charge: b.charge,
        fitted: b.fitted.clone(),
    })
}

enum Action {
    ToggleBench,
    Up,
    Down,
    TierUp,
    TierDown,
    Fit,
    Remove,
}

pub fn tick() {
    let Ok(mut b) = bench().lock() else { return };
    b.frames += 1;
    let frames = b.frames;
    if frames == 1 {
        log::line("tick: first frame seen");
    }
    if b.charge < 100.0 {
        b.charge = (b.charge + 0.08).min(100.0);
    }
    if frames.saturating_sub(b.last_action) <= keys::DEBOUNCE_FRAMES {
        return;
    }
    let Some(action) = read_action(b.open) else { return };
    b.last_action = frames;
    apply(&mut b, action);
}

fn read_action(bench_open: bool) -> Option<Action> {
    use eldenring::util::input::is_key_pressed;

    if is_key_pressed(keys::BENCH) {
        return Some(Action::ToggleBench);
    }
    if !bench_open {
        return None;
    }
    if is_key_pressed(keys::UP) {
        return Some(Action::Up);
    }
    if is_key_pressed(keys::DOWN) {
        return Some(Action::Down);
    }
    if is_key_pressed(keys::LEFT) {
        return Some(Action::TierDown);
    }
    if is_key_pressed(keys::RIGHT) {
        return Some(Action::TierUp);
    }
    if is_key_pressed(keys::CONFIRM) {
        return Some(Action::Fit);
    }
    if is_key_pressed(keys::REMOVE) {
        return Some(Action::Remove);
    }
    None
}

fn apply(b: &mut Bench, action: Action) {
    match action {
        Action::ToggleBench => {
            b.open = !b.open;
            log::line(if b.open { "bench: opened" } else { "bench: closed" });
        }
        Action::Up => {
            b.cursor = (b.cursor + CHROME_COUNT - 1) % CHROME_COUNT;
            log::line(&format!("bench: cursor on {}", CHROME[b.cursor].name));
        }
        Action::Down => {
            b.cursor = (b.cursor + 1) % CHROME_COUNT;
            log::line(&format!("bench: cursor on {}", CHROME[b.cursor].name));
        }
        Action::TierUp => {
            b.tier = (b.tier + 1).min(MAX_TIERS - 1);
            log::line(&format!("bench: tier {}", b.tier));
        }
        Action::TierDown => {
            b.tier = b.tier.saturating_sub(1);
            log::line(&format!("bench: tier {}", b.tier));
        }
        Action::Fit => {
            let row = &CHROME[b.cursor];
            if row.fidelity == Fidelity::NotPossible {
                log::line(&format!("bench: {} cannot exist here", row.name));
                return;
            }
            if !row.tiers.is_empty() && b.tier >= row.tiers.len() {
                b.tier = row.tiers.len() - 1;
            }
            b.fitted.retain(|f| CHROME[f.row].slot != row.slot);
            b.fitted.push(Fitted { row: b.cursor, tier: b.tier });
            log::line(&format!("bench: fitted {} (tier {})", row.name, b.tier));
        }
        Action::Remove => {
            let row = &CHROME[b.cursor];
            b.fitted.retain(|f| CHROME[f.row].slot != row.slot);
            log::line(&format!("bench: removed {}", row.name));
        }
    }
}

pub fn slot_label(slot: table::Slot) -> &'static str {
    match slot {
        table::Slot::FrontalCortex => "frontal cortex",
        table::Slot::OperatingSystem => "operating system",
        table::Slot::Arms => "arms",
        table::Slot::Hands => "hands",
        table::Slot::Skeleton => "skeleton",
        table::Slot::NervousSystem => "nervous system",
        table::Slot::Integumentary => "integumentary",
        table::Slot::Circulatory => "circulatory",
        table::Slot::Legs => "legs",
    }
}
