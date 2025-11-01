require("gui/globals")
require("gui/core/base_component");
require("gui/core/event/event_manager");

VFader = BaseComponent:extend()



function VFader:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)
    obj.bg = prop.bg or { r = 25, g = 25, b = 25, a = 255 }
    obj.fg = prop.fg or { r = 125, g = 125, b = 125, a = 255 }
    obj.color = prop.color or { r = 255, g = 255, b = 255, a = 255 }
    obj.handle_width = prop.handle_width or 3
    obj.value = prop.value or 0
    obj.on_change_cb = function(_) end


    return obj
end

function VFader:on_change(callback)
    self.on_change_cb = callback
end

function VFader:on_mouse_down(btn, x, y)
    if btn == "left" then
        self:on_mouse_drag(x, y)
    end
end

function VFader:on_mouse_drag(_, y)
    local old_value = self.value
    local cv = math.min(math.max(y, self.computed_y), self.computed_y + self.computed_height)
    self.value = (self.computed_y + self.computed_height - cv) / self.computed_height;

    if self.value ~= old_value then
        self.on_change_cb(self.value)
    end
end

function VFader:draw()
    engine.draw_rect({
        x = self.computed_x,
        y = self.computed_y,
        width = self.computed_width,
        height = self.computed_height
    }, self.bg)

    local iy = (1 - self.value) * self.computed_height
    engine.draw_rect({
        x = self.computed_x,
        y = self.computed_y + iy,
        width = self.computed_width,
        height = self.computed_height - iy
    }, self.fg)

    self.color.a = 255
    if self.value < 0.75 then
        local v = self.value * 3;
        self.color.a = self.color.a * v
    end

    engine.draw_rect({
        x = self.computed_x,
        y = self.computed_y + iy - self.handle_width / 2,
        width = self.computed_width,
        height = self.handle_width
    }, self.color)
end

function VFader:draw_overlay()
    if self.toggle then
        local mx, my = engine.get_mouse_pos();
        engine.draw_text(self.value, { x = mx - 2, y = my - 2, size = 22 }, { r = 0, g = 0, b = 0 })
    end
end
