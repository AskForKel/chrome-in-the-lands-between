import json, os
root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sheet = json.load(open(os.path.join(root, "sheets", "chrome.json")))
SLOT = {"frontal_cortex":"FrontalCortex","operating_system":"OperatingSystem","arms":"Arms","hands":"Hands","skeleton":"Skeleton","nervous_system":"NervousSystem","integumentary":"Integumentary","circulatory":"Circulatory","legs":"Legs"}
TRIG = {"key.os":"KeyOs","key.arms":"KeyArms","key.integ":"KeyInteg","key.util":"KeyUtil","passive":"Passive","none":"NotUsable"}
PRIM = {"stat_mod":"StatMod","on_hit_proc":"OnHitProc","timescale":"Timescale","weapon_mod":"WeaponMod","spawn_bullet":"SpawnBullet","projectile_steer":"ProjectileSteer","target_hack":"TargetHack","hud_markers":"HudMarkers","stealth":"Stealth","heal":"Heal","revive":"Revive","jump":"Jump","none":"NotUsed"}
TIER = {"common":"Common","rare":"Rare","epic":"Epic","legendary":"Legendary"}
FID = {"exact":"Exact","adapted":"Adapted","not-possible":"NotPossible"}
L = []
L.append("// GENERATED from sheets/chrome.json by tools/gen_table.py -- edit the sheet, never this file.")
L.append("")
L.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum Slot { FrontalCortex, OperatingSystem, Arms, Hands, Skeleton, NervousSystem, Integumentary, Circulatory, Legs }")
L.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum Tier { Common, Rare, Epic, Legendary }")
L.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum Trigger { Passive, KeyOs, KeyArms, KeyInteg, KeyUtil, NotUsable }")
L.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum Fidelity { Exact, Adapted, NotPossible }")
L.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum Primitive { StatMod, OnHitProc, Timescale, WeaponMod, SpawnBullet, ProjectileSteer, TargetHack, HudMarkers, Stealth, Heal, Revive, Jump, NotUsed }")
L.append("")
L.append("pub struct Chrome { pub id: &'static str, pub slot: Slot, pub name: &'static str, pub tiers: &'static [Tier], pub effect: &'static str, pub trigger: Trigger, pub charge: u8, pub cooldown: u8, pub fidelity: Fidelity, pub primitive: Primitive, pub needs_params: bool }")
L.append("")
L.append("pub const TIER_MULT: &[(Tier, f32)] = &[(Tier::Common, 1.00), (Tier::Rare, 1.25), (Tier::Epic, 1.50), (Tier::Legendary, 1.75)];")
L.append("")
L.append("pub const CHROME: &[Chrome] = &[")
for r in sheet["rows"]:
    tiers = ", ".join("Tier::" + TIER[t] for t in r["tiers"])
    L.append("    Chrome { id: %s, slot: Slot::%s, name: %s, tiers: &[%s], effect: %s, trigger: Trigger::%s, charge: %d, cooldown: %d, fidelity: Fidelity::%s, primitive: Primitive::%s, needs_params: %s }," % (json.dumps(r["id"]), SLOT[r["slot"]], json.dumps(r["name"]), tiers, json.dumps(r["er"]), TRIG[r["trigger"]], r["charge"], r["cooldown"], FID[r["fidelity"]], PRIM[r["primitive"]], "true" if r["needs_params"] else "false"))
L.append("];")
L.append("")
L.append("pub const CHROME_COUNT: usize = %d;" % len(sheet["rows"]))
open(os.path.join(root, "src", "table.rs"), "w").write("\n".join(L) + "\n")
print("rows=%d" % len(sheet["rows"]))
