require("gui/globals")

local BaseComponent = require("gui/core/base_component");

local DualChannelOscilloscope = BaseComponent:extend()


function DualChannelOscilloscope:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)
    obj.bg = prop.bg or { r = 0, g = 0, b = 0, a = 255 }
    obj.lc = prop.lc or { r = 255, g = 255, b = 255, a = 255 }
    obj.rc = prop.rc or { r = 255, g = 255, b = 255, a = 255 }
    obj.thickness = prop.thickness or 14
    obj.ld = { 0, 0 }
    obj.rd = { 0, 0 }
    return obj
end

function DualChannelOscilloscope:draw()
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
            data = self.ld,
        },
        self.lc
    )
    engine.draw_waveform(
        {
            x = self.x,
            y = self.y,
            width = self.width,
            height = self.height,
            thickness = self.thickness,
            data = self.rd,
        },
        self.rc
    )
end

return DualChannelOscilloscope
