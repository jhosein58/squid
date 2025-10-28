BaseComponent = {}
BaseComponent.__index = BaseComponent

function BaseComponent:new(prop)
    local cls  = self
    prop       = prop or {}
    local obj  = setmetatable({}, cls)
    obj.x      = prop.x or 0
    obj.y      = prop.y or 0
    obj.width  = prop.width or 0
    obj.height = prop.height or 0
    return obj
end

function BaseComponent:extend()
    local cls = {}
    setmetatable(cls, { __index = self })
    cls.__index = cls
    return cls
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
    print("test")
end

return BaseComponent
