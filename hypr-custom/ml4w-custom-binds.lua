-- Extra keybindings for the custom pickers. ML4W defaults stay untouched.
-- Loaded from ~/.config/hypr/custom.lua (ml4w-custom-sync adds the require).
local mainMod = "SUPER"
local term = "kitty --class ml4w-picker -e "

hl.bind(mainMod .. " + CTRL + SHIFT + P", hl.dsp.exec_cmd(term .. "ml4w-prompt"), { description = "Pick shell prompt theme" })
hl.bind(mainMod .. " + CTRL + SHIFT + N", hl.dsp.exec_cmd("ml4w-toast"), { description = "Pick notification toast style" })
hl.bind(mainMod .. " + CTRL + SHIFT + R", hl.dsp.exec_cmd("ml4w-rofi"), { description = "Pick launcher (rofi) style" })
