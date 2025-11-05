require("gui/core/interaction_manager")
local draw = require("gui/helpers/drawing")
local BaseComponent = require("gui/core/base_component")

Container = BaseComponent:extend()

function Container:new(prop, child)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)

    obj.bg = prop.bg or { r = 0, g = 0, b = 0, a = 0 }
    obj.radius = prop.radius or 0
    obj.border_width = prop.border_width or 0
    obj.border_color = prop.border_color or { r = 0, g = 0, b = 0, a = 0 }
    obj.padding = prop.padding or 0

    obj.child = child or nil

    return obj
end

function Container:register_permanent()
    if self.child and self.child.register_permanent then
        self.child:register_permanent()
    end
end

function Container:register_interactive()
    if self.child and self.child.register_interactive then
        self.child:register_interactive()
    end
end

function Container:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height, base_z, depth)
    BaseComponent.calculate_layout(self, parent_abs_x, parent_abs_y, parent_width, parent_height, base_z, depth)


    if self.child then
        local child_available_x = self.computed_x + self.padding
        local child_available_y = self.computed_y + self.padding
        local child_available_width = self.computed_width - (self.padding * 2)
        local child_available_height = self.computed_height - (self.padding * 2)

        self.child:calculate_layout(child_available_x, child_available_y, child_available_width, child_available_height,
            base_z, (depth or 0) + 1)
    end
end

function Container:draw()
    draw.bordered_rounded_rect(self.computed_x, self.computed_y, self.computed_width, self.computed_height, self.radius,
        self.border_width, self.bg, self.border_color)

    if self.child then
        self.child:draw()
    end
end

function Container:update()
    if self.child then self.child:update() end
end

function Container:on_mouse_pressed(button, x, y)
    if self.child then self.child:on_mouse_pressed(button, x, y) end
end

function Container:on_mouse_move(x, y)
    if self.child then self.child:on_mouse_move(x, y) end
end

return Container
