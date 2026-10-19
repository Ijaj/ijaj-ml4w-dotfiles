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
| `tools/promptmap/` | Rust tool that re-colors any oh-my-posh theme (stock `shipped:*` and `edm115`) from the wallpaper palette. Built and linked by `ml4w-custom-sync`. |
| `ml4w-themes/{blueprint,riso,transit,rackmount,receipt,sumi}/` (+ `-restore`) | 6 more global themes, each with its own waybar theme, notifications, logout screen, dock, rofi style, window look and matching prompt. See "New design languages" below. |
| `tools/live/`, `bin/ml4w-live` | Reactive themes: **strata**, **vital**, **solar**, **modal** (small daemons). See "Live themes". |
| `bin/ml4w-texture` | Generates palette-colored SVG textures (grid, halftone, paper, brushed, topo) used by the themes above |
| `ml4w-themes/custom-lib.sh` | Shared apply/backup/restore code for the custom global themes |
| `swaync-themes/`, `wlogout-themes/`, `dock-themes/` | Soft Glass parts. swaync and wlogout import ML4W's glass theme and only override shape and transparency. |
| `hypr-decorations/`, `hypr-windows/` | Window look for each theme (Lua config) |
| `zshrc/01-envs` | Environment variables (Android SDK, Flutter/fvm, Chrome) |
| `zshrc/26-functions` | My functions (`ccc`, `startup`, `nn`, `ng`), nvm, LM Studio path |
| `zshrc/custom/20-customization` | Overrides the ML4W file of the same name (oh-my-posh prompt choice) |
| `bin/ml4w-custom-sync` | Puts everything above into place. Only touches what is missing or outdated. |
| `systemd/` | User units that run the sync at login and whenever ML4W replaces its folders |
| `install.sh` | One-shot setup: links the tools into `~/.local/bin`, installs and enables the systemd units (sync, prompt/texture refresh, live themes autostart) and runs the sync |

## Requirements

These themes are built for a specific stack. Check it before installing:

