# Chrome in the Lands Between

A Cyberpunk chrome mashup for **Elden Ring**: a ripperdoc bench at a Site of Grace, and the
whole Cyberpunk cyberware list to fit at it.

Built from the sheets in `sheets/` - `chrome.json` (one row per piece of chrome),
`systems.json` (the bench, keys, tiers, HUD) and `hooks.json` (every hook into the game).
`src/table.rs` is generated from `sheets/chrome.json` by `tools/gen_table.py`: edit the sheet,
never the generated file.

## Building

The DLL is built on GitHub's Windows runners - see `.github/workflows/build.yml`. Nothing needs
to be installed locally: push, and the built `ChromeInTheLandsBetween.dll` is attached to the run
as an artifact.

To build by hand on a Windows machine with the Rust toolchain:

    cargo build --release --target x86_64-pc-windows-msvc

## Loading it

The DLL is a native for **ModEngine2**. It is loaded offline; Easy Anti-Cheat is never touched.

## Status

Slice 1: loads, draws the chrome HUD and the ripperdoc bench, reads its own keys, and keeps the
fitted chrome in memory. The effects on the game land in slice 2. Everything still to be
confirmed in game is listed in `MODLOG.md`.
