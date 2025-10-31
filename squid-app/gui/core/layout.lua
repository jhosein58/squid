Layout = {}
Layout.__index = Layout

function Layout:new(prop, children)
    local cls = self
    local obj = setmetatable({}, cls)

    obj.x = prop.x or 0
    obj.y = prop.y or 0
    obj.width = prop.width or 0
    obj.height = prop.height or 0
    obj.children = children or {}
    obj.bg = prop.bg or nil

    return obj
end

function Layout:update()
    for _, child in ipairs(self.children) do
        if child.update then
            child:update()
        end
    end
end

function Layout:draw()
    if self.bg then
        engine.draw_rect({ x = self.x, y = self.y, width = self.width, height = self.height }, self.bg)
    end
    for _, child in ipairs(self.children) do
        if child.draw then
            local child_abs_x = self.x + (child.x or 0)
            local child_abs_y = self.y + (child.y or 0)
            child:calculate_layout(child_abs_x, child_abs_y, self.width, self.height)
            child:draw()
        end
    end
end
