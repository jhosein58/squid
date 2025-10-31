local BaseComponent = require("gui/core/base_component")

Img = BaseComponent:extend()

function Img:new(prop, child)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)

    obj.path = prop.path or ""
    obj.x = prop.x or 0
    obj.y = prop.y or 0
    obj.width = prop.width or 0
    obj.height = prop.height or 0


    engine.load_texture("gui/assets/" .. obj.path)

    return obj
end

function Img:calculate_layout(parent_abs_x, parent_abs_y, pw, ph)
    BaseComponent.calculate_layout(self, parent_abs_x, parent_abs_y, pw, ph)
end

function Img:draw()
    engine.draw_texture({
        path = "gui/assets/" .. self.path,
        x = self.computed_x,
        y = self.computed_y,
        width = self.computed_width,
        height = self.computed_height,

    })
end
