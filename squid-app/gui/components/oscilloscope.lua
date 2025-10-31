require("gui/globals")

local BaseComponent = require("gui/core/base_component");

Oscilloscope = BaseComponent:extend()


function Oscilloscope:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)
    obj.bg = prop.bg or { r = 0, g = 0, b = 0, a = 255 }
    obj.color = prop.color or { r = 255, g = 255, b = 255, a = 255 }
    obj.border_width = prop.border_width or 0
    obj.border_color = prop.border_color or { r = 255, g = 255, b = 255, a = 0 }
    obj.border_radius = prop.border_radius or 0
    obj.thickness = prop.thickness or 14
    obj.data = {}
    return obj
end

function Oscilloscope:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height)
    BaseComponent.calculate_layout(self, parent_abs_x, parent_abs_y, parent_width, parent_height)
end

function Oscilloscope:draw()
    engine.draw_bordered_rounded_rect(
        {
            x = self.computed_x,
            y = self.computed_y,
            width = self.computed_width,
            height = self.computed_height,
            radius = self.border_radius,
            border_width = self.border_width,
        },
        self.bg,

        self.border_color
    )


    engine.draw_waveform(
        {
            x = self.computed_x,
            y = self.computed_y,
            width = self.computed_width,
            height = self.computed_height,
            thickness = self.thickness,
            data = self.data,
        },
        self.color
    )
end
