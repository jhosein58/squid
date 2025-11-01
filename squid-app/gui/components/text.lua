require("gui/core/size")
local BaseComponent = require("gui/core/base_component")

Text = BaseComponent:extend()

function Text:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)

    obj.text = prop.text or ""
    obj.size = prop.size or 24
    obj.color = prop.color or { r = 255, g = 255, b = 255, a = 255 }
    obj.bg = prop.bg or nil
    obj.radius = prop.radius or 0
    obj.padding = prop.padding or 0
    obj.border_width = prop.border_width or 0
    obj.border_color = prop.border_color or { r = 0, g = 0, b = 0, a = 255 }

    local text_size = engine.measure_text(obj.text, { size = obj.size })

    obj.width = Size.absolute(text_size.width)
    obj.height = Size.absolute(text_size.height)

    return obj
end

function Text:set_text(text)
    self.text = text
    local text_size = engine.measure_text(self.text, { size = self.size })
    self.width = Size.absolute(text_size.width)
    self.height = Size.absolute(text_size.height)
end

function Text:calculate_layout(px, py, pw, ph)
    BaseComponent.calculate_layout(self, px, py, pw, ph)
end

function Text:draw()
    local pad = self.padding;
    if self.bg then
        engine.draw_bordered_rounded_rect({
            x = self.computed_x,
            y = self.computed_y,
            width = self.computed_width + (pad * 2),
            height = self.computed_height + (pad * 2) - self.computed_height / 2,
            radius = self.radius,
            border_width = self.border_width
        }, self.bg, self.border_color);
    end
    engine.draw_text(self.text, {
        x = self.computed_x + pad,
        y = self.computed_y + (self.computed_height / 2) + pad,
        size = self.size
    }, self.color)
end

return Text
