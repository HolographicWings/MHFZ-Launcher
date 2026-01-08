#!/bin/bash

# ═══════════════════════════════════════════════════════════════════
# MHFZ Launcher - Dependency Installation for SteamOS
# Version: 2.1 (con Wine Flatpak + Font check)
# ═══════════════════════════════════════════════════════════════════

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
WHITE='\033[1;37m'
GRAY='\033[0;90m'
NC='\033[0m'
BOLD='\033[1m'

# Special characters
CHECK="${GREEN}✓${NC}"
CROSS="${RED}✗${NC}"
ARROW="${CYAN}→${NC}"
STAR="${YELLOW}★${NC}"
WARN="${YELLOW}⚠${NC}"

# Banner
clear
echo -e "${CYAN}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${NC} ${BOLD}${WHITE}MHFZ Launcher - SteamOS Dependency Installation v2.1${NC} ${CYAN}║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${WHITE}This script will install:${NC}"
echo -e " ${ARROW} ${WHITE}webkit2gtk (launcher GUI)${NC}"
echo -e " ${ARROW} ${WHITE}Wine Flatpak (game runtime)${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════════
# STEP 1: Password Check
# ═══════════════════════════════════════════════════════════════════

echo -e "${CYAN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${NC} ${BOLD}STEP 1/6${NC} - Administrator Password ${CYAN}║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""

check_password_exists() {
    local password_status=$(sudo passwd -S deck 2>/dev/null | awk '{print $2}')
    [[ "$password_status" == "P" || "$password_status" == "PS" ]]
}

if sudo -n true 2>/dev/null; then
    echo -e "${CHECK} ${GREEN}Sudo already active${NC}"
elif check_password_exists 2>/dev/null; then
    echo -e "${CHECK} ${GREEN}Password configured${NC}"
    echo -e "${BLUE}Enter password:${NC}"
    sudo -v || { echo -e "${CROSS} ${RED}Wrong password${NC}"; exit 1; }
else
    echo -e "${WARN} ${YELLOW}Password not set${NC}"
    echo -e "${WHITE}Set a new password:${NC}"
    passwd || { echo -e "${CROSS} ${RED}Failed${NC}"; exit 1; }
    sudo -v || { echo -e "${CROSS} ${RED}Verification failed${NC}"; exit 1; }
fi

echo ""
read -p "$(echo -e ${CYAN}Press ENTER...${NC})" dummy
clear

# ═══════════════════════════════════════════════════════════════════
# STEP 2: Filesystem Unlock
# ═══════════════════════════════════════════════════════════════════

echo -e "${CYAN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${NC} ${BOLD}STEP 2/6${NC} - Unlock Filesystem ${CYAN}║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}${BOLD}Running: sudo steamos-readonly disable${NC}"
if sudo steamos-readonly disable; then
    echo -e "${CHECK} ${GREEN}Filesystem unlocked${NC}"
else
    echo -e "${CROSS} ${RED}Failed${NC}"
    exit 1
fi

echo ""
read -p "$(echo -e ${CYAN}Press ENTER...${NC})" dummy
clear

# ═══════════════════════════════════════════════════════════════════
# STEP 3: Initialize Keys
# ═══════════════════════════════════════════════════════════════════

echo -e "${CYAN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${NC} ${BOLD}STEP 3/6${NC} - Initialize Package Keys ${CYAN}║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}Initializing keyring...${NC}"
sudo pacman-key --init >/dev/null 2>&1
echo -e "${CHECK} ${GREEN}Done${NC}"

echo -e "${BLUE}Populating Arch keys...${NC}"
sudo pacman-key --populate archlinux >/dev/null 2>&1
echo -e "${CHECK} ${GREEN}Done${NC}"

echo -e "${BLUE}Populating SteamOS keys...${NC}"
sudo pacman-key --populate holo >/dev/null 2>&1
echo -e "${CHECK} ${GREEN}Done${NC}"

echo ""
read -p "$(echo -e ${CYAN}Press ENTER...${NC})" dummy
clear

# ═══════════════════════════════════════════════════════════════════
# STEP 4: Install webkit2gtk
# ═══════════════════════════════════════════════════════════════════

echo -e "${CYAN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${NC} ${BOLD}STEP 4/6${NC} - Install webkit2gtk ${CYAN}║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}${BOLD}Running: sudo pacman -Sy --needed --noconfirm webkit2gtk${NC}"
if sudo pacman -Sy --needed --noconfirm webkit2gtk; then
    echo -e "${CHECK} ${GREEN}webkit2gtk installed${NC}"
else
    echo -e "${CROSS} ${RED}Installation failed${NC}"
    exit 1
fi

# Verifica fontconfig
echo ""
echo -e "${BLUE}Verifying fontconfig...${NC}"
if command -v fc-list >/dev/null 2>&1; then
    echo -e "${CHECK} ${GREEN}fontconfig available${NC}"
else
    echo -e "${WARN} ${YELLOW}fontconfig not found (may be needed)${NC}"
fi

echo ""
read -p "$(echo -e ${CYAN}Press ENTER...${NC})" dummy
clear

