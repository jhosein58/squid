---@diagnostic disable: lowercase-global

if not engine then
    engine = {}
    engine.__index = engine

    ---@type fun():number
    function engine.get_screen_width() return 0 end

    ---@type fun():number
    function engine.get_screen_height() return 0 end

    ---@type fun():number, number
    function engine.get_mouse_pos() return 0, 0 end

    ---@type fun(button: string): boolean
    function engine.is_mouse_down(button) return false end

    ---@type fun(x,y,w,h,r,bw,c_r,c_g,c_b,c_a,bc_r,bc_g,bc_b,bc_a, s)
    function engine.draw_rect(x, y, w, h, r, bw, c_r, c_g, c_b, c_a, bc_r, bc_g, bc_b, bc_a, s) end

    ---@type fun(prop, color)
    function engine.draw_circle(prop, color) end

    ---@type fun(prop, color)
    function engine.draw_line(prop, color) end

    ---@type fun(key: string): boolean
    function engine.is_key_down(key) return false end

    ---@type fun(): string[]
    function engine.get_pressed_keys() return {} end

    ---@type fun(): number
    function engine.get_delta_time() return 0 end

    ---@type fun(prop, color)
    function engine.draw_waveform(p, color) end

    ---@type fun(note)
    function engine.send_note_on_event(note) end

    ---@type fun(note)
    function engine.send_note_off_event(note) end

    ---@type fun(path)
    function engine.load_texture(path) end

    ---@type fun(prop)
    function engine.draw_texture(prop) end

    ---@type fun(text, size, font)
    function engine.measure_text(text, size, font) end

    ---@type fun(path)
    function engine.load_font(path) end

    ---@type fun(text, font, x, y, s, c_r, c_g, c_b, c_a)
    function engine.draw_text(text, font, x, y, size, color_r, color_g, color_b, color_a) end
end
