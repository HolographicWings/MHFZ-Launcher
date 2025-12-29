╔══════════════════════════════════════════════════════════════╗
║     MHFZ Launcher - Portable Edition for SteamOS/Linux       ║
║                     Version 1.4.7                            ║
╚══════════════════════════════════════════════════════════════╝

█ QUICK INSTALLATION (3 STEPS)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Open this folder in terminal (Konsole on Steam Deck)

2. Run the installation script:
   ./install-deps-steamos.sh

3. Launch the launcher:
   ./mhfz-launcher

Done! The launcher is ready.


█ ADD TO STEAM
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Switch to Desktop Mode (Steam > Power > Switch to Desktop)

2. Open Steam

3. Games → Add a Game → Add a Non-Steam Game

4. Click "Browse..."

5. Navigate to this folder

6. Select the file: mhfz-launcher

7. (Optional) Right-click → Properties → Change icon

✓ Now it works in Game Mode!


█ WHAT install-deps-steamos.sh DOES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

The script:
  ✓ Guides you through password setup (if needed)
  ✓ Temporarily unlocks the SteamOS filesystem
  ✓ Installs webkit2gtk (~50MB)
  ✓ Re-locks the filesystem
  ✓ Everything with clear explanations!

It's completely safe and reversible.


█ FILE STRUCTURE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

mhfz-launcher               ← Run this (wrapper)
mhfz-launcher.bin           ← Main binary (don't run directly)
install-deps-steamos.sh     ← Dependency installation script
README.txt                  ← This file


█ COMPATIBILITY
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✓ Steam Deck (SteamOS 3.x)
✓ Desktop Mode
✓ Big Picture Mode
✓ Game Mode
✓ Any Linux distro with webkit2gtk available


█ TROUBLESHOOTING
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Error: "webkit2gtk not found"
  → Run: ./install-deps-steamos.sh

Error: "Permission denied"
  → Run: chmod +x mhfz-launcher install-deps-steamos.sh

Launcher won't open from Steam:
  → Check that the path in Steam Properties is correct
  → Target must point to: .../mhfz-launcher (not .bin!)

After SteamOS update the launcher won't start:
  → SteamOS resets installed packages
  → Re-run: ./install-deps-steamos.sh


█ SUPPORT
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

GitHub: https://github.com/mrsasy89/MHFZ-Launcher
Discord: Check the GitHub repo for community links


█ IMPORTANT NOTES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

• The folder can be moved anywhere
• Works from SD card too
• The installation script needs to run ONLY ONCE
• SteamOS updates may require re-installation

Good hunting! 🎮
