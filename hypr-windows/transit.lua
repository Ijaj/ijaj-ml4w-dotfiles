hl.config({
    general = {
        gaps_in  = 6,
        gaps_out = 12,
        border_size = 3,
        col = {
            active_border   = { colors = { primary, tertiary }, angle = 90 },
            inactive_border = outline_variant,
        },
        resize_on_border = true,
        allow_tearing = false,
        layout = "dwindle",
    }
})
