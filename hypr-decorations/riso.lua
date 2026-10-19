-- -----------------------------------------------------
-- General window decoration
-- name: "Riso"
-- Tiny corners, hard offset tertiary 'ink' shadow, light blur on layers
-- -----------------------------------------------------

hl.config({
    decoration = {
        rounding = 2,
        active_opacity = 1.0,
        inactive_opacity = 0.97,
        fullscreen_opacity = 1.0,

        blur = {
            enabled = true,
            size = 3,
            passes = 2,
            new_optimizations = true,
            ignore_opacity = true,
            xray = false,
            vibrancy = 0.12,
            noise = 0.01,
        },

        shadow = {
            enabled = true,
            range = 6,
            render_power = 4,
            sharp = true,
            offset = "6 6",
            color = tertiary,
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
