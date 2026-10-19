#!/usr/bin/env bash
# ML4W Theme Blueprint (custom)
source "$HOME/.config/ml4w/themes/custom-lib.sh"
cc_backup blueprint blueprint.lua
cc_apply custom-blueprint default blueprint 0px
echo ":: Theme set to Blueprint"
