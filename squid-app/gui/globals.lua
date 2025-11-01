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

    ---@type fun(prop, color)
    function engine.draw_rect(prop, color) end

    ---@type fun(prop, color)
    function engine.draw_circle(prop, color) end

    ---@type fun(prop, color)
    function engine.draw_line(prop, color) end

    ---@type fun(text, x, y, color)
    function engine.draw_text(text, x, y, color) end

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

    ---@type fun(prop, color)
    function engine.draw_rounded_rect(prop, color) end

    ---@type fun(prop, color, border_color)
    function engine.draw_bordered_rounded_rect(prop, color, border_color) end

    ---@type fun(path)
    function engine.load_texture(path) end

    ---@type fun(prop)
    function engine.draw_texture(prop) end

    ---@type fun(text, prop)
    function engine.measure_text(text, prop) end
end
