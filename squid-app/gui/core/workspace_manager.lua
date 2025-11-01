require("gui/core/animator")
local BaseComponent = require("gui/core/base_component")

WorkspaceManager = BaseComponent:extend()

function WorkspaceManager:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)

    obj.children = prop.children or {}
    obj.current = 1

    obj.text = Text:new({
        text = "Workspace: 1",
        size = 28,
        color = { r = 255, g = 255, b = 255, a = 230 },
        x = 16,
        y = 16,

        padding = 24,
        radius = 8,
        border_width = 4,
        border_color = { r = 0, g = 0, b = 0, a = 100 },
        bg = { r = 0, g = 0, b = 0, a = 100 },

    })

    self.opacity = 0
    obj.anim = nil

    return obj
end

function WorkspaceManager:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height)
    BaseComponent.calculate_layout(self, parent_abs_x, parent_abs_y, parent_width, parent_height)

    if self.opacity > 0 then
        self.text:calculate_layout(self.computed_x, self.computed_y, self.computed_width, self.computed_height)
        self.text.color.a = self.opacity
        self.text.bg.a = self.opacity / 2
        self.text.border_color.a = self.opacity / 2
    end

    local active_child = self.children[self.current]
    if active_child and active_child.calculate_layout then
        active_child:calculate_layout(self.computed_x, self.computed_y, self.computed_width, self.computed_height)
    end
end

function WorkspaceManager:draw()
    local active_child = self.children[self.current]

    if active_child and active_child.draw then
        active_child:draw()
    end


    if self.opacity > 0 then
        self.text:draw()
    end
end

function WorkspaceManager:update()
    local active_child = self.children[self.current]
    if active_child and active_child.update then
        active_child:update()
    end
end

function WorkspaceManager:switch(index)
    if index >= 1 and index <= #self.children then
        self.current = index
        self.text:set_text("Workspace: " .. index)

        self.opacity = 255

        if self.anim then
            Animator:stop(self.anim)
        end

        self.anim = Animator:new({
            target = self,
            property = 'opacity',
            to = 0,
            duration = 1,
            delay = 0.8,
            ease = 'outBack'
        })
    end
end

return WorkspaceManager
