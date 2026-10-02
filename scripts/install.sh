#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "$0")/.." && pwd)"
CFG="${XDG_CONFIG_HOME:-$HOME/.config}"
BIN="${XDG_BIN_HOME:-$HOME/.local/bin}"
STAMP="$(date +%Y%m%d-%H%M%S)"

backup() { [[ -e "$1" ]] && mv "$1" "$1.backup-$STAMP"; }

mkdir -p "$BIN" "$CFG/systemd/user" "$CFG/quickshell/ultimate-shell"   "$CFG/hypr" "$CFG/kitty" "$CFG/fuzzel" "$CFG/nvim/colors"   "$CFG/ultimate-shell/themes"

# Build the Rust control plane when Cargo is available.
if command -v cargo >/dev/null 2>&1; then
  cargo build --manifest-path "$ROOT/Cargo.toml" --release
  install -Dm755 "$ROOT/target/release/lem0x01d" "$BIN/lem0x01d"
  install -Dm755 "$ROOT/target/release/lem0x01ctl" "$BIN/lem0x01ctl"
fi

backup "$CFG/quickshell/ultimate-shell"
cp -a "$ROOT/quickshell/." "$CFG/quickshell/ultimate-shell/"
cp "$ROOT/hypr/ultimate-shell.conf" "$CFG/hypr/"
cp "$ROOT/kitty/ultimate-shell.conf" "$CFG/kitty/"
cp "$ROOT/fuzzel/ultimate.ini" "$CFG/fuzzel/"
cp "$ROOT/nvim/colors/ultimate.lua" "$CFG/nvim/colors/"
cp -a "$ROOT/themes/." "$CFG/ultimate-shell/themes/"
cp "$ROOT/scripts/theme-sync" "$CFG/ultimate-shell/theme-sync"
cp "$ROOT/config/lem0x01.toml.example" "$CFG/lem0x01.toml.example"
chmod +x "$CFG/ultimate-shell/theme-sync"

install -Dm644 "$ROOT/systemd/lem0x01.service" "$CFG/systemd/user/lem0x01.service"
systemctl --user daemon-reload

if command -v systemctl >/dev/null 2>&1 && command -v lem0x01d >/dev/null 2>&1; then
  systemctl --user enable --now lem0x01.service
fi

printf '%s\n'   "Installed lem0x01 Linux Environment Manager."   "Control plane: $BIN/lem0x01d"   "CLI:           $BIN/lem0x01ctl"   "Frontend:      qs -c ultimate-shell"   "Hyprland:      source ~/.config/hypr/ultimate-shell.conf"
