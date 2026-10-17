# ijaj-ml4w-dotfiles

My personal customizations on top of [ML4W dotfiles](https://github.com/mylinuxforwork/dotfiles) for Arch Linux.
They are kept outside the ML4W folder and re-applied automatically, so ML4W updates and system updates don't wipe them.

## What's inside

| Path | What |
|---|---|
| `waybar-themes/` | Custom Waybar themes: Islands, Hairline, Neon, Capsule (2 variants each), Soft Glass, Flat. Colors follow the wallpaper (Matugen). |
| `ml4w-themes/softglass/` | **Soft Glass**: a global ML4W theme (shows in the ML4W theme picker). Sets bar, notifications, logout screen, dock, launcher and window look in one go. |
| `ml4w-themes/softglass-restore/` | Undoes Soft Glass, back to whatever was set before |
| `ml4w-themes/flat/`, `flat-restore/` | **Flat**: solid and minimal. No blur or shadows, 6px corners, thin 1px lines, muted text with the accent only on the active workspace. |
| `ml4w-themes/cyber/`, `cyber-restore/` | **Cyber**: neon glow. Glowing accent outlines on bar, notifications, logout, dock and window shadow/borders. Accent follows the wallpaper. |
| `ml4w-themes/retro/`, `paper/`, `pastel/`, `brutal/` (+ `-restore`) | **Retro**: terminal look, square, mono, accent text. **Paper**: calm, low contrast, soft shadows, serif clock. **Pastel**: floating pill bar, big radius, tinted chips. **Brutal**: thick borders, hard offset shadows, bold mono. All colors follow the wallpaper. |
| `waybar-themes/custom-{underline,gradient,bottom,mac,blocks,tmux,sidebar,aurora}` | 8 more Waybar themes, each with 3 accent variants (primary, secondary, tertiary). |
| `swaync-themes/{accentbar,bubble,compact,outline,solid,terminal,gradient,elevated,banner,ghost}` | Notification toast styles. Switch with `ml4w-toast`. |
| `rofi-themes/` | Launcher styles: one per global theme (softglass, flat, cyber, retro, paper, pastel, brutal) plus spotlight, grid, fullscreen, sidebar, bar, terminal, minimal. Switch with `ml4w-rofi`. |
| `prompts/` | 15 oh-my-posh prompt themes (palette from the wallpaper). Switch with `ml4w-prompt` (alias `prompt`). |
| `ml4w-themes/custom-lib.sh` | Shared apply/backup/restore code for the custom global themes |
| `swaync-themes/`, `wlogout-themes/`, `dock-themes/` | Soft Glass parts. swaync and wlogout import ML4W's glass theme and only override shape and transparency. |
| `hypr-decorations/`, `hypr-windows/` | Window look for each theme (Lua config) |
| `zshrc/01-envs` | Environment variables (Android SDK, Flutter/fvm, Chrome) |
| `zshrc/26-functions` | My functions (`ccc`, `startup`, `nn`, `ng`), nvm, LM Studio path |
| `zshrc/custom/20-customization` | Overrides the ML4W file of the same name (oh-my-posh prompt choice) |
| `bin/ml4w-custom-sync` | Puts everything above into place. Only touches what is missing or outdated. |
| `systemd/` | User units that run the sync at login and whenever ML4W replaces its folders |
| `install.sh` | One-shot setup |

## Setup after reinstalling Arch

1. Install Arch and the ML4W dotfiles as usual.
2. Clone and install:
   ```sh
   git clone https://github.com/Ijaj/ijaj-ml4w-dotfiles.git ~/ijaj-ml4w-dotfiles
   cd ~/ijaj-ml4w-dotfiles
   ./install.sh
   ```
3. Pick a Waybar theme with `SUPER+CTRL+T` and open a new terminal.

Keep the repo at `~/ijaj-ml4w-dotfiles` (or re-run `./install.sh` after moving it), since files are linked from it.

## Global themes (Soft Glass, Flat, Cyber, Retro, Paper, Pastel, Brutal)

Open the ML4W theme picker with `CTRL+ALT+T` and choose `softglass`, `flat`, `cyber`, `retro`, `paper`, `pastel` or `brutal`. To go back, choose the matching `-restore` entry.
(`SUPER+CTRL+T` is a different picker: it only lists Waybar themes.)
The settings from before are saved to `~/.local/state/ml4w-custom/pre-<theme>/` when you apply a theme. Re-applying a theme that is already active keeps that backup.

To add another global theme: copy the `flat` entries in each folder, change the look, and write a small `theme.sh` that calls `cc_backup` and `cc_apply` from `custom-lib.sh`.

Known limit: blur behind swaync notification popups does not show on this setup, so those cards use a mostly opaque background instead.

## Making changes

- **zsh files**: edit in the repo or in `~/.config/zshrc/` (those are symlinks to the repo, same file).
- **New zsh file**: add it to `zshrc/` (or `zshrc/custom/<ml4w-file-name>` to override an ML4W file), then run `ml4w-custom-sync`.
- **Waybar and other themes**: edit in the repo (`waybar-themes/`, `swaync-themes/`, ...), not in the copies under `~/.config`, then run `ml4w-custom-sync`.
  A variant's colors are in its `accent.css`; the layout is in the theme's `style.css`.
- Save: `git add -A && git commit -m "..." && git push` (or the `ccc "message"` function).

## How it works

- Waybar themes are **copied** into `~/.config/waybar/themes/`, because the ML4W theme switcher ignores symlinked folders.
- zsh files are **symlinked** into `~/.config/zshrc/`. Files that were already there get moved to `~/.local/state/ml4w-custom/backups/`.
- `ml4w-custom-sync.path` watches the ML4W folders. When an update replaces them, the sync runs again within a second.

## Notes

- `zshrc/custom/20-customization` is a full copy of the ML4W file. If ML4W changes theirs a lot, re-copy it and re-apply the prompt change.
- If an ML4W update resets the selected Waybar theme, pick it again with `SUPER+CTRL+T`.

## Switching styles (no file editing)

| Command | What |
|---|---|
| `prompt` | Pick a terminal prompt with fzf and a live preview. `prompt <name>`, `prompt list`, `prompt random`, `prompt edm115` (your original). `shipped:<name>` uses oh-my-posh's own themes (fixed colors). Applies to open terminals at the next prompt. |
| `toast` | Pick a notification popup style (rofi menu, or fzf in a terminal). `toast <name>`, `toast list`, `toast test`. |
| `rofitheme` | Pick a launcher style; it opens the launcher to show it. `rofitheme default` goes back to ML4W's look. |
| `SUPER+CTRL+T` | Waybar themes. `CTRL+ALT+T`: global themes. |

(`prompt`, `toast` and `rofitheme` are aliases for `ml4w-prompt`, `ml4w-toast`, `ml4w-rofi`.)

Prompt colors follow the wallpaper: `ml4w-prompt-refresh.path` watches the palette file Matugen rewrites and rebuilds `~/.config/ohmyposh/active.omp.json` with the new colors. Prompt themes use palette names (`p:primary`, `p:surface_container`, ...), never fixed colors.
Terminal (kitty) colors already follow the wallpaper through Matugen's `colors-matugen.conf`.

Rofi: `ml4w-custom-sync` makes sure `~/.config/rofi/config.rasi` imports `~/.config/ml4w/settings/rofi-theme.rasi`, which `ml4w-rofi` writes. Rofi has no alpha-on-theme-color syntax and no `height: auto`, so styles use solid theme colors.

Applying a global theme also sets the rofi style of the same name, and its `-restore` entry puts back the previous one. `rofitheme` still works afterwards for a one-off change.

## Moving the bar to any edge

`ml4w-waybar-side` (alias for the menu: run it with no arguments) puts every Waybar theme on `top`, `bottom`, `left` or `right`; `default` removes it. It uses ML4W's own `config-custom` / `style-custom.css` hooks, so no ML4W file is edited. Left/right drop the wide modules (window title, quicklinks, hardware/tools groups, bluetooth, updates) and show a stacked clock. Edge borders and chip spacing are flipped automatically, but themes drawn for a horizontal bar can look a bit tight vertically.
If you change a module toggle in the ML4W settings app, the sync unit regenerates the overrides (the bar restarts once).
