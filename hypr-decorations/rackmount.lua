-- -----------------------------------------------------
-- General window decoration
-- name: "Rackmount"
-- Tight panel corners, low hard shadow like a bevel, light blur on layers
-- -----------------------------------------------------

hl.config({
    decoration = {
        rounding = 3,
        active_opacity = 1.0,
        inactive_opacity = 0.96,
        fullscreen_opacity = 1.0,

        blur = {
            enabled = true,
            size = 6,
            passes = 3,
            new_optimizations = true,
            ignore_opacity = true,
            xray = false,
            vibrancy = 0.12,
            noise = 0.01,
        },

        shadow = {
            enabled = true,
            range = 8,
            render_power = 4,
            sharp = false,
            offset = "0 3",
            color = "rgba(00000099)",
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
