# Contributing

Thanks for helping build Ultimate Quickshell.

## Development principles
- Keep the shell modular: UI belongs in quickshell/components, system integration in quickshell/services, and shared tokens in quickshell/config.
- Prefer native Quickshell APIs over external commands when a stable native API exists.
- Avoid hot polling. Prefer models, signals and bindings.
- Keep the shell responsive on low-power hardware and multi-monitor setups.
- Do not copy source code from Caelestia, Impasto, or other projects. Inspiration is welcome; implementation must be original or appropriately licensed.
- Keep themes portable across Quickshell, Kitty, Fuzzel, Zellij and Neovim.

## Pull requests
1. Fork the repository.
2. Create a focused branch: feat/..., fix/..., docs/... or refactor/....
3. Make the smallest coherent change.
4. Test on a real Wayland/Hyprland session when possible.
5. Update documentation when behavior changes.
6. Open a pull request with testing notes and screenshots for visual changes.

## Commit style
Prefer feat:, fix:, refactor:, docs:, perf: and chore:.

New visual work should preserve fluid surfaces, restrained motion, dynamic color, readable typography and low resource overhead.