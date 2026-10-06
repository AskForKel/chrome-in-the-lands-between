# Chrome in the Lands Between - v0.1 (bench build)

Cyberpunk chrome inside Elden Ring: a ripperdoc bench at a Site of Grace, and the whole cyberware
list to fit at it.

## Needs
- Elden Ring 1.17.1 (eldenring.exe 2.7.1.0), played offline. Easy Anti-Cheat is never touched.
- Mod Engine 2 - the launcher installs it for you.

## What this build does
- Opens its own chrome HUD, and the ripperdoc bench on F.
- All 37 pieces of chrome: browse them, set a tier, fit them to their slot. Fitted chrome stays
  fitted for the session.
- It does not touch the game's own numbers yet. Bullet time, the arm cannon, the quickhacks and the
  passives land in the next build - this one is here so the loading is proven first.

## Keys
| key | does |
|---|---|
| F | open / close the ripperdoc bench |
| up / down | browse chrome |
| left / right | tier |
| enter | fit it to its slot |
| delete | take it out of its slot |

## If the HUD does not appear
- `ChromeInTheLandsBetween.log`, next to `eldenring.exe`, says what the mod saw. The last lines
  are the ones that matter.
- Nothing at all? The mod never loaded. Check that the `external_dlls` line in
  `config_eldenring.toml` points at
  `mod\\ChromeInTheLandsBetween\\ChromeInTheLandsBetween.dll` and that this file is there.

## Credits
- Built by an AI agent (Base 1 on Base44) working from the user's own design sheets. The user
  directs and plays.
- Elden Ring is FromSoftware's. This mod ships none of its files.
