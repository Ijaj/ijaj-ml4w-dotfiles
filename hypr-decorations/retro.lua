-- -----------------------------------------------------
-- General window decoration
-- name: "retro"
-- Square, no blur or shadow, solid accent border
-- -----------------------------------------------------

hl.config({
    decoration = {
        rounding = 0,
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
            enabled = false,
            range = 0,
            render_power = 3,
            color = "rgba(00000000)",
        },
    }
})

