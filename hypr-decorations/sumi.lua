-- -----------------------------------------------------
-- General window decoration
-- name: "Sumi"
-- Wide gaps, no borders, inactive windows dimmed like dry ink, soft shadow
-- -----------------------------------------------------

hl.config({
    decoration = {
        rounding = 2,
        active_opacity = 1.0,
        inactive_opacity = 0.82,
        fullscreen_opacity = 1.0,
        dim_inactive = true,
        dim_strength = 0.28,

        blur = {
            enabled = true,
            size = 6,
            passes = 3,
            new_optimizations = true,
            ignore_opacity = true,
            xray = false,
            vibrancy = 0.1,
            noise = 0.02,
        },

        shadow = {
            enabled = true,
            range = 34,
            render_power = 3,
            sharp = false,
            offset = "0 8",
            color = "rgba(00000040)",
        },
    }
})
hl.layer_rule({ match = { namespace = "waybar" }, blur = false, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "rofi" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "swaync-control-center" }, blur = true, ignore_alpha = 0.1 })
hl.layer_rule({ match = { namespace = "swaync-notification-window" }, blur = true, ignore_alpha = 0.1 })

-- Slow ink-bleed fades (restored by hyprctl reload when another theme is applied)
hl.animation({ leaf = "fade", enabled = true, speed = 9, bezier = "easeOutExpo" })
hl.animation({ leaf = "windows", enabled = true, speed = 8, bezier = "easeOutExpo", style = "popin 92%" })
hl.animation({ leaf = "border", enabled = true, speed = 14, bezier = "easeOutExpo" })
