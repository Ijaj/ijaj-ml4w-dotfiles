-- -----------------------------------------------------
-- General window decoration
-- name: "Receipt"
-- Square, flat, light blur on layers; hairline borders
-- -----------------------------------------------------

hl.config({
    decoration = {
        rounding = 0,
        active_opacity = 1.0,
        inactive_opacity = 0.97,
        fullscreen_opacity = 1.0,

        blur = {
            enabled = true,
            size = 4,
            passes = 2,
            new_optimizations = true,
            ignore_opacity = true,
            xray = false,
            vibrancy = 0.12,
            noise = 0.01,
        },

        shadow = {
            enabled = false,
            range = 0,
            render_power = 3,
            color = "rgba(00000000)",
        },
    }
})

-- Frost the layers; ignore_alpha keeps blur off fully transparent gaps
hl.layer_rule({ match = { namespace = "waybar" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "rofi" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "swaync-control-center" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "swaync-notification-window" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "logout_dialog" }, blur = true })
hl.layer_rule({ match = { namespace = "nwg-dock" }, blur = true, ignore_alpha = 0.1 })
