#!/usr/bin/env bash
# ML4W Theme Soft Glass (custom)
cfg="$HOME/.config"
bk="$HOME/.local/state/ml4w-custom/pre-softglass"

# Remember what was set before, once, so softglass-restore can undo this
if [ ! -d "$bk" ]; then
    mkdir -p "$bk"
    for f in waybar-theme.sh dock-theme rofi-border.rasi rofi-border-radius.rasi launcher; do
        cp "$cfg/ml4w/settings/$f" "$bk/" 2>/dev/null
    done
    cp "$cfg/swaync/style.css" "$bk/swaync-style.css" 2>/dev/null
    cp "$cfg/wlogout/style.css" "$bk/wlogout-style.css" 2>/dev/null
    cp "$cfg/hypr/conf/decoration.lua" "$cfg/hypr/conf/window.lua" "$bk/" 2>/dev/null
fi

echo "/custom-softglass;/custom-softglass/default" > "$cfg/ml4w/settings/waybar-theme.sh"
"$cfg/waybar/launch.sh" &

echo '@import "themes/softglass/style.css";' > "$cfg/swaync/style.css"
swaync-client -rs

echo '@import "themes/softglass/style.css";' > "$cfg/wlogout/style.css"

echo 'softglass' > "$cfg/ml4w/settings/dock-theme"
[ -f "$cfg/ml4w/settings/dock-disabled" ] || "$cfg/nwg-dock-hyprland/launch.sh" &

echo 'rofi' > "$cfg/ml4w/settings/launcher"
echo '* { border-width: 1px; }' > "$cfg/ml4w/settings/rofi-border.rasi"
echo '* { border-radius: 1.4em; }' > "$cfg/ml4w/settings/rofi-border-radius.rasi"

echo -e 'local name = "softglass.lua"\nload_variant(name,"windows")' > "$cfg/hypr/conf/window.lua"
echo -e 'local name = "softglass.lua"\nload_variant(name,"decorations")' > "$cfg/hypr/conf/decoration.lua"
hyprctl reload

echo ":: Theme set to Soft Glass"
