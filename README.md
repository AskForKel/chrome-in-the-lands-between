# Chrome in the Lands Between

A Cyberpunk chrome mashup for **Elden Ring**: a ripperdoc bench at a Site of Grace, and the whole
Cyberpunk cyberware list to fit at it.

Built from the sheets in `sheets/` - `chrome.json` (one row per piece of chrome),
`systems.json` (the bench, keys, tiers, HUD) and `hooks.json` (every hook into the game).
`src/table.rs` is generated from `sheets/chrome.json` by `tools/gen_table.py`: edit the sheet,
never the generated file.

## Install

1. Download `ChromeInTheLandsBetween-<version>.zip` from this repo's **Releases**.
2. Unzip it straight into your **Mod Engine 2** folder - the one the launcher set up, the folder
   holding `modengine2_launcher.exe`. You should end up with:
   - `mod\\ChromeInTheLandsBetween\\ChromeInTheLandsBetween.dll`
   - `config_eldenring.toml` (already lists that DLL for you)
3. Already have Mod Engine 2 mods enabled? Then don't overwrite `config_eldenring.toml`. Open it and
   add this line inside its `external_dlls` list instead:

       "mod\\ChromeInTheLandsBetween\\ChromeInTheLandsBetween.dll",

4. Launch Elden Ring offline through Mod Engine 2. Press **F** in game - the ripperdoc bench opens.

Nothing is copied over the game's own files, and Easy Anti-Cheat is never touched.

## Check your game version

The mod's memory tables are built for **Elden Ring 1.17.1 (eldenring.exe 2.7.1.0)**.

- In game: the title screen shows `App Ver.` in its bottom corner. 1.17.1 is what this build wants.
- Exactly: right-click `eldenring.exe` in your game folder, **Properties > Details**, and read
  **File version**. It should read `2.7.1.0`.
- A launcher that manages the game (Melty included) also shows the version it detects for the
  installed game.

On a different patch the mod may still load, but nothing is promised on that - the tables are
per-patch.

## Publishing it on Melty (entry recipe)

Melty's entry wants a loader and a download, not a hand-copied folder:

- **mode:** installed - the package installs into the loader's own folder, nothing goes over the
  game's files.
- **requirement:** the **Mod Engine 2** loader, so Melty installs and launches it for the player.
- **main component:** this repo's release `.zip` (one entry = one download).
- **target:** the loader's folder - `{managed}/modengine2/` - so the DLL lands in
  `mod/ChromeInTheLandsBetween/` and `config_eldenring.toml` sits beside `modengine2_launcher.exe`.

If Melty's tester has other Mod Engine 2 mods, our `config_eldenring.toml` replaces their config
list; the honest fix for a public release is to paste our one `external_dlls` line into the
existing config instead of shipping ours. Worth knowing before this leaves draft.

## Building

The DLL is built on GitHub's Windows runners - see `.github/workflows/build.yml`. Nothing needs to
be installed locally: push, and each run produces the DLL plus the ready-to-install package.

Pushing a version tag (e.g. `git tag v0.1.0 && git push origin v0.1.0`) builds and **publishes a
GitHub release** with `ChromeInTheLandsBetween-<tag>.zip` attached - that is the link Melty consumes.

To build by hand on a Windows machine with the Rust toolchain:

    cargo build --release --target x86_64-pc-windows-msvc

## Status

Slice 1: loads, draws the chrome HUD and the ripperdoc bench, reads its own keys, and keeps the
fitted chrome in memory. The effects on the game land in slice 2. Everything still to be confirmed
in game is listed in `MODLOG.md`.
