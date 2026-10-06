//! Chrome in the Lands Between - Cyberpunk chrome inside Elden Ring.
//!
//! Slice 1: the mod loads, draws its own panel over the game, reads only its own keys and
//! holds the ripperdoc bench state. No game state is written yet - slice 2 adds the effects.

mod keys;
mod log;
mod state;
mod table;
mod ui;

use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::windows::Win32::Foundation::HINSTANCE;
use hudhook::windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use hudhook::Hudhook;
use std::time::Duration;

/// Entry point. ModEngine2 loads this file as a native DLL.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(
    hmodule: HINSTANCE,
    reason: u32,
    _reserved: *mut std::ffi::c_void,
) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        log::init();
        log::line("boot: dll attached");
        let hmodule_raw = hmodule.0 as usize;
        std::thread::spawn(move || attach_overlay(hmodule_raw));
        std::thread::spawn(attach_game_task);
    }
    true
}

fn attach_overlay(hmodule_raw: usize) {
    let hmodule = HINSTANCE(hmodule_raw as _);
    let result = Hudhook::builder()
        .with::<ImguiDx12Hooks>(ui::Overlay::new())
        .with_hmodule(hmodule)
        .build()
        .apply();
    match result {
        Ok(()) => log::line("boot: d3d12 overlay attached"),
        Err(e) => log::line(&format!("boot: overlay failed: {e:?}")),
    }
}

/// Waits for the game's task runner and registers a per-frame callback.
/// The runner is the one thing the game guarantees to have once it is up.
fn attach_game_task() {
    use eldenring::cs::{CSTaskGroupIndex, CSTaskImp};
    use eldenring::fd4::FD4TaskData;
    use fromsoftware_shared::{FromStatic, SharedTaskImpExt};

    let task = match CSTaskImp::wait_for_instance(Duration::from_secs(300)) {
        Ok(task) => task,
        Err(e) => {
            log::line(&format!("boot: game task not found: {e:?}"));
            return;
        }
    };
    log::line("boot: game task found, frame callback registered");
    task.run_recurring(
        |_: &FD4TaskData| {
            state::tick();
        },
        CSTaskGroupIndex::FrameBegin,
    );
}
