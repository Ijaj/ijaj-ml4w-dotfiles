-- -----------------------------------------------------
-- General window decoration
-- name: "Flat"
-- Small radius, no blur, no shadows, no dimming
-- -----------------------------------------------------

hl.config({
    decoration = {
        rounding = 6,
        active_opacity = 1.0,
        inactive_opacity = 1.0,
        fullscreen_opacity = 1.0,

        blur = {
            enabled = false,
        },

        shadow = {
            enabled = false,
        },
    }
})
