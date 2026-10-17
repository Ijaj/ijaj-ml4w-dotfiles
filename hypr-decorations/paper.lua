-- -----------------------------------------------------
-- General window decoration
-- name: "paper"
-- Medium radius, soft diffuse shadow, no blur, windows stay fully opaque
-- -----------------------------------------------------

hl.config({
    decoration = {
        rounding = 12,
        active_opacity = 1.0,
        inactive_opacity = 1.0,
        fullscreen_opacity = 1.0,

        blur = {
            enabled = false,
            size = 1,
            passes = 1,
            new_optimizations = true,
            ignore_opacity = true,
            xray = false,
            noise = 0.01,
        },

        shadow = {
            enabled = true,
            range = 22,
            render_power = 3,
            color = "rgba(00000040)",
        },
    }
})