| Needed | Why |
|---|---|
| [ML4W dotfiles](https://github.com/mylinuxforwork/dotfiles), installed and working | Everything here is applied on top of it (`install.sh` stops if it is missing) |
| Hyprland **0.55 or newer with the Lua config** (tested on 0.56.2) | Window looks are `.lua` files and live themes call `hyprctl eval`. The old `hyprland.conf` format and `hyprctl keyword` are not used |
| `waybar`, `swaync`, `rofi` (2.x), `wlogout`, `nwg-dock-hyprland` | The parts being themed (all part of ML4W) |
| `python3`, `jq`, `fzf`, `oh-my-posh` | The `ml4w-*` tools, prompts, pickers |
| Fonts: `FiraCode Nerd Font`, `Fira Sans`, `Noto Sans/Serif CJK JP` | Type for the themes. Sumi needs Noto CJK for its kanji numerals and 印 stamp |
| `grim` (optional) | Only for taking screenshots while developing |
| `hyprsunset` (optional) | `ml4w-live` `solar` |
| `cargo` (optional) | Builds `promptmap`, which re-colors stock oh-my-posh themes |

On Arch, most of it is one line (names may differ slightly in the AUR): `sudo pacman -S python jq fzf rofi waybar wlogout otf-firacode-nerd ttf-fira-sans noto-fonts-cjk hyprsunset rust`.

## Setup after reinstalling Arch

1. Install Arch and the ML4W dotfiles as usual.
2. Clone and install:
   ```sh
   git clone https://github.com/Ijaj/ijaj-ml4w-dotfiles.git ~/ijaj-ml4w-dotfiles
   cd ~/ijaj-ml4w-dotfiles
   ./install.sh
   ```
3. Pick a Waybar theme with `SUPER+CTRL+T` and open a new terminal.
4. Try the new themes: see the quick start below.

Keep the repo at `~/ijaj-ml4w-dotfiles` (or re-run `./install.sh` after moving it), since files are linked from it.

## Global themes (Soft Glass, Flat, Cyber, Retro, Paper, Pastel, Brutal)

Open the ML4W theme picker with `CTRL+ALT+T` and choose `softglass`, `flat`, `cyber`, `retro`, `paper`, `pastel` or `brutal`. To go back, choose the matching `-restore` entry.
(`SUPER+CTRL+T` is a different picker: it only lists Waybar themes.)
The settings from before are saved to `~/.local/state/ml4w-custom/pre-<theme>/` when you apply a theme. Re-applying a theme that is already active keeps that backup.

To add another global theme: copy the `flat` entries in each folder, change the look, and write a small `theme.sh` that calls `cc_backup` and `cc_apply` from `custom-lib.sh`.

Known limit: blur behind swaync notification popups does not show on this setup, so those cards use a mostly opaque background instead.

## New design languages

Same picker (`CTRL+ALT+T`), same `-restore` entries. Colors always come from the wallpaper.

| Theme | Idea |
|---|---|
| `blueprint` | Technical drawing. Sheet-colored bar with dashed construction lines, `[1] [2]` workspaces, `SHEET 12:00 · date` clock, grid texture, `REV.A ▸ FIND` launcher, `FIG.` prompt. |
| `riso` | Risograph print. Chips with two misregistered ink shadows (tertiary + secondary), hard offset window shadow, halftone texture, two-ink gradient borders. |
| `transit` | Metro map. Workspaces are stations on a line, the active one is the interchange ring. Station-sign cards and launcher, prompt as stops on a line. |
| `rackmount` | 1U hardware panel. Brushed-metal strip, a status LED per module (CSS radial gradients), screws at both ends, LCD clock, patch-panel launcher. |
| `receipt` | Thermal printout. Inverse-surface "paper" everywhere, dotted tear lines, mono caps, narrow launcher, line-item prompt. |
| `sumi` | Ink wash. Floating transparent bar, kanji numerals with a 印 stamp on the active workspace, wide gaps, no borders, inactive windows dimmed, slow fades, paper texture. |

`ml4w-rofi topo` is an extra launcher style: a generated contour map behind the list.

### Quick start: try the new themes

1. Press `CTRL+ALT+T` and choose one of `blueprint`, `riso`, `transit`, `rackmount`, `receipt`, `sumi` (or `softglass`, `flat`, ...). It switches the bar, notifications, logout screen, dock, launcher and window look together.
2. Choose the matching `-restore` entry (for example `blueprint-restore`) to go back to what you had.
3. Match your shell prompt with `prompt blueprint` (same names), and try `rofitheme topo` for the contour-map launcher.
4. Turn on a live theme: `ml4w-live on strata` (or `vital`, `solar`, `modal`), or press `SUPER+CTRL+SHIFT+L` for a picker. `ml4w-live status` shows what runs. `modal` only shows something while you are in a mode: press `SUPER+ALT+R` (resize) or `SUPER+ALT+M` (move), `Esc` leaves.
5. `vital --bar` needs a Waybar side: run `ml4w-waybar-side top` once (see the section below).

If the bar looks wrong after switching, or something fails, run `ml4w-custom-sync` once and check `hyprctl configerrors`.

## Live themes (reactive)

`ml4w-live` (alias `live`, `SUPER+CTRL+SHIFT+L`) turns small daemons on or off. Enabled ones start at login (`ml4w-live.service`), and turning one off restores the compositor state.

| Name | What it does |
|---|---|
| `strata` | Focus-recency depth. The focused window is crisp; older ones step down in opacity and border tone along the palette. |
| `vital` | Accent follows load: primary when calm, tertiary when busy, error when hot (CPU load, trusted temperature sensors, low battery). Recolors the border and any colored shadow. `ml4w-live on vital --bar` also tints the Waybar edge line (needs `ml4w-waybar-side` on a side, not `default`; the bar reloads at most twice a minute). |
| `solar` | Screen temperature and gamma glide with the time of day through hyprsunset (starts it if it is not running). Optional `~/.config/ml4w/settings/solar.conf`: `sunrise=06:00 sunset=18:00 day_k=6500 night_k=3600 night_gamma=88 fade_minutes=90`. |
| `modal` | The border shows the active Hyprland submap. Adds `SUPER+ALT+R` (resize mode) and `SUPER+ALT+M` (move mode) with arrows or hjkl, `Esc`/`Return` leaves. While a mode is active the border belongs to modal, so vital pauses. |

Commands: `ml4w-live` (pick), `on|off|toggle <name>`, `status`.

## Textures

`ml4w-texture refresh` writes SVGs to `~/.local/share/ml4w-custom/textures/` in the current palette. The palette-change unit (`ml4w-prompt-refresh.path`) runs it after a wallpaper change, and the sync links the folder as `~/.config/swaync/textures` and `~/.config/wlogout/textures` so CSS can use `url("../../textures/grid.svg")`. Rofi cannot tile backgrounds, so it uses the `-wide` variants (stretched with `both`).

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
| `prompt` | Pick a terminal prompt with fzf and a live preview. `prompt <name>`, `prompt list`, `prompt random`, `prompt edm115` (your original). `shipped:<name>` uses oh-my-posh's own themes, re-colored from the wallpaper by `promptmap`. Applies to open terminals at the next prompt. |
| `toast` | Pick a notification popup style (rofi menu, or fzf in a terminal). `toast <name>`, `toast list`, `toast test`. |
| `rofitheme` | Pick a launcher style; it opens the launcher to show it. `rofitheme default` goes back to ML4W's look. |
| `SUPER+CTRL+T` | Waybar themes. `CTRL+ALT+T`: global themes. |
| `SUPER+CTRL+SHIFT+P` / `N` / `R` | Prompt theme (opens a kitty with fzf) / toast style / rofi style. Defined in `hypr-custom/`. |

(`prompt`, `toast` and `rofitheme` are aliases for `ml4w-prompt`, `ml4w-toast`, `ml4w-rofi`.)

Prompt colors follow the wallpaper: `ml4w-prompt-refresh.path` watches the palette file Matugen rewrites and rebuilds `~/.config/ohmyposh/active.omp.json` with the new colors. Prompt themes use palette names (`p:primary`, `p:surface_container`, ...), never fixed colors. Stock themes go through `promptmap`: greys become palette neutrals, colors become accent slots (`pm0`, `pm1`, ...) whose hue is rotated onto the wallpaper's primary hue (reds onto the error hue), with lightness kept and fg/bg contrast re-checked on every palette change (about 2 ms).
Terminal (kitty) colors already follow the wallpaper through Matugen's `colors-matugen.conf`.

Rofi: `ml4w-custom-sync` makes sure `~/.config/rofi/config.rasi` imports `~/.config/ml4w/settings/rofi-theme.rasi`, which `ml4w-rofi` writes. Rofi has no alpha-on-theme-color syntax and no `height: auto`, so styles use solid theme colors.

Applying a global theme also sets the rofi style of the same name, and its `-restore` entry puts back the previous one. `rofitheme` still works afterwards for a one-off change.

## Moving the bar to any edge

`ml4w-waybar-side` (alias for the menu: run it with no arguments) puts every Waybar theme on `top`, `bottom`, `left` or `right`; `default` removes it. It uses ML4W's own `config-custom` / `style-custom.css` hooks, so no ML4W file is edited. Left/right drop the wide modules (window title, quicklinks, hardware/tools groups, bluetooth, updates) and show a stacked clock. Edge borders and chip spacing are flipped automatically, but themes drawn for a horizontal bar can look a bit tight vertically.
If you change a module toggle in the ML4W settings app, the sync unit regenerates the overrides (the bar restarts once).

## Uninstall

1. Go back to your old look: apply the `-restore` entry of the global theme you used (`CTRL+ALT+T`), and `ml4w-rofi default`.
2. Stop the live themes and the units:
   ```sh
   ml4w-live off-all; rm -f ~/.config/ml4w/settings/live-enabled
   systemctl --user disable --now ml4w-custom-sync.path ml4w-prompt-refresh.path ml4w-live.service ml4w-custom-sync.service
   rm ~/.config/systemd/user/ml4w-{custom-sync,prompt-refresh,live}.{path,service} 2>/dev/null; systemctl --user daemon-reload
   ```
3. Remove the tools: `rm ~/.local/bin/ml4w-{custom-sync,prompt,toast,rofi,waybar-side,live,texture} ~/.local/bin/promptmap`, the `require("ml4w-custom-binds")` line in `~/.config/hypr/custom.lua`, and the link `~/.config/hypr/ml4w-custom-binds.lua`.
4. Delete the theme copies the sync made: the folders from this repo inside `~/.config/waybar/themes/`, `~/.config/swaync/themes/`, `~/.config/wlogout/themes/`, `~/.config/nwg-dock-hyprland/themes/`, `~/.config/ml4w/themes/`, `~/.config/hypr/conf/{decorations,windows}/`, plus `~/.config/{swaync,wlogout}/textures` and `~/.local/share/ml4w-custom/`.
5. Optionally remove the repo clone. The zsh files are symlinks into it, so restore the originals from `~/.local/state/ml4w-custom/backups/` first.
