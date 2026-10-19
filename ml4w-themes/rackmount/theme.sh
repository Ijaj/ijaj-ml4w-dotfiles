#!/usr/bin/env bash
# ML4W Theme Rackmount (custom)
source "$HOME/.config/ml4w/themes/custom-lib.sh"
cc_backup rackmount rackmount.lua
cc_apply custom-rackmount default rackmount 2px
echo ":: Theme set to Rackmount"
