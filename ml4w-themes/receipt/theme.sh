#!/usr/bin/env bash
# ML4W Theme Receipt (custom)
source "$HOME/.config/ml4w/themes/custom-lib.sh"
cc_backup receipt receipt.lua
cc_apply custom-receipt default receipt 0px
echo ":: Theme set to Receipt"
