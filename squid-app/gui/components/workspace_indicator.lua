require("gui/core/event/event_manager")
require("gui/core/size")
require("gui/core/interaction_manager")

require("gui/core/base_component")
require("gui/components/container")
require("gui/components/row")
require("gui/components/wspoint")




WorksapceIndicator = BaseComponent:extend()

function WorksapceIndicator:new(prop)
    prop            = prop or {}
    local obj       = BaseComponent.new(self, prop)

    obj.size        = prop.size or 10
    obj.spacing     = prop.spacing or 10
    obj.bg          = prop.bg or { r = 100, g = 100, b = 100, a = 255 }
    obj.count       = prop.count or 4;

    obj.point_count = 0
    obj.point_list  = {}
    obj.root        = Container:new({
        width = obj.width,
        height = obj.height,
        x = obj.x,
        y = obj.y,

    }, Row:new({
        align_y_center = true,
        spacing = obj.spacing,
        reverse_order = true,
        children = obj.point_list
    }))

    for _ = 1, obj.count do
        obj:add_point()
    end

    EventManager:on("switch_workspace", function(num)
        obj:switch_workspace(num)
    end)

    if obj.point_list[1] then
        obj.point_list[1]:on_active()
    end

    return obj
end

function WorksapceIndicator:add_point()
    self.point_count = self.point_count + 1
    local point = WsPoint:new({
        width = Size.absolute(self.size),
        height = Size.absolute(self.size),
        num = self.point_count,
    })
    table.insert(self.point_list, point)
end

function WorksapceIndicator:switch_workspace(num)
    for i, p in ipairs(self.point_list) do
        if i == num then
            p:on_active()
        else
            p:on_inactive()
        end
    end
end

function WorksapceIndicator:register_permanent()
    self.root:register_permanent()
end

function WorksapceIndicator:register_interactive()
    self.root:register_interactive()
end

function WorksapceIndicator:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height, base_z, depth)
    self.root:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height, base_z, depth + 1)
end

function WorksapceIndicator:draw()
    self.root:draw()
end
