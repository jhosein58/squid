require("gui/globals")

local BaseComponent = require("gui/core/base_component");

local Oscilloscope = BaseComponent:extend()


function Oscilloscope:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)
    obj.bg = prop.bg or { r = 0, g = 0, b = 0, a = 255 }
    obj.color = prop.color or { r = 255, g = 255, b = 255, a = 255 }
    obj.thickness = prop.thickness or 14
    obj.data = {}
    return obj
end

function Oscilloscope:draw()
    engine.draw_rect(
        { x = self.x, y = self.y, width = self.width, height = self.height },
        self.bg
    )

    engine.draw_waveform(
        {
            x = self.x,
            y = self.y,
            width = self.width,
            height = self.height,
            thickness = self.thickness,
            data = self.data,
        },
        self.color
    )
end

return Oscilloscope
