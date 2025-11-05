local BaseComponent = require("gui/core/base_component")
local Size = require("gui/core/size")

Column = BaseComponent:extend()

function Column:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)

    obj.children = prop.children or {}
    obj.spacing = prop.spacing or 0
    obj.reverse_order = prop.reverse_order or false
    obj.align_x_center = prop.align_x_center or false

    return obj
end

function Column:register_permanent()
    for _, child in ipairs(self.children) do
        child:register_permanent()
    end
end

function Column:register_interactive()
    for _, child in ipairs(self.children) do
        child:register_interactive()
    end
end

function Column:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height, base_z, depth)
    BaseComponent.calculate_layout(self, parent_abs_x, parent_abs_y, parent_width, parent_height, base_z, depth)

    if #self.children == 0 then return end

    if self.reverse_order then
        local current_y = self.computed_y + self.computed_height

        for i = #self.children, 1, -1 do
            local child = self.children[i]

            child:calculate_layout(0, 0, self.computed_width, self.computed_height, base_z, depth + 1)

            current_y = current_y - child.computed_height

            child:calculate_layout(self.computed_x, current_y, self.computed_width, self.computed_height, base_z,
                depth + 1)

            if self.align_x_center then
                child.computed_x = self.computed_x + (self.computed_width - child.computed_width) / 2
            end

            current_y = current_y - self.spacing
        end
    else
        local current_y = self.computed_y

        for _, child in ipairs(self.children) do
            child:calculate_layout(self.computed_x, current_y, self.computed_width, self.computed_height, base_z,
                depth + 1)

            if self.align_x_center then
                child.computed_x = self.computed_x + (self.computed_width - child.computed_width) / 2
            end

            current_y = current_y + child.computed_height + self.spacing
        end
    end
end

function Column:draw()
    for _, child in ipairs(self.children) do
        child:draw()
    end
end

function Column:update()
    for _, child in ipairs(self.children) do
        child:update()
    end
end

function Column:on_mouse_pressed(button, x, y)
    for _, child in ipairs(self.children) do
        child:on_mouse_pressed(button, x, y)
    end
end

function Column:on_mouse_move(x, y)
    for _, child in ipairs(self.children) do
        child:on_mouse_move(x, y)
    end
end

return Column
