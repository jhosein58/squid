function update()
    local w = get_screen_width()
    local h = get_screen_height()
    local mx, my = get_mouse_pos()
    local t = os.clock()
    local hue_shift = (math.sin(t) * 0.5 + 0.5)
    local mouse_ratio_x = mx / w
    local mouse_ratio_y = my / h

    for i = 0, 30000 do
        local cx = (math.sin(t + i * 0.2) * 0.5 + 0.5) * w
        local cy = (math.cos(t * 0.6 + i * 0.3) * 0.5 + 0.5) * h

        local sz = 20 + 30 * mouse_ratio_y + math.sin(t + i) * 10
        local r = math.abs(math.sin(i * 0.3 + hue_shift + mouse_ratio_x)) * 255
        local g = math.abs(math.sin(i * 0.4 + hue_shift + mouse_ratio_y)) * 255
        local b = math.abs(math.cos(i * 0.5 + hue_shift)) * 255

        draw_rect({ x = cx - sz / 2, y = cy - sz / 2, width = sz, height = sz }, { r = r, g = g, b = b, a = 255 })
    end

    draw_rect(
        { x = 0, y = 0, width = w, height = h },
        { r = mouse_ratio_x * 80, g = mouse_ratio_y * 80, b = 100, a = 80 }
    )
end
