#!/usr/bin/env bash
# Sets up the customizations from this repo. Run AFTER installing ML4W dotfiles.
# Safe to re-run.
set -e
repo="$(cd "$(dirname "$0")" && pwd)"

if [ ! -d "$HOME/.config/waybar/themes" ] || [ ! -d "$HOME/.config/zshrc" ]; then
    echo "ML4W dotfiles not found. Install ML4W first, then run this again."
    exit 1
fi

mkdir -p "$HOME/.local/bin" "$HOME/.config/systemd/user"
ln -sf "$repo/bin/ml4w-custom-sync" "$HOME/.local/bin/ml4w-custom-sync"
cp "$repo"/systemd/ml4w-custom-sync.* "$HOME/.config/systemd/user/"

systemctl --user daemon-reload
systemctl --user enable --now ml4w-custom-sync.path
systemctl --user enable ml4w-custom-sync.service
"$HOME/.local/bin/ml4w-custom-sync"

echo ":: done. Pick a theme with SUPER+CTRL+T, open a new terminal for zsh changes."
