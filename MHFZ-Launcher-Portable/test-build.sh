#!/bin/bash

# ═══════════════════════════════════════════════════════════════════
# MHFZ Launcher - Pre-Build Test Script
# ═══════════════════════════════════════════════════════════════════

set -e

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║       MHFZ Launcher - Pre-Build Verification Tests          ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

FAILED=0
PASSED=0

# Test 1: Verifica esistenza file
echo "🔍 [TEST 1/6] Checking required files..."
FILES=(
    "src-tauri/src/lib_linux.rs"
    "MHFZ-Launcher-Portable/install-deps-steamos.sh"
    "MHFZ-Launcher-Portable/README.txt"
    "build-steamos.sh"
)

for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "  ✅ $file"
        ((PASSED++))
    else
        echo "  ❌ $file NOT FOUND"
        ((FAILED++))
    fi
done
echo ""

# Test 2: Verifica funzioni in lib_linux.rs
echo "🔍 [TEST 2/6] Checking lib_linux.rs functions..."
if grep -q "fn configure_flatpak_permissions" src-tauri/src/lib_linux.rs; then
    echo "  ✅ configure_flatpak_permissions() found"
    ((PASSED++))
else
    echo "  ❌ configure_flatpak_permissions() NOT FOUND"
    ((FAILED++))
fi

if grep -q "CRITICO: PULISCI directory Fonts" src-tauri/src/lib_linux.rs; then
    echo "  ✅ Font cleaning logic found"
    ((PASSED++))
else
    echo "  ❌ Font cleaning logic NOT FOUND"
    ((FAILED++))
fi

if grep -q "max 2 fonts" src-tauri/src/lib_linux.rs; then
    echo "  ✅ Font limit (max 2) found"
    ((PASSED++))
else
    echo "  ❌ Font limit logic NOT FOUND"
    ((FAILED++))
fi

if grep -q "FIX CRITICO: Passa --env=WINEPREFIX" src-tauri/src/lib_linux.rs; then
    echo "  ✅ --env=WINEPREFIX fix found"
    ((PASSED++))
else
    echo "  ❌ --env=WINEPREFIX fix NOT FOUND"
    ((FAILED++))
fi
echo ""

# Test 3: Verifica install-deps-steamos.sh
echo "🔍 [TEST 3/6] Checking install-deps-steamos.sh..."
if grep -q "org.winehq.Wine" MHFZ-Launcher-Portable/install-deps-steamos.sh; then
    echo "  ✅ Wine Flatpak installation found"
    ((PASSED++))
else
    echo "  ❌ Wine Flatpak installation NOT FOUND"
    ((FAILED++))
fi

if grep -q "IMPORTANT - Font Configuration" MHFZ-Launcher-Portable/install-deps-steamos.sh; then
    echo "  ✅ Font configuration warning found"
    ((PASSED++))
else
    echo "  ❌ Font configuration warning NOT FOUND"
    ((FAILED++))
fi
echo ""

# Test 4: Verifica README.txt
echo "🔍 [TEST 4/6] Checking README.txt..."
if grep -q "CRITICAL - FONT CONFIGURATION" MHFZ-Launcher-Portable/README.txt; then
    echo "  ✅ Critical font section found"
    ((PASSED++))
else
    echo "  ❌ Critical font section NOT FOUND"
    ((FAILED++))
fi

if grep -q "msgothic.ttc" MHFZ-Launcher-Portable/README.txt; then
    echo "  ✅ Font file reference found"
    ((PASSED++))
else
    echo "  ❌ Font file reference NOT FOUND"
    ((FAILED++))
fi
echo ""

# Test 5: Sintassi Rust
echo "🔍 [TEST 5/6] Checking Rust syntax..."
if command -v cargo >/dev/null 2>&1; then
    cd src-tauri
    if cargo check --quiet 2>/dev/null; then
        echo "  ✅ Rust code compiles without errors"
        ((PASSED++))
    else
        echo "  ❌ Rust compilation errors detected"
        echo "  Run: cd src-tauri && cargo check"
        ((FAILED++))
    fi
    cd ..
else
    echo "  ⚠️  cargo not found, skipping Rust syntax check"
fi
echo ""

# Test 6: Permessi eseguibili
echo "🔍 [TEST 6/6] Checking executable permissions..."
if [ -x "build-steamos.sh" ]; then
    echo "  ✅ build-steamos.sh is executable"
    ((PASSED++))
else
    echo "  ⚠️  build-steamos.sh not executable (fixing...)"
    chmod +x build-steamos.sh
    ((PASSED++))
fi

if [ -x "MHFZ-Launcher-Portable/install-deps-steamos.sh" ]; then
    echo "  ✅ install-deps-steamos.sh is executable"
    ((PASSED++))
else
    echo "  ⚠️  install-deps-steamos.sh not executable (fixing...)"
    chmod +x MHFZ-Launcher-Portable/install-deps-steamos.sh
    ((PASSED++))
fi
echo ""

# Risultati
echo "╔══════════════════════════════════════════════════════════════╗"
if [ $FAILED -eq 0 ]; then
    echo "║  ✅ ALL TESTS PASSED ($PASSED/$((PASSED+FAILED)))                                     ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo ""
    echo "🚀 Ready to build! Run:"
    echo "   ./build-steamos.sh"
    echo ""
    exit 0
else
    echo "║  ❌ SOME TESTS FAILED ($PASSED passed, $FAILED failed)                  ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo ""
    echo "⚠️  Fix errors before building"
    echo ""
    exit 1
fi
