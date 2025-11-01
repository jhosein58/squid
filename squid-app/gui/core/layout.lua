local BaseComponent = require("gui/core/base_component")

Layout = BaseComponent:extend()

function Layout:new(prop, children)
    prop = prop or {}
    prop.children = children or {}

    local obj = BaseComponent.new(self, prop)

    obj.bg = prop.bg or nil
    obj.children = prop.children or {}

    return obj
end

function Layout:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height)
    BaseComponent.calculate_layout(self, parent_abs_x, parent_abs_y, parent_width, parent_height)

    for _, child in ipairs(self.children) do
        if child.calculate_layout then
            child:calculate_layout(self.computed_x, self.computed_y, self.computed_width, self.computed_height)
        end
    end
end

function Layout:draw()
    if self.bg then
        engine.draw_rect(
            { x = self.computed_x, y = self.computed_y, width = self.computed_width, height = self.computed_height },
            self
            .bg)
    end

    for _, child in ipairs(self.children) do
        if child.draw then
            child:draw()
        end
    end
end

function Layout:update()
    for _, child in ipairs(self.children) do
        if child.update then
            child:update()
        end
    end
end

return Layout
