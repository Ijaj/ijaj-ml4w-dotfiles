-- -----------------------------------------------------
-- General window decoration
-- name: "Riso"
-- Tiny corners, hard offset tertiary 'ink' shadow, no blur
-- -----------------------------------------------------

hl.config({
    decoration = {
        rounding = 2,
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
            range = 6,
            render_power = 4,
            sharp = true,
            offset = "6 6",
            color = tertiary,
        },
    }
})
