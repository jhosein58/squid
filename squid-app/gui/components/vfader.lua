require("gui/globals")

local BaseComponent = require("gui/core/base_component");

local VFader = BaseComponent:extend()


function VFader:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)
    obj.bg = prop.bg or { r = 0, g = 0, b = 0, a = 255 }
    obj.fg = prop.fg or { r = 255, g = 0, b = 255, a = 255 }
    obj.color = prop.color or { r = 0, g = 0, b = 255, a = 255 }
    obj.value = prop.value or 0
    obj.last_value = obj.value
    obj.toggle = false;
    obj.on_change_cb = function (v) end

    return obj
end

function VFader:update()
    if not (self.last_value == self.value) then
        self.last_value = self.value
        local c = self.on_change_cb
        c(self.value)
    end
    if is_mouse_down("left") and self:is_hovered() then
        self.toggle = true
    end
    if self.toggle and not is_mouse_down("left") then
        self.toggle = false
    end
    if self.toggle then
        local bottom = self.y + self.height;
        local mx, my = get_mouse_pos();

        if my >= bottom then
            self.value = 0
            return
        elseif my <= self.y then
            self.value = 1
            return
        end

        local mh = bottom - my;
        self.value = mh / self.height;
    end
end

function VFader:on_change(callback)
    self.on_change_cb = callback
end

function VFader:draw()
    draw_rect({
        x = self.x,
        y = self.y,
        width = self.width,
        height = self.height
    }, self.bg)

    local iy = (1 - self.value) * self.height
    draw_rect({
        x = self.x,
        y = self.y + iy,
        width = self.width,
        height = self.height - iy
    }, self.fg)
end

function VFader:draw_overlay()
    if self.toggle then
        local mx, my = get_mouse_pos();
        draw_text(self.value, {x = mx - 2, y = my - 2, size = 22}, {r = 0, g = 0, b = 0})

    end
end

return VFader
