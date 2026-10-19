-- Extra keybindings for the custom pickers. ML4W defaults stay untouched.
-- Loaded from ~/.config/hypr/custom.lua (ml4w-custom-sync adds the require).
local mainMod = "SUPER"
local term = "kitty --class ml4w-picker -e "

hl.bind(mainMod .. " + CTRL + SHIFT + P", hl.dsp.exec_cmd(term .. "ml4w-prompt"), { description = "Pick shell prompt theme" })
hl.bind(mainMod .. " + CTRL + SHIFT + N", hl.dsp.exec_cmd("ml4w-toast"), { description = "Pick notification toast style" })
hl.bind(mainMod .. " + CTRL + SHIFT + R", hl.dsp.exec_cmd("ml4w-rofi"), { description = "Pick launcher (rofi) style" })
hl.bind(mainMod .. " + CTRL + SHIFT + L", hl.dsp.exec_cmd("ml4w-live"), { description = "Toggle live themes (strata, vital, solar, modal)" })

-- Modal modes. The border color shows the active mode while ml4w-live modal is on.
-- SUPER+ALT+R: resize, SUPER+ALT+M: move. Esc or Enter leaves.
local function leave_keys()
    hl.bind("Escape", hl.dsp.submap("reset"))
    hl.bind("Return", hl.dsp.submap("reset"))
end

hl.bind(mainMod .. " + ALT + R", hl.dsp.submap("resize"), { description = "Resize mode (arrows or hjkl, Esc to leave)" })
hl.define_submap("resize", "reset", function()
    for key, dx, dy in (function()
        local t = { { "right", 40, 0 }, { "left", -40, 0 }, { "down", 0, 40 }, { "up", 0, -40 },
                    { "l", 40, 0 }, { "h", -40, 0 }, { "j", 0, 40 }, { "k", 0, -40 } }
        local i = 0
        return function() i = i + 1; if t[i] then return t[i][1], t[i][2], t[i][3] end end
    end)() do
        hl.bind(key, hl.dsp.window.resize({ x = dx, y = dy, relative = true }), { repeating = true })
    end
    leave_keys()
end)

hl.bind(mainMod .. " + ALT + M", hl.dsp.submap("move"), { description = "Move mode (arrows or hjkl, Esc to leave)" })
hl.define_submap("move", "reset", function()
    for key, dir in pairs({ right = "right", left = "left", down = "down", up = "up", l = "right", h = "left", j = "down", k = "up" }) do
        hl.bind(key, hl.dsp.window.move({ direction = dir }))
    end
    leave_keys()
end)
