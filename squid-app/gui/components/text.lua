require("gui/core/size")
local draw = require("gui/helpers/drawing")
local BaseComponent = require("gui/core/base_component")

Text = BaseComponent:extend()

function Text:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)


    obj.size = prop.size or 24
    obj.color = prop.color or { r = 255, g = 255, b = 255, a = 255 }

    obj.font = prop.font or "gui/assets/fonts/Roboto/static/Roboto-Regular.ttf"
    engine.load_font(obj.font)
    obj:set_text(prop.text or "")
    return obj
end

function Text:register_permanent()

end

function Text:register_interactive()

end

function Text:measure()
    local text_size = engine.measure_text(self.text, self.size, self.font)
    self.width = Size.absolute(text_size.width)
    self.height = Size.absolute(text_size.height)
end

function Text:set_text(text)
    self.text = text
    self:measure()
end

function Text:calculate_layout(px, py, pw, ph)
    self:measure()
    BaseComponent.calculate_layout(self, px, py, pw, ph)
end

function Text:draw()
    engine.draw_text(self.text or "", self.font, self.computed_x or 0,
        (self.computed_y or 0) + (self.computed_height or 0),
        self
        .size or 18, self.color.r or 255, self.color.g or 255, self.color.b or 255, self.color.a or 255)
end

return Text
