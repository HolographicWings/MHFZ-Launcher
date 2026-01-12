
╔══════════════════════════════════════════════════════════════╗
║  MHFZ Launcher - File Verificati e Pronti per Build SteamOS  ║
║                     Build v1.4.7 - Ready!                     ║
╚══════════════════════════════════════════════════════════════╝

📁 FILE MODIFICATI/CREATI
═══════════════════════════════════════════════════════════════

1. ✅ lib_linux.rs (NUOVO - 540 righe)
   ├─ Funzione: configure_flatpak_permissions() [NUOVA]
   ├─ Funzione: install_japanese_fonts() [RISCRITTA]
   │  ├─ Cerca Font/ o fonts/
   │  ├─ PULISCE directory Fonts/ prima di installare
   │  ├─ WHITELIST: SOLO msgothic.ttc + MS Gothic.ttf
   │  └─ STOP dopo 2 font (max)
   ├─ Funzione: run_linux() [MODIFICATA]
   │  ├─ Chiama configure_flatpak_permissions()
   │  └─ Lancio gioco: --env=WINEPREFIX per Flatpak
   └─ Compatibilità: 100% con wrapper e script

2. ✅ install-deps-steamos.sh (AGGIORNATO - v2.1)
   ├─ STEP 1-6: Invariati (password, unlock, keys, webkit, Wine, lock)
   ├─ STEP 5: Installa Wine Flatpak stable-25.08
   └─ NUOVO: Istruzioni dettagliate sui font (max 2 file)

3. ✅ README.txt (AGGIORNATO - 5.5KB)
   ├─ Sezione CRITICAL: Configurazione font (SOLO 2 file)
   ├─ Sezione FOLDER STRUCTURE: Struttura chiara
   ├─ Sezione TROUBLESHOOTING: Font errors risolti
   └─ Sezione TECHNICAL DETAILS: Comandi Wine Flatpak

4. ✅ mhfz-launcher (wrapper bash - INVARIATO)
   ├─ Check webkit2gtk
   ├─ Esporta FONTCONFIG_PATH, FONTCONFIG_FILE, etc.
   └─ Lancia mhfz-launcher.bin

5. ✅ build-steamos.sh (script build - INVARIATO)
   ├─ Compila frontend (npm)
   ├─ Compila binario Rust
   ├─ Crea wrapper
   ├─ Copia install-deps e README
   └─ Genera archivio .tar.gz


🔍 VERIFICA COMPATIBILITÀ
═══════════════════════════════════════════════════════════════

┌─────────────────────────┬──────────────┬─────────────────────┐
│ Componente              │ File         │ Stato               │
├─────────────────────────┼──────────────┼─────────────────────┤
│ Wine Flatpak detection  │ lib_linux.rs │ ✅ is_steamos()     │
│ Wine command selection  │ lib_linux.rs │ ✅ get_wine_command()│
│ Flatpak permissions     │ lib_linux.rs │ ✅ NUOVO: auto-conf │
│ Font cleaning           │ lib_linux.rs │ ✅ NUOVO: rm old    │
│ Font whitelist          │ lib_linux.rs │ ✅ NUOVO: max 2     │
│ --env=WINEPREFIX        │ lib_linux.rs │ ✅ NUOVO: Flatpak   │
│ Wrapper env vars        │ mhfz-launcher│ ✅ FONTCONFIG_*     │
│ Wine installation       │ install-deps │ ✅ Flatpak stable   │
│ User instructions       │ README.txt   │ ✅ Font warnings    │
└─────────────────────────┴──────────────┴─────────────────────┘


🎯 WORKFLOW DI BUILD E TEST
═══════════════════════════════════════════════════════════════

STEP 1: Sostituisci i file nel progetto
─────────────────────────────────────────────────────────────

cd /path/to/MHFZ-Launcher

# Backup (opzionale)
cp src-tauri/src/lib_linux.rs src-tauri/src/lib_linux.rs.backup

# Sostituisci file modificati
cp lib_linux.rs src-tauri/src/lib_linux.rs
cp install-deps-steamos.sh MHFZ-Launcher-Portable/install-deps-steamos.sh
cp README.txt MHFZ-Launcher-Portable/README.txt

STEP 2: Build per SteamOS
─────────────────────────────────────────────────────────────

chmod +x build-steamos.sh
./build-steamos.sh

Output atteso:
  ✅ Build frontend
  ✅ Build binario Rust
  ✅ Creato wrapper con env vars
  ✅ Archivio: MHFZ-Launcher-SteamOS-v1.4.7.tar.gz

STEP 3: Deploy su Steam Deck
─────────────────────────────────────────────────────────────

# Da PC di sviluppo:
scp MHFZ-Launcher-SteamOS-v1.4.7.tar.gz deck@steamdeck:/home/deck/

# Su Steam Deck (Konsole):
cd /home/deck
tar -xzf MHFZ-Launcher-SteamOS-v1.4.7.tar.gz
cd MHFZ-Launcher-SteamOS-v1.4.7

