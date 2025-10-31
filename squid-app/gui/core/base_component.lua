require("gui/core/size")

BaseComponent = {}
BaseComponent.__index = BaseComponent

function BaseComponent:new(prop)
    local cls           = self
    prop                = prop or {}
    local obj           = setmetatable({}, cls)
    obj.x               = prop.x or 0
    obj.y               = prop.y or 0

    obj.width           = prop.width or Size.fill()
    obj.height          = prop.height or Size.fill()

    obj.align_h         = prop.align_h or 'start'
    obj.align_v         = prop.align_v or 'start'

    obj.computed_x      = 0
    obj.computed_y      = 0
    obj.computed_width  = 0
    obj.computed_height = 0
    return obj
end

function BaseComponent:extend()
    local cls = {}
    setmetatable(cls, { __index = self })
    cls.__index = cls
    return cls
end

function BaseComponent:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height)
    self.computed_x = parent_abs_x + self.x
    self.computed_y = parent_abs_y + self.y

    if self.width.type == 'absolute' then
        self.computed_width = self.width.value
    elseif self.width.type == 'relative' then
        self.computed_width = parent_width * self.width.value
    elseif self.width.type == 'fill' then
        self.computed_width = parent_width - self.x
    elseif self.width.type == 'fill_minus' then
        self.computed_width = (parent_width - self.x) - self.width.value
    end

    if self.height.type == 'absolute' then
        self.computed_height = self.height.value
    elseif self.height.type == 'relative' then
        self.computed_height = parent_height * self.height.value
    elseif self.height.type == 'fill' then
        self.computed_height = parent_height - self.y
    elseif self.height.type == 'fill_minus' then
        self.computed_height = (parent_height - self.y) - self.height.value
    end
end

function BaseComponent:is_hovered()
    local x, y = engine.get_mouse_pos()
    return x >= self.x and x <= self.x + self.width and y >= self.y and y <= self.y + self.height
end

function BaseComponent:update()
end

function BaseComponent:on_mouse_pressed(button, x, y)
end

function BaseComponent:on_mouse_move(x, y)
end

function BaseComponent:draw()

end

return BaseComponent
