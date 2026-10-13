#!/usr/bin/env python3
"""
.hack//G.U. Asset Extraction Tool (BYOG - Bring Your Own Game)
Extracts required game assets from a legitimate copy of .hack//G.U. Last Recode (PC)
or PlayStation 2 ISO disc images into the project target directory.

Zero assets are bundled with the repository.
"""

import sys
import os
import shutil
import struct
import subprocess
from pathlib import Path

COMMON_STEAM_PATHS = [
    r"F:\SteamLibrary\steamapps\common\hackGU",
    r"C:\Program Files (x86)\Steam\steamapps\common\hackGU",
    r"C:\SteamLibrary\steamapps\common\hackGU",
    r"D:\SteamLibrary\steamapps\common\hackGU",
    r"E:\SteamLibrary\steamapps\common\hackGU",
    r"G:\SteamLibrary\steamapps\common\hackGU",
]

# Archives the Vol.1 port reads, and where they go. Each volume has a main (_a) and an index (_i) archive plus PC
# (_pc) companions holding the PC-specific files (English UI textures, button glyphs, messageInfo...); all four are
# extracted into the same folder.
# File paths inside the archives start with their volume folder (vol1/..., cmn/...), so they unpack into target/.
LAST_RECODE_MAPPINGS = [
    (prefix + suffix, Path("target"))
    for prefix in ("hackGU_vol1", "hackGU_cmn")
    for suffix in ("_a.cpk", "_i.cpk", "_a_pc.cpk", "_i_pc.cpk")
]


def remember_game_dir(repo_root: Path, game_dir: Path):
    """Sound, movies and the game DLL are read from the install at run time (game_dir.txt, see gamedir.rs)."""
    (repo_root / "game_dir.txt").write_text(str(game_dir.resolve()), encoding="utf8")
    print(f"[OK] Remembered the game folder: {game_dir}")


def generate_derived_data(repo_root: Path):
    """Data the port computes from the game's own code (event locations, story dungeon layouts). Needs Python
    packages capstone and unicorn."""
    print("\n[DATA] Installing the tool packages (capstone, unicorn)...")
    subprocess.run([sys.executable, "-m", "pip", "install", "-q", "capstone", "unicorn"], cwd=repo_root)
    steps = [
        [sys.executable, "tools/event_locations.py"],
        [sys.executable, "tools/enemy_table.py"],
        [sys.executable, "tools/dungeon_layout.py", "11", "27", "21", "target/vol1/data/data/ev/map/m103.ccs",
         "assets/dungeons/m103.json"],
    ]
    for cmd in steps:
        print("[DATA] " + " ".join(cmd[1:]))
        if subprocess.run(cmd, cwd=repo_root).returncode != 0:
            print("[ERROR] Generating game data failed; see the message above.")
            return False
    return True


