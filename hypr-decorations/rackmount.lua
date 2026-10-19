-- -----------------------------------------------------
-- General window decoration
-- name: "Rackmount"
-- Tight panel corners, low hard shadow like a bevel, no blur
-- -----------------------------------------------------

hl.config({
    decoration = {
        rounding = 3,
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
            range = 8,
            render_power = 4,
            sharp = false,
            offset = "0 3",
            color = "rgba(00000099)",
        },
    }
})
