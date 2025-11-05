require("gui/globals")
require("gui/core/base_component");
require("gui/core/event/event_manager");
require("gui/components/text");

Center = require("gui/components/center")
local draw = require("gui/helpers/drawing")

Knob = BaseComponent:extend()



function Knob:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)
    obj.bg = prop.bg or { r = 25, g = 25, b = 25, a = 255 }
    obj.fg = prop.fg or { r = 230, g = 230, b = 230, a = 255 }
    obj.color = prop.color or { r = 255, g = 255, b = 255, a = 255 }
    obj.radius = prop.radius or Size.fill()

    obj.width = obj.radius
    obj.height = obj.radius

    obj.value = 0
    obj.last_mouse_y = 0

    obj.speed = 3

    obj.overlay_text = Container:new({
        width = Size.relative(1),
        height = Size.relative(1),
        bg = { r = 0, g = 0, b = 0, a = 180 },
        radius = 4,
    }, Center:new({ child = Text:new({ text = "Hello World !", size = 12 }) }))


    obj.empty_table = {}

    return obj
end

function Knob:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height, base_z, depth)
    BaseComponent.calculate_layout(self, parent_abs_x, parent_abs_y, parent_width, parent_height, base_z, depth)
    self.computed_width = self.computed_width * 2
    self.computed_height = self.computed_height * 2
end

function Knob:on_mouse_drag(_, y)
    if y < self.last_mouse_y then
        self.value = self.value + self.speed
    elseif y > self.last_mouse_y then
        self.value = self.value - self.speed
    end

    if self.value > 100 then
        self.value = 0
    elseif self.value < 0 then
        self.value = 100
    end

    self.last_mouse_y = y
end

function Knob:draw()
    local radius = self.computed_width / 2
    local cx = self.computed_x + radius
    local cy = self.computed_y + radius

    local br = radius * 1.1
    draw.circle(cx, cy, br, { r = 12, g = 12, b = 12, a = 255 })
    draw.circle(cx, cy, radius, self.bg)

    self.a_factor = self.value / 100
    self.angle = self.a_factor * math.pi * 2



    local x = cx + math.cos(self.angle) * radius
    local y = cy + math.sin(self.angle) * radius

    draw.line(cx, cy, x, y, 3, self.fg)

    local sr = radius * 0.5;
    draw.circle(cx, cy, sr, self.bg)

    local ssr = radius * 0.35;
    draw.circle(cx, cy, ssr, { r = 12, g = 12, b = 12, a = 255 })
end

return Knob