# ═══════════════════════════════════════════════════════════════════
# STEP 5: Install Wine Flatpak
# ═══════════════════════════════════════════════════════════════════

echo -e "${CYAN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${NC} ${BOLD}STEP 5/6${NC} - Install Wine Flatpak ${CYAN}║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check se Wine è già installato
if flatpak list | grep -q "org.winehq.Wine"; then
    echo -e "${CHECK} ${GREEN}Wine Flatpak already installed!${NC}"
else
    echo -e "${WHITE}Installing Wine Flatpak (stable-25.08)...${NC}"
    echo -e "${GRAY}This may take 2-3 minutes...${NC}"
    echo ""

    # Installa Wine senza prompt
    if flatpak install -y --system flathub org.winehq.Wine//stable-25.08; then
        echo ""
        echo -e "${CHECK} ${GREEN}Wine Flatpak installed successfully!${NC}"
    else
        echo ""
        echo -e "${CROSS} ${RED}Wine installation failed${NC}"
        echo -e "${YELLOW}You can install manually:${NC}"
        echo -e "${WHITE}flatpak install flathub org.winehq.Wine${NC}"
        # Non blocchiamo l'installazione se Wine fallisce
    fi
fi

echo ""
read -p "$(echo -e ${CYAN}Press ENTER...${NC})" dummy
clear

# ═══════════════════════════════════════════════════════════════════
# STEP 6: Lock Filesystem
# ═══════════════════════════════════════════════════════════════════

echo -e "${CYAN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${NC} ${BOLD}STEP 6/6${NC} - Lock Filesystem ${CYAN}║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}${BOLD}Running: sudo steamos-readonly enable${NC}"
if sudo steamos-readonly enable; then
    echo -e "${CHECK} ${GREEN}Filesystem locked${NC}"
else
    echo -e "${WARN} ${YELLOW}Could not lock - run manually:${NC}"
    echo -e "${WHITE}sudo steamos-readonly enable${NC}"
fi

echo ""
sleep 2
clear

# ═══════════════════════════════════════════════════════════════════
# INSTALLATION COMPLETE
# ═══════════════════════════════════════════════════════════════════

echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║${NC} ${BOLD}${WHITE}✓ INSTALLATION COMPLETED!${NC} ${GREEN}║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${WHITE}${BOLD}Installed:${NC}"
echo -e " ${CHECK} ${WHITE}webkit2gtk (launcher GUI)${NC}"
echo -e " ${CHECK} ${WHITE}Wine Flatpak stable-25.08 (game runtime)${NC}"
echo ""

echo -e "${CYAN}${BOLD}Next steps:${NC}"
echo -e " ${MAGENTA}1.${NC} ${WHITE}Place game files in ${BOLD}/home/deck/MHFZ/${NC}"
echo -e " ${MAGENTA}2.${NC} ${WHITE}Create ${BOLD}Font/${NC} folder inside game directory${NC}"
echo -e " ${MAGENTA}3.${NC} ${WHITE}Copy ${BOLD}msgothic.ttc${NC} and ${BOLD}MS Gothic.ttf${NC} to Font/${NC}"
echo -e " ${MAGENTA}4.${NC} ${WHITE}Run: ${BOLD}./mhfz-launcher${NC}"
echo ""

echo -e "${YELLOW}${BOLD}⚠️  IMPORTANT - Font Configuration:${NC}"
echo -e " ${ARROW} ${WHITE}The launcher requires ${BOLD}ONLY 2 font files${NC}${WHITE}:${NC}"
echo -e "    ${BOLD}msgothic.ttc${NC} and ${BOLD}MS Gothic.ttf${NC}"
echo -e " ${ARROW} ${WHITE}Too many fonts (13+) will ${BOLD}BREAK${NC}${WHITE} Japanese text!${NC}"
echo -e " ${ARROW} ${WHITE}Remove all other fonts (NotoSans, Meiryo, etc.)${NC}"
echo ""

echo -e "${CYAN}${BOLD}How to launch:${NC}"
echo -e " ${MAGENTA}Option A:${NC} ${WHITE}Run: ${BOLD}./mhfz-launcher${NC}"
echo -e " ${MAGENTA}Option B:${NC} ${WHITE}Add to Steam as Non-Steam Game${NC}"
echo ""

echo -e "${CYAN}${BOLD}Technical notes:${NC}"
echo -e " ${ARROW} ${WHITE}Wine prefix: ${BOLD}/home/deck/MHFZ/pfx${NC}"
echo -e " ${ARROW} ${WHITE}First launch: 1-2 minutes (prefix initialization)${NC}"
echo -e " ${ARROW} ${WHITE}Flatpak permissions: auto-configured by launcher${NC}"
echo -e " ${ARROW} ${WHITE}Logs: ${BOLD}~/mhfz-launcher.log${NC}"
echo ""

echo -e "${GRAY}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${STAR} ${YELLOW}Happy hunting in Monster Hunter Frontier Z!${NC} ${STAR}"
echo -e "${GRAY}═══════════════════════════════════════════════════════════════${NC}"
echo ""
