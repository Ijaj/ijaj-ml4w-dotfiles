hl.config({
    general = {
        gaps_in  = 6,
        gaps_out = 12,
        border_size = 2,
        col = {
            active_border   = { colors = {primary, tertiary}, angle = 45 },
            inactive_border = outline_variant,
        },
        resize_on_border = true,
        allow_tearing = false,
        layout = "dwindle",
    }
})