class Iso9660Extractor:
    """Minimal standalone ISO 9660 reader using standard Python."""
    SECTOR_SIZE = 2048

    def __init__(self, iso_path: Path):
        self.iso_path = iso_path
        self.file = open(iso_path, "rb")

    def close(self):
        self.file.close()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()

    def read_sector(self, lba: int, count: int = 1) -> bytes:
        self.file.seek(lba * self.SECTOR_SIZE)
        return self.file.read(count * self.SECTOR_SIZE)

    def is_valid_iso(self) -> bool:
        try:
            pvd = self.read_sector(16)
            return len(pvd) >= 6 and pvd[1:6] == b"CD001"
        except Exception:
            return False

    def extract_directory(self, lba: int, size: int, dest_dir: Path):
        dest_dir.mkdir(parents=True, exist_ok=True)
        sectors_to_read = (size + self.SECTOR_SIZE - 1) // self.SECTOR_SIZE
        data = self.read_sector(lba, sectors_to_read)[:size]

        offset = 0
        while offset < len(data):
            rec_len = data[offset]
            if rec_len == 0:
                # Padding to sector boundary
                offset = ((offset // self.SECTOR_SIZE) + 1) * self.SECTOR_SIZE
                continue

            if offset + rec_len > len(data):
                break

            rec = data[offset:offset + rec_len]
            ext_lba = struct.unpack_from("<I", rec, 2)[0]
            data_len = struct.unpack_from("<I", rec, 10)[0]
            flags = rec[25]
            name_len = rec[32]
            raw_name = rec[33:33 + name_len]

            offset += rec_len

            # Skip current (0x00) and parent (0x01) dir pointers
            if raw_name in (b"\x00", b"\x01"):
                continue

            name = raw_name.decode("latin1", errors="replace").split(";")[0]

            if flags & 0x02: # Directory
                sub_dir = dest_dir / name
                self.extract_directory(ext_lba, data_len, sub_dir)
            else: # File
                target_file = dest_dir / name
                print(f"  Extracting PS2 ISO file: {target_file.name} ({data_len:,} bytes)...")
                self.file.seek(ext_lba * self.SECTOR_SIZE)
                with open(target_file, "wb") as out_f:
                    remaining = data_len
                    chunk_sz = 64 * 1024
                    while remaining > 0:
                        to_read = min(remaining, chunk_sz)
                        chunk = self.file.read(to_read)
                        if not chunk:
                            break
                        out_f.write(chunk)
                        remaining -= len(chunk)


def find_last_recode_install(user_input: str = None) -> Path:
    if user_input:
        p = Path(user_input.strip('"\''))
        if p.is_file():
            if p.suffix.lower() == ".cpk":
                return p.parent.parent if p.parent.name.lower() == "cpk" else p.parent
            if p.suffix.lower() == ".iso":
                return p
        if p.is_dir():
            return p

    for path_str in COMMON_STEAM_PATHS:
        p = Path(path_str)
        if p.exists() and (p / "cpk").exists():
            return p

    return None


def run_cpk_extractor(repo_root: Path, cpk_path: Path, out_dir: Path):
    out_dir.mkdir(parents=True, exist_ok=True)
    print(f"\n[CPK] Extracting {cpk_path.name} -> {out_dir}...")
    cmd = [
        "cargo", "run", "--release", "-p", "hackgu-cpk", "--",
        "extract", str(cpk_path.resolve()), "--out", str(out_dir.resolve())
    ]
    res = subprocess.run(cmd, cwd=repo_root)
    if res.returncode != 0:
        print(f"[ERROR] Failed to extract {cpk_path.name} (exit code {res.returncode})")
    else:
        print(f"[OK] {cpk_path.name} extracted successfully.")


def extract_from_steam_folder(repo_root: Path, game_dir: Path):
    cpk_dir = game_dir / "cpk"
    if not cpk_dir.exists():
        cpk_dir = game_dir
        game_dir = game_dir.parent
    if not (game_dir / "hackGU_vol1.dll").exists():
        print(f"[ERROR] {game_dir} does not look like the .hack//G.U. Last Recode folder (no hackGU_vol1.dll).")
        sys.exit(1)

    print(f"\nScanning for Last Recode CPKs in: {cpk_dir}")
    print("[NOTE] The first archive takes a minute or two longer: the unpacking tool is compiled first.")
    missing = []
    for cpk_name, target_rel in LAST_RECODE_MAPPINGS:
        src_cpk = cpk_dir / cpk_name
        if src_cpk.exists():
            run_cpk_extractor(repo_root, src_cpk, repo_root / target_rel)
        else:
            missing.append(cpk_name)
    if missing:
        print("[ERROR] Missing archives: " + ", ".join(missing))
        sys.exit(1)
    remember_game_dir(repo_root, game_dir)
    if not generate_derived_data(repo_root):
        sys.exit(1)


def extract_from_iso(repo_root: Path, iso_path: Path):
    print(f"\n[ISO] Inspecting disc image: {iso_path}")
    try:
        with Iso9660Extractor(iso_path) as iso:
            if not iso.is_valid_iso():
                print("[ERROR] The provided file is not a valid standard ISO 9660 disc image.")
                return

            pvd = iso.read_sector(16)
            root_rec = pvd[156:156 + 34]
            root_lba = struct.unpack_from("<I", root_rec, 2)[0]
            root_size = struct.unpack_from("<I", root_rec, 10)[0]

            iso_dest = repo_root / "target" / "ps2_extracted" / iso_path.stem
            print(f"[ISO] Extracting PS2 ISO contents to: {iso_dest}...")
            iso.extract_directory(root_lba, root_size, iso_dest)
            print(f"[OK] Files copied to {iso_dest}.")
            print("[NOTE] The port cannot run from PS2 discs yet: it is built on the Last Recode (PC) files and")
            print("       its game code (hackGU_vol1.dll). Please point this tool at your Last Recode install.")
    except Exception as e:
        print(f"[ERROR] Failed extracting ISO: {e}")


def main():
    repo_root = Path(__file__).resolve().parent.parent
    print("=" * 70)
    print("      .hack//G.U. Asset Extractor (BYOG Setup)")
    print("=" * 70)
    print("This utility pulls required game assets from your legitimately owned copy.")
    print("Zero copyrighted assets or binaries are distributed with this project.")
    print("=" * 70)

    target_input = sys.argv[1] if len(sys.argv) > 1 else None

    if target_input in ("-h", "--help"):
        print("\nUsage:")
        print("  python tools/extract_assets.py [path_to_game_dir | path_to_cpk | path_to_iso]")
        print("\nDrag and Drop:")
        print("  Drag your Steam hackGU directory, a .cpk, or a PS2 .iso onto Extract_Game_Assets.bat")
        sys.exit(0)

    if target_input:
        chosen_path = Path(target_input.strip('"\''))
    else:
        detected = find_last_recode_install()
        if detected and detected.is_dir():
            print(f"\nAuto-detected Steam installation at:\n  {detected}")
            ans = input("\nWould you like to extract assets from this install? [Y/n]: ").strip().lower()
            if ans in ("", "y", "yes"):
                chosen_path = detected
            else:
                chosen_path = None
        else:
            chosen_path = None

    if not chosen_path:
        print("\nPlease drag and drop or enter the path to:")
        print("  1) Your .hack//G.U. Last Recode folder (e.g. F:\\SteamLibrary\\...\\hackGU)")
        print("  2) A specific .cpk file (e.g. hackGU_vol1_a.cpk)")
        print("  3) A PlayStation 2 .iso disc image file")
        entered = input("\nPath: ").strip('"\' ')
        if not entered:
            print("No path provided. Exiting.")
            sys.exit(1)
        chosen_path = Path(entered)

    if not chosen_path.exists():
        print(f"[ERROR] Path does not exist: {chosen_path}")
        sys.exit(1)

    if chosen_path.is_file():
        ext = chosen_path.suffix.lower()
        if ext == ".iso":
            extract_from_iso(repo_root, chosen_path)
        elif ext == ".cpk":
            # All the volume's archives are needed: use the folder the .cpk is in.
            extract_from_steam_folder(repo_root, chosen_path.parent)
        else:
            print(f"[ERROR] Unsupported file extension: {ext}. Expected .cpk or .iso")
            sys.exit(1)
    elif chosen_path.is_dir():
        extract_from_steam_folder(repo_root, chosen_path)

    print("\n" + "=" * 70)
    print("Asset extraction setup complete!")
    print("(PS2 discs are not supported yet; see above if you used one.)") if str(chosen_path).lower().endswith('.iso') else None
    print("You can now build and run the game with: play_game.bat")
    print("=" * 70)


if __name__ == "__main__":
    main()
