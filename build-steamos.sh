#!/bin/bash

set -e

VERSION="1.4.7"
BUILD_NAME="MHFZ-Launcher-SteamOS-v${VERSION}"
OUTPUT_DIR="${BUILD_NAME}"

echo "══════════════════════════════════════════════════════════"
echo "  MHFZ Launcher - Portable Build for SteamOS"
echo "  Version: ${VERSION}"
echo "══════════════════════════════════════════════════════════"
echo ""

# 1. Verifica dipendenze
echo "🔍 [1/5] Checking dependencies..."
command -v node >/dev/null 2>&1 || { echo "❌ Node.js required"; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "❌ npm required"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo required"; exit 1; }
echo "✅ Dependencies OK"

# 2. Clean
echo "🧹 [2/5] Cleaning previous builds..."
rm -rf src-tauri/target/release/bundle
rm -rf dist
rm -rf "$OUTPUT_DIR"
rm -f "${BUILD_NAME}.tar.gz"
echo "✅ Clean complete"

# 3. Build frontend
echo "🎨 [3/5] Building frontend..."
npm install
npm run build
echo "✅ Frontend built"

# 4. Build Rust binary
echo "⚙️ [4/5] Building Rust binary..."
cd src-tauri
cargo build --release --features custom-protocol
cd ..

BINARY_PATH="src-tauri/target/release/app"
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ ERROR: Binary not found"
    exit 1
fi

BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
echo "✅ Binary size: $BINARY_SIZE"

# 5. Create portable package
echo "📦 [5/5] Creating portable package..."
mkdir -p "$OUTPUT_DIR"

# ✅ CREA IL WRAPPER con fontconfig
cat > "$OUTPUT_DIR/mhfz-launcher" << 'WRAPPER_EOF'
#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ✅ Check binario esiste
if [ ! -f "$SCRIPT_DIR/mhfz-launcher.bin" ]; then
    echo "❌ ERROR: mhfz-launcher.bin not found!"
    exit 1
fi

# ✅ Forza fontconfig + X11
export FONTCONFIG_PATH="${FONTCONFIG_PATH:-/etc/fonts}"
export FONTCONFIG_FILE="${FONTCONFIG_FILE:-/etc/fonts/fonts.conf}"
export XDG_DATA_DIRS="${XDG_DATA_DIRS:-/usr/share:/usr/local/share}"
export XAUTHORITY="${XAUTHORITY:-$HOME/.Xauthority}"
export WEBKIT_DISABLE_COMPOSITING_MODE=1
export WEBKIT_DISABLE_DMABUF_RENDERER=1

exec "$SCRIPT_DIR/mhfz-launcher.bin" "$@"
WRAPPER_EOF

chmod +x "$OUTPUT_DIR/mhfz-launcher"

# Copia il binario
cp "$BINARY_PATH" "$OUTPUT_DIR/mhfz-launcher.bin"
chmod +x "$OUTPUT_DIR/mhfz-launcher.bin"

# ✅ COPIA install-deps-steamos.sh SE ESISTE
if [ -f "MHFZ-Launcher-Portable/install-deps-steamos.sh" ]; then
    cp "MHFZ-Launcher-Portable/install-deps-steamos.sh" "$OUTPUT_DIR/"
    chmod +x "$OUTPUT_DIR/install-deps-steamos.sh"
    echo "✅ Copied install-deps-steamos.sh"
else
    echo "⚠️  install-deps-steamos.sh not found, skipping"
fi

# ✅ COPIA README.txt SE ESISTE
if [ -f "MHFZ-Launcher-Portable/README.txt" ]; then
    cp "MHFZ-Launcher-Portable/README.txt" "$OUTPUT_DIR/"
    echo "✅ Copied README.txt"
else
    # Crea README di default
    cat > "$OUTPUT_DIR/README.txt" << 'EOF'
MHFZ Launcher - Portable SteamOS Build
=======================================

Installation:
1. Extract this folder anywhere
2. Run: ./install-deps-steamos.sh (first time only)
3. Place game files in the same directory as mhfz-launcher
4. Run: ./mhfz-launcher

The launcher will:
- Auto-create Wine prefix on first launch
- Install Japanese fonts from fonts/ folder
- Configure environment for optimal compatibility

Requirements:
- SteamOS 3.x or compatible Linux
- Wine Flatpak (org.winehq.Wine) or system Wine
- fontconfig

Troubleshooting:
- Check logs: ~/mhfz-launcher.log
- Font issues: Ensure fonts/ folder exists with .ttc/.ttf files
EOF
    echo "✅ Created default README.txt"
fi

# ✅ COPIA ICONA SE ESISTE
if [ -f src-tauri/icons/128x128.png ]; then
    cp src-tauri/icons/128x128.png "$OUTPUT_DIR/mhfz-launcher.png"
    echo "✅ Copied icon"
elif [ -f src-tauri/icons/icon.png ]; then
    cp src-tauri/icons/icon.png "$OUTPUT_DIR/mhfz-launcher.png"
    echo "✅ Copied icon"
fi

# ✅ CREA SCRIPT DI TEST
cat > "$OUTPUT_DIR/test-steamos.sh" << 'EOF'
#!/bin/bash

echo "═══════════════════════════════════════════════════"
echo "  MHFZ Launcher - SteamOS Test Script"
echo "═══════════════════════════════════════════════════"
echo ""

# Test 1: Verifica sistema
echo "🔍 [1/5] Checking system..."
if [ -f /etc/os-release ]; then
    echo "System info:"
    grep "ID=" /etc/os-release | head -2
else
    echo "⚠️  /etc/os-release not found"
fi
echo ""

# Test 2: Verifica Wine
echo "🔍 [2/5] Checking Wine..."
if command -v flatpak >/dev/null 2>&1; then
    if flatpak list | grep -q "org.winehq.Wine"; then
        echo "✅ Wine Flatpak found"
    else
        echo "⚠️  Wine Flatpak NOT installed"
        echo "   Run: ./install-deps-steamos.sh"
    fi
else
    echo "⚠️  Flatpak not found"
fi

if command -v wine >/dev/null 2>&1; then
    echo "✅ System Wine found: $(wine --version 2>/dev/null || echo 'error')"
else
    echo "⚠️  System Wine not found"
fi
echo ""

# Test 3: Verifica dipendenze binario
echo "🔍 [3/5] Checking binary dependencies..."
if command -v ldd >/dev/null 2>&1; then
    ldd ./mhfz-launcher.bin | grep "not found" && echo "❌ Missing libraries!" || echo "✅ All libraries found"
else
    echo "⚠️  ldd command not available"
fi
echo ""

# Test 4: Verifica fontconfig
echo "🔍 [4/5] Checking fontconfig..."
if [ -f /etc/fonts/fonts.conf ]; then
    echo "✅ fonts.conf exists"
else
    echo "❌ fonts.conf NOT FOUND"
fi

if command -v fc-list >/dev/null 2>&1; then
    FONT_COUNT=$(fc-list | wc -l)
    echo "✅ fc-list found ($FONT_COUNT fonts)"
else
    echo "⚠️  fc-list not found"
fi
echo ""

# Test 5: Lancia launcher
echo "🚀 [5/5] Launching MHFZ Launcher..."
echo "Log file: ~/mhfz-launcher.log"
echo ""
./mhfz-launcher

# Mostra log
echo ""
echo "📋 Debug log (last 30 lines):"
if [ -f ~/mhfz-launcher.log ]; then
    tail -30 ~/mhfz-launcher.log
else
    echo "⚠️  Log file not found"
fi

echo ""
echo "═══════════════════════════════════════════════════"
echo "  Test completed!"
echo "  Full log: ~/mhfz-launcher.log"
echo "═══════════════════════════════════════════════════"
EOF

chmod +x "$OUTPUT_DIR/test-steamos.sh"
echo "✅ Created test-steamos.sh"

# ✅ CREA ARCHIVIO TAR.GZ
echo ""
echo "📦 Creating archive..."
tar -czf "${BUILD_NAME}.tar.gz" "$OUTPUT_DIR"
ARCHIVE_SIZE=$(du -h "${BUILD_NAME}.tar.gz" | cut -f1)

echo ""
echo "══════════════════════════════════════════════════════════"
echo "  ✅ BUILD COMPLETE!"
echo "══════════════════════════════════════════════════════════"
echo "📦 Package: ${OUTPUT_DIR}/"
echo "📦 Archive: ${BUILD_NAME}.tar.gz"
echo "📏 Binary size: $BINARY_SIZE"
echo "📏 Archive size: $ARCHIVE_SIZE"
echo ""
echo "📋 Package contents:"
ls -lh "$OUTPUT_DIR/" | awk '{print "   " $9 " (" $5 ")"}'
echo ""
echo "🚀 To deploy on SteamOS:"
echo "   1. Copy archive:"
echo "      scp ${BUILD_NAME}.tar.gz deck@<IP>:/home/deck/"
echo ""
echo "   2. On Steam Deck:"
echo "      tar -xzf ${BUILD_NAME}.tar.gz"
echo "      cd ${BUILD_NAME}"
echo "      ./install-deps-steamos.sh    # First time only"
echo "      ./test-steamos.sh            # Run tests"
echo "      ./mhfz-launcher              # Launch normally"
echo ""
echo "   3. Check logs:"
echo "      cat ~/mhfz-launcher.log"
echo "══════════════════════════════════════════════════════════"
