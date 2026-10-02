#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd -- "$(dirname -- "$0")/.." && pwd)"
CFG="${XDG_CONFIG_HOME:-$HOME/.config}"
STAMP="$(date +%Y%m%d-%H%M%S)"
backup() { [[ -e "$1" ]] && mv "$1" "$1.backup-$STAMP"; }
backup "$CFG/quickshell/ultimate-shell"
mkdir -p "$CFG/quickshell/ultimate-shell" "$CFG/hypr" "$CFG/kitty" "$CFG/fuzzel" "$CFG/nvim/colors" "$CFG/ultimate-shell/themes"
cp -a "$ROOT/quickshell/." "$CFG/quickshell/ultimate-shell/"
cp "$ROOT/hypr/ultimate-shell.conf" "$CFG/hypr/"
cp "$ROOT/kitty/ultimate-shell.conf" "$CFG/kitty/"
cp "$ROOT/fuzzel/ultimate.ini" "$CFG/fuzzel/"
cp "$ROOT/nvim/colors/ultimate.lua" "$CFG/nvim/colors/"
cp -a "$ROOT/themes/." "$CFG/ultimate-shell/themes/"
cp "$ROOT/scripts/theme-sync" "$CFG/ultimate-shell/theme-sync"
chmod +x "$CFG/ultimate-shell/theme-sync"
printf '%s\n' "Installed Ultimate Quickshell v7." "Run: qs -c ultimate-shell" "Add: source = ~/.config/hypr/ultimate-shell.conf"