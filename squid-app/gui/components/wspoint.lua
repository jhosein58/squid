local BaseComponent = require("gui/core/base_component")

WsPoint = BaseComponent:extend()

function WsPoint:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)

    obj.bg = prop.bg or { r = 100, g = 100, b = 100, a = 255 }
    obj.num = prop.num or 1
    obj.workspaces = prop.workspaces or {}

    obj.w_factor = 1;
    obj.dt = 0
    self.w_factor_anim = nil

    obj.animation_speed = 3

    obj.is_active = false
    obj.last_active = false

    return obj
end

function WsPoint:on_active()
    self.bg = { r = 255, g = 255, b = 255, a = 255 }

    if self.w_factor_anim then
        Animator:stop(self.w_factor_anim)
    end

    self.w_factor_anim = Animator:new({
        target = self,
        property = 'w_factor',
        to = 2,
        duration = 0.5,
        ease = 'outBack'
    })
end

function WsPoint:on_inactive()
    self.bg = { r = 100, g = 100, b = 100, a = 255 }
    if self.w_factor_anim then
        Animator:stop(self.w_factor_anim)
    end

    self.w_factor_anim = Animator:new({
        target = self,
        property = 'w_factor',
        to = 1,
        duration = 0.5,
        ease = 'outBack'
    })
end

function WsPoint:update()
    if self.workspaces.current == self.num then
        self.is_active = true

        if self.is_active ~= self.last_active then
            self:on_active()
            self.last_active = self.is_active
        end
    else
        self.is_active = false

        if self.last_active ~= self.is_active then
            self:on_inactive()
            self.last_active = self.is_active
        end
    end
end

function WsPoint:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height)
    BaseComponent.calculate_layout(self, parent_abs_x, parent_abs_y, parent_width, parent_height)

    self.computed_width = self.computed_width * self.w_factor
end

function WsPoint:draw()
    engine.draw_rounded_rect({
        x = self.computed_x,
        y = self.computed_y,
        width = self.computed_width,
        height = self.computed_height,
        shape = "capsule",
    }, self.bg)
end

return WsPoint
