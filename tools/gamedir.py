"""The player's game install folder, shared by the tools: HACKGU_GAME_DIR, else game_dir.txt (written by the asset
extractor), else common Steam folders. Mirrors crates/hackgu-render/src/gamedir.rs."""
import os

_GUESSES = [r"C:\Program Files (x86)\Steam\steamapps\common\hackGU"] + \
    [d + r":\SteamLibrary\steamapps\common\hackGU" for d in "CDEFG"]


def game_dir():
    if os.environ.get("HACKGU_GAME_DIR"):
        return os.environ["HACKGU_GAME_DIR"]
    here = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "game_dir.txt")
    if os.path.exists(here):
        p = open(here, encoding="utf8").read().strip()
        if os.path.exists(os.path.join(p, "hackGU_vol1.dll")):
            return p
    for p in _GUESSES:
        if os.path.exists(os.path.join(p, "hackGU_vol1.dll")):
            return p
    raise SystemExit("game install not found: run Extract_Game_Assets.bat first or set HACKGU_GAME_DIR")


def dll(name="hackGU_vol1.dll"):
    return os.path.join(game_dir(), name)
