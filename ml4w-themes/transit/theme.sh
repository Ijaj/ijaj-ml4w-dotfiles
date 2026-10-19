#!/usr/bin/env bash
# ML4W Theme Transit (custom)
source "$HOME/.config/ml4w/themes/custom-lib.sh"
cc_backup transit transit.lua
cc_apply custom-transit default transit 4px
echo ":: Theme set to Transit"
