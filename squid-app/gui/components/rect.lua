require("gui/globals")

local BaseComponent = require("gui/core/base_component");

local Rect = BaseComponent:extend()


function Rect:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)
    obj.color = prop.color or { r = 0, g = 0, b = 0, a = 255 }
    return obj
end

function Rect:draw()
    draw_rect({
        x = self.x,
        y = self.y,
        width = self.width,
        height = self.height,

    }, self.color)
end

return Rect
