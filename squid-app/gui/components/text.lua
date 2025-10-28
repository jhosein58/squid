require("gui/globals")

local BaseComponent = require("gui/core/base_component");

local Text = BaseComponent:extend()


function Text:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)
    obj.color = prop.color or { r = 0, g = 0, b = 0, a = 255 }
    obj.text = prop.text or ""
    obj.size = prop.size or 14
    return obj
end

function Text:draw()
    engine.draw_text(self.text, {
        x = self.x,
        y = self.y,
        size = self.size,
    }, self.color)
end

return Text
