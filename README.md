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

## Global themes (Soft Glass, Flat)

Open the ML4W theme picker with `CTRL+ALT+T` and choose `softglass` or `flat`. To go back, choose `softglass-restore` or `flat-restore`.
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
