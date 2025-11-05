require("gui/globals")
require("gui/core/base_component");
require("gui/core/event/event_manager");
require("gui/components/text");

Center = require("gui/components/center")
local draw = require("gui/helpers/drawing")

VFader = BaseComponent:extend()



function VFader:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)
    obj.bg = prop.bg or { r = 25, g = 25, b = 25, a = 255 }
    obj.fg = prop.fg or { r = 125, g = 125, b = 125, a = 255 }
    obj.color = prop.color or { r = 255, g = 255, b = 255, a = 255 }
    obj.handle_height = 4
    obj.handle_color_a = 100
    obj.handle_width = 0.5
    obj.value = prop.value or 0
    obj.on_change_cb = function(_) end
    obj.radius = prop.radius or 6


    obj.hover_anim = nil
    obj.is_dragged = false

    obj.overlay_text = Container:new({
        width = Size.relative(1),
        height = Size.relative(1),
        bg = { r = 0, g = 0, b = 0, a = 180 },
        radius = 4,
    }, Center:new({ child = Text:new({ text = "Hello World !", size = 12 }) }))


    obj.empty_table = {}

    return obj
end

function VFader:on_change(callback)
    self.on_change_cb = callback
end

function VFader:on_mouse_down(btn, x, y)
    if btn == "left" then
        self:on_mouse_drag(x, y)
    end
    self.is_dragged = true
end

function VFader:on_mouse_drag(_, y)
    local old_value = self.value
    local cv = math.min(math.max(y, self.computed_y), self.computed_y + self.computed_height)
    self.value = (self.computed_y + self.computed_height - cv) / self.computed_height;

    if self.value ~= old_value then
        self.on_change_cb(self.value)
    end
end

function VFader:on_mouse_enter()
    if self.hover_anim then
        Animator:stop(self.hover_anim)
    end


    self.hover_anim = Animator:new({
        target   = self,
        to       = {

            handle_color_a = 255,
            handle_width = 0.6,
            handle_height = 5
        },

        duration = 0.05,
        ease     = 'outBack'
    })
end

function VFader:on_mouse_leave()
    if not self.is_dragged then
        self:drag_stop()
    end
end

function VFader:draw()
    draw.bordered_rounded_rect(self.computed_x, self.computed_y, self.computed_width, self.computed_height, self.radius,
        0, self.bg, self.empty_table)

    self.fg.a = 255
    if self.value < 0.80 then
        local v = self.value * 5;
        self.fg.a = self.fg.a * v
    end


    local iy = (1 - self.value) * self.computed_height

    draw.bordered_rounded_rect(self.computed_x, self.computed_y + iy, self.computed_width, self.computed_height - iy,
        self.radius, 2, self.fg, self.bg)


    self.color.a = 255
    if self.value < 0.75 then
        local v = self.value * 3;
        self.color.a = self.color.a * v
    end

    self.color.a = self.handle_color_a


    draw.capsule(self.computed_x + (self.computed_width * (1 - self.handle_width)) / 2,
        self.computed_y + iy - self.handle_height * 2, self.computed_width * self.handle_width, self.handle_height,
        self.color)
end

function VFader:on_mouse_up()
    self.is_dragged = false
    self:drag_stop()
end

function VFader:drag_stop()
    if self.hover_anim then
        Animator:stop(self.hover_anim)
    end

    self.handle_color = { r = 100, g = 100, b = 100, a = 255 }
    self.hover_anim = Animator:new({
        target = self,

        to = {

            handle_height = 4,
            handle_color_a = 100,
            handle_width = 0.5
        },
        duration = 0.4,
        delay = 0.2,
        ease = 'outBack'
    })
end

function VFader:draw_overlay()
    if self.is_dragged then
        local mx, my = engine.get_mouse_pos();
        self.overlay_text.child.child:set_text(string.format("%.2f", self.value))
        self.overlay_text:calculate_layout(mx, my - 20, 40, 20, 0, 0)
        self.overlay_text:draw()
    end
end

return VFader
