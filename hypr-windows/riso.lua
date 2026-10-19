hl.config({
    general = {
        gaps_in  = 10,
        gaps_out = 20,
        border_size = 3,
        col = {
            active_border   = { colors = { primary, secondary }, angle = 45 },
            inactive_border = outline_variant,
        },
        resize_on_border = true,
        allow_tearing = false,
        layout = "dwindle",
    }
})
