require("gui/core/workspace_manager")
require("gui/core/base_component")
local draw = require("gui/helpers/drawing")

WsPoint = BaseComponent:extend()

function WsPoint:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)

    obj.bg = prop.bg or { r = 100, g = 100, b = 100, a = 255 }
    obj.num = prop.num or 1

    obj.w_factor = 1;
    obj.w_factor_anim = nil

    obj.is_active = false

    return obj
end

function WsPoint:on_active()
    if self.is_active then return end
    self.is_active = true

    self.bg = { r = 255, g = 255, b = 255, a = 255 }

    if self.w_factor_anim then
        Animator:stop(self.w_factor_anim)
    end

    self.w_factor_anim = Animator:new({
        target = self,
        to = {
            w_factor = 2
        },

        duration = 0.5,
        ease = 'outBack'
    })
end

function WsPoint:on_inactive()
    if not self.is_active then return end
    self.is_active = false

    self.bg = { r = 100, g = 100, b = 100, a = 255 }
    if self.w_factor_anim then
        Animator:stop(self.w_factor_anim)
    end

    self.w_factor_anim = Animator:new({
        target = self,
        to = {
            w_factor = 1
        },

        duration = 0.5,
        ease = 'outBack'
    })
end

function WsPoint:on_click()
    Workspaces:switch(self.num)
end

function WsPoint:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height)
    BaseComponent.calculate_layout(self, parent_abs_x, parent_abs_y, parent_width, parent_height)

    self.computed_width = self.computed_width * self.w_factor
end

function WsPoint:draw()
    draw.capsule(
        self.computed_x,
        self.computed_y,
        self.computed_width,
        self.computed_height,
        self.bg)
end

return WsPoint
