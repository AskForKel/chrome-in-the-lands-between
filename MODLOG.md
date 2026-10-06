# Chrome in the Lands Between - mod log

Melty project: not created yet (list_my_mods returned none). modId: TBD
Host game: Elden Ring (elden-ring), exe 2.7.1.0 / 2.7.1.1, Steam, Windows x64
Loader: ModEngine2 (Melty installs release-2.1.0). Offline only, Easy Anti-Cheat untouched
Route: native Rust DLL (fromsoftware-rs 'eldenring' crate, Ww2710 RVA table) + ImGui via hudhook,
       loaded by ModEngine2. Rejected: params-only (cannot add mechanics or a UI), passthrough (no
       meaning offline), Elden Mod Loader (me3/ModEngine2 gives offline launch + separate save).
Prior art: universal-modder knowledge/games/elden-ring/cs2-conversion-of-elden-ring-offline-native-rust-dll-via-me3.md

Sheets (source of truth):
  sheets/chrome.json   one row per piece of chrome (slot, tiers, effect, trigger, cost, primitive)
  sheets/systems.json  the bench, slots, capacity, charge, keys, HUD, persistence
  sheets/hooks.json    every hook into the running game the chrome stands on

Decisions:
  - Single player. Elden Ring online + Easy Anti-Cheat is never entered; Melty launches offline.
  - v0.1 delivers the full cyberware catalogue (kinds x tiers), not just three implants.
  - No Cyberpunk 2077 assets are shipped. Cyberpunk art/models are CDPR's and I have no access to
    the user's install to convert them. v0.1 uses Elden Ring's own models and effects plus a bench
    panel we draw ourselves. Converted Cyberpunk visuals are a later, separate step.

Open:
  - Confirm the installed Elden Ring patch (2.7.1.0 or 2.7.1.1).
  - Projectile Launch System needs a bullet row in the player's regulation.bin (one file from the user).

Journal:
  2026-10-06  Melty connected; both games' requirements read; no mashup crosses these two games yet.
  2026-10-06  Design settled with the user: play in Elden Ring, Cyberpunk brings the chrome; bench at a
              Site of Grace; the user asked for the whole Cyberpunk cyberware list with options to swap.
  2026-10-06  Slice 1 BUILT. Repo https://github.com/AskForKel/chrome-in-the-lands-between (public,
              created through the GitHub account connection). .github/workflows/build.yml builds
              ChromeInTheLandsBetween.dll on windows-latest - no toolchain on anyone's PC.
              Run 1 failed: missing src/table.rs in the push + key codes as u32 where the game's reader
              wants i32. Both fixed; run 2 (37428333474) green.
              Artifact fetched and verified as a real binary, not just a green build: PE image, machine
              0x8664 (x86_64), GUI subsystem, 837,632 bytes, and our own HUD strings inside it.
              Distilled into the DLL: hudhook 0.9.3 (dx12) for the overlay, eldenring 0.14.0 +
              fromsoftware-shared 0.14.0 for the game side. Every dependency compiled - hudhook's dx12
              hook and the eldenring crate both built clean against our code.
              What slice 1 is: the chrome HUD and the ripperdoc bench, the whole catalogue browsable and
              fittable, held in memory. No game state is written yet - that is slice 2 by design, so a
              first in-game run can only fail on loading, not on mechanics.
              Package: dist/ChromeInTheLandsBetween-0.1.0.zip - mod/ChromeInTheLandsBetween/
              ChromeInTheLandsBetween.dll plus config_eldenring.toml listing that DLL in external_dlls
              (verified against ModEngine2's own docs and its source) plus a README. "um publish check"
              passes: 2 files, 0 failures, 0 warnings.
              Loading route decided: Mod Engine 2, which Melty installs for Elden Ring, reads
              external_dlls from config_eldenring.toml in its own folder. Two ways to get our config in
              front of it, to settle when the Melty entry is made: (a) point the entry's launch at our
              config with -c, (b) place ours at {managed}/modengine2/config_eldenring.toml. (a) is
              preferred: it never overwrites the loader's own file, at the cost of the player's other
              data mods not loading in that session.
              BLOCKED: no Melty connection in this session, so the mod entry, the upload, the Test run
              and install/launch telemetry are not done. Nothing is published - the listing is a draft.
              OPEN: the player's exact patch (2.7.1.0 or 2.7.1.1) - the built DLL targets 2.7.1.0's
              tables. Cyberpunk's own models/art are still out of scope: converting them needs the
              user's Cyberpunk install.
  2026-10-06  Release automation. The package layout now lives in the repo (packaging/: the loader
              config + the player README + release notes) and the workflow assembles the zip on the
              runner, so every build is package-and-release from a clean machine, no local toolchain.
              Pushing a tag publishes a GitHub release with the zip attached - that is the link Melty
              consumes. v0.1.0 is out:
              https://github.com/AskForKel/chrome-in-the-lands-between/releases/tag/v0.1.0
              The published zip was downloaded back and read: it carries
              mod/ChromeInTheLandsBetween/ChromeInTheLandsBetween.dll, the loader config listing it, and
              the player README. Verified, not assumed - the first zip I built was missing the DLL.
              Note for later: our config_eldenring.toml replaces the loader's own config for a player
              who already has Mod Engine 2 mods. Fine for the tester; the README says to paste our one
              external_dlls line into theirs instead before this goes public.
