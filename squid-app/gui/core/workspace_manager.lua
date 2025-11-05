require("gui/core/animation/animator")
require("gui/core/interaction_manager")

Center = require("gui/components/center")
local BaseComponent = require("gui/core/base_component")

local WorkspaceManager = BaseComponent:extend()

function WorkspaceManager:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)

    obj.children = prop.children or {}
    obj.current = 1

    if obj.children[1].register_interactive then
        obj.children[1]:register_interactive()
    end

    obj.text = Container:new({
        width = Size.relative(1),
        height = Size.relative(1),
        bg = { r = 0, g = 0, b = 0, a = 255 }
    }, Center:new({ child = Text:new({ text = "Hello World !" }) }))


    obj.opacity = 0
    obj.text_size = 24
    obj.anim = nil


    return obj
end

function WorkspaceManager:calculate_layout(parent_abs_x, parent_abs_y, parent_width, parent_height, base_z, depth)
    BaseComponent.calculate_layout(self, parent_abs_x, parent_abs_y, parent_width, parent_height, base_z, depth)

    if self.opacity > 0 then
        self.text.child.child.size = self.text_size
        self.text:calculate_layout(self.computed_x, self.computed_y, self.computed_width, self.computed_height,
            base_z, depth + 1)
        self.text.bg.a = self.opacity * 0.5
        self.text.child.child.color.a = self.opacity
    end

    local active_child = self.children[self.current]
    if active_child and active_child.calculate_layout then
        active_child:calculate_layout(self.computed_x, self.computed_y, self.computed_width, self.computed_height,
            base_z, depth + 1)
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

        EventManager:emit("switch_workspace", index)

        local active_child = self.children[self.current]
        InteractionManager:clear_workspace()
        active_child:register_interactive()
        self.text.child.child:set_text("Workspace: " .. index)

        self.opacity = 255
        self.text_size = 24

        if self.anim then
            Animator:stop(self.anim)
        end

        self.anim = Animator:new({
            target = self,
            to = {

                opacity = 0,
                text_size = 28

            },

            duration = 0.4,
            delay = 0.4,

            ease = 'outExpo'
        })
    end
end

local instance = nil

Workspaces = {}

function Workspaces:init(prop)
    prop = prop or {}
    if not instance then
        instance = WorkspaceManager:new(prop)
        table.insert(Workspaces, instance)
    end
    return instance
end

function Workspaces:switch(idx)
    if instance.switch then
        instance:switch(idx)
    end
end
