hl.config({
    general = {
        gaps_in  = 10,
        gaps_out = 18,
        border_size = 2,
        col = {
            active_border   = { colors = {primary, tertiary}, angle = 45 },
            inactive_border = "rgba(ffffff10)",
        },
        resize_on_border = true,
        allow_tearing = false,
        layout = "dwindle",
    }
})
