#!/usr/bin/env bash
# ML4W Theme Flat (custom)
source "$HOME/.config/ml4w/themes/custom-lib.sh"
cc_backup flat flat.lua
cc_apply custom-flat default flat 6px
echo ":: Theme set to Flat"
