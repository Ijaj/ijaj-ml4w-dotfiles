#!/usr/bin/env bash
# Shared helpers for the custom global ML4W themes (sourced by their theme.sh).
# Not a folder, so the ML4W theme picker does not list it.
cc_state="$HOME/.local/state/ml4w-custom"

# cc_backup <tag> <marker>
# Saves the current look to pre-<tag>/. Skipped when the theme is already active
# (marker found in decoration.lua) and a backup exists, so re-applying never
# overwrites the real "before" state.
cc_backup() {
    local cfg="$HOME/.config" bk="$cc_state/pre-$1" f
    if grep -q "$2" "$cfg/hypr/conf/decoration.lua" 2>/dev/null && [ -d "$bk" ]; then
        return
    fi
    rm -rf "$bk"; mkdir -p "$bk"
    for f in waybar-theme.sh dock-theme rofi-border.rasi rofi-border-radius.rasi launcher; do
        cp "$cfg/ml4w/settings/$f" "$bk/" 2>/dev/null
    done
    cp "$cfg/swaync/style.css" "$bk/swaync-style.css" 2>/dev/null
    cp "$cfg/wlogout/style.css" "$bk/wlogout-style.css" 2>/dev/null
    cp "$cfg/hypr/conf/decoration.lua" "$cfg/hypr/conf/window.lua" "$bk/" 2>/dev/null
}

# cc_restore <tag>
cc_restore() {
    local cfg="$HOME/.config" bk="$cc_state/pre-$1" f
    [ -d "$bk" ] || { echo ":: nothing to restore"; return 0; }
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
}

# cc_apply <waybar-theme> <waybar-variant> <name> <rofi-radius>
# Applies the shared parts; <name> is the folder name used in every themes/ dir
# and the .lua variant name in hypr conf.
cc_apply() {
    local cfg="$HOME/.config" wb="$1" var="$2" name="$3" radius="$4"
    echo "/$wb;/$wb/$var" > "$cfg/ml4w/settings/waybar-theme.sh"
    "$cfg/waybar/launch.sh" &

    echo "@import \"themes/$name/style.css\";" > "$cfg/swaync/style.css"
    swaync-client -rs

    echo "@import \"themes/$name/style.css\";" > "$cfg/wlogout/style.css"

    echo "$name" > "$cfg/ml4w/settings/dock-theme"
    [ -f "$cfg/ml4w/settings/dock-disabled" ] || "$cfg/nwg-dock-hyprland/launch.sh" &

    echo 'rofi' > "$cfg/ml4w/settings/launcher"
    echo '* { border-width: 1px; }' > "$cfg/ml4w/settings/rofi-border.rasi"
    echo "* { border-radius: $radius; }" > "$cfg/ml4w/settings/rofi-border-radius.rasi"

    echo -e "local name = \"$name.lua\"\nload_variant(name,\"windows\")" > "$cfg/hypr/conf/window.lua"
    echo -e "local name = \"$name.lua\"\nload_variant(name,\"decorations\")" > "$cfg/hypr/conf/decoration.lua"
    hyprctl reload
}
