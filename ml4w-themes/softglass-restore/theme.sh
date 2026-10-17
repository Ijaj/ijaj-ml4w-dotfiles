#!/usr/bin/env bash
# ML4W Theme: undo Soft Glass, back to what was set before
cfg="$HOME/.config"
bk="$HOME/.local/state/ml4w-custom/pre-softglass"
[ -d "$bk" ] || { echo ":: nothing to restore"; exit 0; }

for f in waybar-theme.sh dock-theme rofi-border.rasi rofi-border-radius.rasi launcher; do
    cp "$bk/$f" "$cfg/ml4w/settings/" 2>/dev/null
done
cp "$bk/decoration.lua" "$bk/window.lua" "$cfg/hypr/conf/" 2>/dev/null
cp "$bk/swaync-style.css" "$cfg/swaync/style.css" 2>/dev/null
cp "$bk/wlogout-style.css" "$cfg/wlogout/style.css" 2>/dev/null
"$cfg/waybar/launch.sh" &
swaync-client -rs
[ -f "$cfg/ml4w/settings/dock-disabled" ] || "$cfg/nwg-dock-hyprland/launch.sh" &
hyprctl reload
echo ":: Restored previous theme"
