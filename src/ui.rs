//! The overlay: the small chrome HUD and the ripperdoc bench panel.

use crate::state;
use crate::table::{self, CHROME, CHROME_COUNT, Fidelity, Tier};
use hudhook::imgui::{Condition, Ui};
use hudhook::ImguiRenderLoop;

const WINDOW_HUD: &str = "Chrome in the Lands Between";
const WINDOW_BENCH: &str = "Ripperdoc bench";
const ROWS_AROUND_CURSOR: usize = 7;

pub struct Overlay;

impl Overlay {
    pub fn new() -> Self {
        Self
    }
}

impl ImguiRenderLoop for Overlay {
    fn render(&mut self, ui: &mut Ui) {
        let Some(view) = state::view() else { return };

        ui.window(WINDOW_HUD)
            .size([320.0, 150.0], Condition::FirstUseEver)
            .position([16.0, 16.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("CHROME IN THE LANDS BETWEEN  v0.1");
                ui.separator();
                ui.text(format!("charge   {:>3.0} / 100", view.charge));
                ui.text(format!("fitted   {} of 9 slots", view.fitted.len()));
                ui.text(format!("frames   {}", view.frames));
                ui.separator();
                ui.text("[F] ripperdoc bench");
            });

        if view.open {
            ui.window(WINDOW_BENCH)
                .size([560.0, 430.0], Condition::FirstUseEver)
                .position([360.0, 60.0], Condition::FirstUseEver)
                .build(|| {
                    ui.text("[up/down] browse   [left/right] tier   [enter] fit   [del] remove   [F] close");
                    ui.separator();
                    let lo = view.cursor.saturating_sub(ROWS_AROUND_CURSOR);
                    let hi = (view.cursor + ROWS_AROUND_CURSOR).min(CHROME_COUNT - 1);
                    for i in lo..=hi {
                        let row = &CHROME[i];
                        let cursor_mark = if i == view.cursor { ">" } else { " " };
                        let fitted_mark = if view.fitted.iter().any(|f| f.row == i) { "*" } else { " " };
                        ui.text(format!(
                            "{cursor_mark}{fitted_mark} {:<26} {:<16} {}",
                            row.name,
                            state::slot_label(row.slot),
                            tier_label(row, view.tier)
                        ));
                    }
                    ui.separator();
                    let row = &CHROME[view.cursor];
                    ui.text(format!("{} - {}", row.name, state::slot_label(row.slot)));
                    ui.text(row.effect);
                    if row.fidelity == Fidelity::NotPossible {
                        ui.text("cannot exist in Elden Ring");
                    }
                });
        }
    }
}

fn tier_label(row: &table::Chrome, tier: usize) -> &'static str {
    match row.tiers.get(tier) {
        Some(Tier::Common) => "common",
        Some(Tier::Rare) => "rare",
        Some(Tier::Epic) => "epic",
        Some(Tier::Legendary) => "legendary",
        None => "-",
    }
}
