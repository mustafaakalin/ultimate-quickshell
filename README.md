# Ultimate Quickshell v7 — Community Edition

A clean-room Hyprland desktop shell built around Quickshell 0.3.1+, with one shared visual system for Quickshell, Neovim, Kitty, Fuzzel and Zellij.

## Highlights
- Native Quickshell DesktopEntries launcher with ScriptModel filtering.
- Native NetworkManager and BlueZ integrations.
- Native StatusNotifier/SystemTray integration.
- Native UPower, PipeWire, MPRIS and notifications.
- Multi-monitor top bars and a Dynamic Island surface.
- Shared palettes for Quickshell, Neovim, Kitty, Fuzzel and Zellij.
- MIT licensed and contribution-friendly.

## Install
    ./scripts/install.sh
Then add to Hyprland: source = ~/.config/hypr/ultimate-shell.conf
Start with: qs -c ultimate-shell

## Themes
    ~/.config/ultimate-shell/theme-sync caelestia
    ~/.config/ultimate-shell/theme-sync impasto
    ~/.config/ultimate-shell/theme-sync ultimate

## Neovim
    vim.o.termguicolors = true
    vim.cmd.colorscheme('ultimate')
Copy nvim/colors/ultimate.lua into ~/.config/nvim/colors/.

## IPC
    qs ipc call ultimate launcher
    qs ipc call ultimate notifications
    qs ipc call ultimate control
    qs ipc call ultimate media
    qs ipc call ultimate close
    qs ipc call ultimate reloadShell

## Community
This is a public open-source project. Use issues and pull requests for bugs, features, documentation and design improvements. See CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md and GOVERNANCE.md.

## Design
Caelestia and Impasto are visual references only. This project is a clean-room implementation and does not copy their source code.

## Roadmap
- [x] Hyprland / MPRIS / PipeWire / UPower
- [x] NetworkManager / Bluetooth / SystemTray
- [x] Native application launcher
- [x] Shared cross-application themes
- [ ] CAVA spectrum visualizer
- [ ] Wallpaper-driven Material/OKLCH palette generation
- [ ] OSD
- [ ] Clipboard history
- [ ] Lock screen / idle management
- [ ] Screenshot and color-picker surfaces
- [ ] Calendar and system monitor
- [ ] More community themes

Quickshell provides native DesktopEntries, Networking, Bluetooth and SystemTray APIs used by this project.