STEP 4: Installa dipendenze (PRIMA VOLTA)
─────────────────────────────────────────────────────────────

./install-deps-steamos.sh

Verifica output:
  ✅ Password configurata
  ✅ Filesystem unlocked/locked
  ✅ webkit2gtk installato
  ✅ Wine Flatpak installato
  ✅ Istruzioni font mostrate

STEP 5: Prepara font (CRITICO!)
─────────────────────────────────────────────────────────────

mkdir -p Font
# Copia SOLO questi 2 file:
cp /path/to/msgothic.ttc Font/
cp "/path/to/MS Gothic.ttf" Font/

# ⚠️  Verifica che ci siano SOLO 2 file:
ls -la Font/
# Output atteso:
# msgothic.ttc
# MS Gothic.ttf

STEP 6: Test lancio
─────────────────────────────────────────────────────────────

./test-steamos.sh

Oppure direttamente:
./mhfz-launcher

Verifica log in tempo reale:
tail -f ~/mhfz-launcher.log


📋 LOG ATTESI (~/mhfz-launcher.log)
═══════════════════════════════════════════════════════════════

[TIMESTAMP] 🎮 Monster Hunter Frontier Z - Linux Launcher
[TIMESTAMP] 🎮 Detected SteamOS
[TIMESTAMP] 🍷 Using Wine Flatpak (SteamOS)
[TIMESTAMP] 🍷 WINEPREFIX: /home/deck/MHFZ/.../pfx
[TIMESTAMP] 🔧 First launch detected - Wine prefix does not exist
[TIMESTAMP] ⏳ Running wineboot --init...
[TIMESTAMP] ✅ wineboot --init completed successfully
[TIMESTAMP] 🔐 Configuring Flatpak permissions...
[TIMESTAMP] ✅ Flatpak permissions granted for: /home/deck/MHFZ/...
[TIMESTAMP] 🧹 Cleaning existing fonts (CRITICAL for SteamOS)...
[TIMESTAMP]   Removed 0 old font file(s)
[TIMESTAMP] 🔤 Installing MS Gothic fonts (MAX 2 files for SteamOS)...
[TIMESTAMP]   ✅ Installed: "msgothic.ttc"
[TIMESTAMP]   ✅ Installed: "MS Gothic.ttf"
[TIMESTAMP]   ⚠️ Reached max 2 fonts, stopping
[TIMESTAMP] ✅ MS Gothic fonts installed (2 file(s) - OPTIMAL for SteamOS)
[TIMESTAMP] 📝 Registering fonts in Wine registry...
[TIMESTAMP] ✅ Fonts registered in Wine registry
[TIMESTAMP] 🚀 Launching game...
[TIMESTAMP]   Wine command: flatpak ["run", "org.winehq.Wine"]
[TIMESTAMP]   🔑 Using Flatpak --env=WINEPREFIX=/home/deck/MHFZ/.../pfx
[TIMESTAMP] ✅ Game launched successfully (PID: XXXX)
[TIMESTAMP] 🎮 Game is running!


❌ ERRORI COMUNI E SOLUZIONI
═══════════════════════════════════════════════════════════════

Errore: "webkit2gtk not found"
├─ Causa: install-deps-steamos.sh non eseguito
└─ Soluzione: ./install-deps-steamos.sh

Errore: "Wine Flatpak not found"
├─ Causa: Wine non installato
└─ Soluzione: flatpak install flathub org.winehq.Wine

Errore: Font giapponesi non caricati
├─ Causa: Troppi font in Font/ folder (13+ file)
└─ Soluzione:
   cd Font
   rm *.ttf *.ttc *.otf
   # Copia SOLO msgothic.ttc + MS Gothic.ttf

Errore: "Permission denied" al lancio
├─ Causa: File non eseguibili
└─ Soluzione: chmod +x mhfz-launcher install-deps-steamos.sh

Warning: "system.reg not found"
├─ Causa: wineboot lento su SteamOS
└─ Soluzione: Normale, launcher attende 10s e continua


✅ CHECKLIST PRE-COMMIT
═══════════════════════════════════════════════════════════════

Prima di committare su GitHub:

□ lib_linux.rs compila senza errori
□ build-steamos.sh genera archivio .tar.gz
□ Wrapper mhfz-launcher funziona
□ install-deps-steamos.sh testato su SteamOS
□ README.txt ha istruzioni chiare sui font
□ Log ~/mhfz-launcher.log mostra "2 font(s) - OPTIMAL"
□ Gioco lancia e carica font giapponesi


🚀 COMANDO FINALE PER BUILD
═══════════════════════════════════════════════════════════════

# Dalla root del progetto:
./build-steamos.sh

# Output finale:
# ✅ BUILD COMPLETE!
# 📦 Package: MHFZ-Launcher-SteamOS-v1.4.7/
# 📦 Archive: MHFZ-Launcher-SteamOS-v1.4.7.tar.gz


╔══════════════════════════════════════════════════════════════╗
║               TUTTO PRONTO PER BUILD E TEST!                  ║
╚══════════════════════════════════════════════════════════════╝
