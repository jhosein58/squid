---@diagnostic disable: lowercase-global


if not get_screen_width then
    ---@type fun():number
    function get_screen_width() return 0 end
end

if not get_screen_height then
    ---@type fun():number
    function get_screen_height() return 0 end
end


if not get_mouse_pos then
    ---@type fun():number, number
    function get_mouse_pos() return 0, 0 end
end

if not is_mouse_down then
    ---@type fun(button: string): boolean
    function is_mouse_down(button) return false end
end

if not draw_rect then
    ---@type fun(prop, color)
    function draw_rect(prop, color) end
end

if not draw_circle then
    ---@type fun(prop, color)
    function draw_circle(prop, color) end
end

if not draw_line then
    ---@type fun(prop, color)
    function draw_line(prop, color) end
end

if not draw_text then
    ---@type fun(text, x, y, color)
    function draw_text(text, x, y, color) end
end

if not is_key_down then
    ---@type fun(key: string): boolean
    function is_key_down(key) return false end
end

if not get_pressed_keys then
    ---@type fun(): string[]
    function get_pressed_keys() return {} end
end

if not get_delta_time then
    ---@type fun(): number
    function get_delta_time() return 0 end
end

if not draw_waveform then
     ---@type fun(prop)
     function draw_waveform(p) end
end
