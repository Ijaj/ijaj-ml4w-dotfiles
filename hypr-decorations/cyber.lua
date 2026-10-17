-- -----------------------------------------------------
-- General window decoration
-- name: "Cyber"
-- Medium radius, glowing accent shadow, light blur on layers
-- -----------------------------------------------------

hl.config({
    decoration = {
        rounding = 10,
        active_opacity = 1.0,
        inactive_opacity = 0.92,
        fullscreen_opacity = 1.0,

        blur = {
            enabled = true,
            size = 5,
            passes = 2,
            new_optimizations = true,
            ignore_opacity = true,
            xray = false,
            vibrancy = 0.1,
            noise = 0.01,
        },

        shadow = {
            enabled = true,
            range = 18,
            render_power = 2,
            color = primary:sub(1, -4) .. "70)",  -- primary at ~44% alpha
        },
    }
})

hl.layer_rule({ match = { namespace = "waybar" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "rofi" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "logout_dialog" }, blur = true })
hl.layer_rule({ match = { namespace = "nwg-dock" }, blur = true, ignore_alpha = 0.1 })
