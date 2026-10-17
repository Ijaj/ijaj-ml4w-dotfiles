-- -----------------------------------------------------
-- General window decoration
-- name: "Soft Glass"
-- Big radius, opaque active windows, frosted layers (bar, notifications, launcher)
-- -----------------------------------------------------

hl.config({
    decoration = {
        rounding = 16,
        active_opacity = 1.0,
        inactive_opacity = 0.94,
        fullscreen_opacity = 1.0,

        blur = {
            enabled = true,
            size = 7,
            passes = 3,
            new_optimizations = true,
            ignore_opacity = true,
            xray = false,
            vibrancy = 0.17,
            noise = 0.015,
        },

        shadow = {
            enabled = true,
            range = 40,
            render_power = 3,
            color = "0x40000000",
        },
    }
})

-- Frost the layers; ignore_alpha keeps blur off the transparent gaps between pills
hl.layer_rule({ match = { namespace = "waybar" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "swaync-control-center" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "swaync-notification-window" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "rofi" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "logout_dialog" }, blur = true })
hl.layer_rule({ match = { namespace = "nwg-dock" }, blur = true, ignore_alpha = 0.1 })
