#!/usr/bin/env bash
# ML4W Theme Paper (custom)
source "$HOME/.config/ml4w/themes/custom-lib.sh"
cc_backup paper paper.lua
cc_apply custom-paper default paper 10px
echo ":: Theme set to Paper"
