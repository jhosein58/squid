local BaseComponent = require("gui/core/base_component")

local Center = BaseComponent:extend()

function Center:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)

    obj.child = prop.child or nil

    return obj
end

function Center:register_permanent()
    if self.child then
        self.child:register_permanent()
    end
end

function Center:register_interactive()
    if self.child then
        self.child:register_interactive()
    end
end

function Center:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height, base_z, depth)
    BaseComponent.calculate_layout(self, parent_abs_x, parent_abs_y, parent_width, parent_height, base_z, depth)

    if not self.child then return end

    self.child:calculate_layout(self.computed_x, self.computed_y, self.computed_width, self.computed_height, base_z,
        (depth or 0) + 1)
    self.child.computed_x = self.computed_x + (self.computed_width - self.child.computed_width) / 2
    self.child.computed_y = self.computed_y + (self.computed_height - self.child.computed_height) / 2

    self.child:calculate_layout(self.child.computed_x, self.child.computed_y, self.computed_width, self.computed_height,
        base_z,
        (depth or 0) + 1)
end

function Center:draw()
    if self.child then
        self.child:draw()
    end
end

function Center:update()
    if self.child then
        self.child:update()
    end
end

return Center
