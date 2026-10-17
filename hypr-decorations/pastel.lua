-- -----------------------------------------------------
-- General window decoration
-- name: "pastel"
-- Big radius, soft blur and shadow, rounded and airy
-- -----------------------------------------------------

hl.config({
    decoration = {
        rounding = 18,
        active_opacity = 1.0,
        inactive_opacity = 0.95,
        fullscreen_opacity = 1.0,

        blur = {
            enabled = true,
            size = 6,
            passes = 3,
            new_optimizations = true,
            ignore_opacity = true,
            xray = false,
            noise = 0.01,
        },

        shadow = {
            enabled = true,
            range = 30,
            render_power = 3,
            color = "rgba(00000038)",
        },
    }
})

hl.layer_rule({ match = { namespace = "waybar" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "rofi" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "logout_dialog" }, blur = true })
hl.layer_rule({ match = { namespace = "nwg-dock" }, blur = true, ignore_alpha = 0.1 })
