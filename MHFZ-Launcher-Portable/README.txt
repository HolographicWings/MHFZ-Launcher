╔══════════════════════════════════════════════════════════════╗
║     MHFZ Launcher - Portable Edition for SteamOS/Linux       ║
║                     Version 1.4.7                            ║
╚══════════════════════════════════════════════════════════════╝

█ QUICK INSTALLATION (4 STEPS)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Place this folder in: /home/deck/MHFZ/

2. Open Konsole (terminal) and run:
   cd /home/deck/MHFZ
   chmod +x install-deps-steamos.sh mhfz-launcher
   ./install-deps-steamos.sh

3. Create Font/ folder and add ONLY these 2 files:
   mkdir -p Font
   # Copy msgothic.ttc and "MS Gothic.ttf" to Font/

4. Launch the launcher:
   ./mhfz-launcher

Done! The launcher is ready.


█ ⚠️  CRITICAL - FONT CONFIGURATION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

The launcher requires EXACTLY 2 font files in Font/ folder:

  ✓ msgothic.ttc
  ✓ MS Gothic.ttf

❌ DO NOT add other fonts (NotoSans, Meiryo, YuGothic, etc.)
   Too many fonts (13+) will BREAK Japanese text rendering!

The launcher automatically:
  • Cleans old fonts before installation
  • Installs ONLY MS Gothic (max 2 files)
  • Configures Wine Flatpak permissions
  • Passes --env=WINEPREFIX correctly to Flatpak


█ FOLDER STRUCTURE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/home/deck/MHFZ/
├── mhfz-launcher               ← Run this (wrapper)
├── mhfz-launcher.bin           ← Main binary
├── install-deps-steamos.sh     ← Dependency installer
├── README.txt                  ← This file
├── Font/                       ← ⚠️  Create this!
│   ├── msgothic.ttc           ← Required
│   └── MS Gothic.ttf          ← Required
├── pfx/                        ← Auto-created (Wine prefix)
├── mhf-iel-cli.exe            ← Game executable
└── (other game files...)


█ ADD TO STEAM
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Switch to Desktop Mode:
   Steam → Power → Switch to Desktop

2. Open Steam

3. Games → Add a Game → Add a Non-Steam Game

4. Click "Browse..." and navigate to:
   /home/deck/MHFZ/

5. Select: mhfz-launcher

6. (Optional) Right-click → Properties → Change icon

✓ Now it works in Game Mode!


█ WHAT install-deps-steamos.sh DOES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

The script:
  ✓ Guides you through password setup (if needed)
  ✓ Temporarily unlocks the SteamOS filesystem
  ✓ Installs webkit2gtk (~50MB - launcher GUI)
  ✓ Installs Wine Flatpak (org.winehq.Wine stable-25.08)
  ✓ Re-locks the filesystem
  ✓ Provides font configuration instructions

It's completely safe and reversible.


█ FIRST LAUNCH
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

When you run ./mhfz-launcher for the first time:

1. Wine prefix initialization (1-2 minutes)
   → Creates /home/deck/MHFZ/pfx/

2. Flatpak permissions configuration
   → Grants Wine access to game folder

3. Font installation (automatic)
   → Cleans Fonts/ directory
   → Installs ONLY msgothic.ttc + MS Gothic.ttf
   → Registers fonts in Wine registry

4. Game launch
   → mhf-iel-cli.exe starts via Wine Flatpak

Check logs: ~/mhfz-launcher.log


█ COMPATIBILITY
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✓ Steam Deck (SteamOS 3.x)
✓ Desktop Mode
✓ Big Picture Mode
✓ Game Mode
✓ Any Linux distro with webkit2gtk and Wine Flatpak


█ TROUBLESHOOTING
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Error: "webkit2gtk not found"
  → Run: ./install-deps-steamos.sh

Error: "Permission denied"
  → Run: chmod +x mhfz-launcher install-deps-steamos.sh

Japanese text not showing in game:
  → Check Font/ folder has ONLY msgothic.ttc + MS Gothic.ttf
  → Remove all other fonts (NotoSans, Meiryo, etc.)
  → Delete pfx/ folder and relaunch to rebuild prefix

Launcher won't open from Steam:
  → Check Steam Properties target points to:
    /home/deck/MHFZ/mhfz-launcher (NOT .bin!)

After SteamOS update launcher won't start:
  → SteamOS resets installed packages
  → Re-run: ./install-deps-steamos.sh

Game crashes with font errors:
  → Too many fonts installed!
  → Run in terminal:
    rm -f /home/deck/MHFZ/pfx/drive_c/windows/Fonts/*
    # Then relaunch launcher (will reinstall only 2 fonts)


█ TECHNICAL DETAILS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Wine Command (SteamOS):
  flatpak run --env=WINEPREFIX=/home/deck/MHFZ/pfx     org.winehq.Wine mhf-iel-cli.exe

Wine Prefix:
  • Location: /home/deck/MHFZ/pfx/
  • Type: Local (NOT Flatpak's internal prefix)
  • Auto-created on first launch

Font Installation Process:
  1. Clean: rm -f pfx/drive_c/windows/Fonts/*
  2. Copy: ONLY msgothic.ttc + MS Gothic.ttf
  3. Register: Wine registry entries created
  4. Max 2 fonts enforced by launcher code

Flatpak Permissions:
  • Granted via: flatpak override --user --filesystem=...
  • Allows Wine to access /home/deck/MHFZ/
  • Configured automatically by launcher


█ SUPPORT
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

GitHub: https://github.com/mrsasy89/MHFZ-Launcher
Discord: Check the GitHub repo for community links
Logs: ~/mhfz-launcher.log


█ IMPORTANT NOTES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

• Folder can be moved, but update Steam shortcut path
• Works from SD card (performance may vary)
• install-deps-steamos.sh needs to run ONLY ONCE
• SteamOS updates may require re-running installer
• Font/ folder with 2 files is MANDATORY for Japanese text

Good hunting! 🎮
