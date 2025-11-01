local BaseComponent = require("gui/core/base_component")
local Size = require("gui/core/size")

Row = BaseComponent:extend()

function Row:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)

    obj.children = prop.children or {}
    obj.spacing = prop.spacing or 0
    obj.reverse_order = prop.reverse_order or false
    obj.align_y_center = prop.align_y_center or false

    return obj
end

function Row:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height)
    BaseComponent.calculate_layout(self, parent_abs_x, parent_abs_y, parent_width, parent_height)

    if #self.children == 0 then return end

    if self.reverse_order then
        local current_x = self.computed_x + self.computed_width

        for i = #self.children, 1, -1 do
            local child = self.children[i]

            child:calculate_layout(0, 0, self.computed_width, self.computed_height)

            current_x = current_x - child.computed_width

            child:calculate_layout(current_x, self.computed_y, self.computed_width, self.computed_height)

            if self.align_y_center then
                child.computed_y = self.computed_y + (self.computed_height - child.computed_height) / 2
            end

            current_x = current_x - self.spacing
        end
    else
        local current_x = self.computed_x

        for _, child in ipairs(self.children) do
            child:calculate_layout(current_x, self.computed_y, self.computed_width, self.computed_height)

            if self.align_y_center then
                child.computed_y = self.computed_y + (self.computed_height - child.computed_height) / 2
            end

            current_x = current_x + child.computed_width + self.spacing
        end
    end
end

function Row:draw()
    for _, child in ipairs(self.children) do
        child:draw()
    end
end

function Row:update()
    for _, child in ipairs(self.children) do
        child:update()
    end
end

function Row:on_mouse_pressed(button, x, y)
    for _, child in ipairs(self.children) do
        child:on_mouse_pressed(button, x, y)
    end
end

function Row:on_mouse_move(x, y)
    for _, child in ipairs(self.children) do
        child:on_mouse_move(x, y)
    end
end

return Row
