require("gui/core/event/event_manager")
require("gui/core/event/handlers/mouse_handler")

InteractionManager = {}

InteractionManager.workspace_interactables = {}
InteractionManager.permanent_interactables = {}

InteractionManager.hovered_component = nil
InteractionManager.pressed_component = nil
InteractionManager.focused_component = nil

function InteractionManager:clear_workspace()
    local is_hovered_in_workspace = false
    for _, comp in ipairs(self.workspace_interactables) do
        if comp == self.hovered_component then
            is_hovered_in_workspace = true
            break
        end
    end

    if is_hovered_in_workspace and self.hovered_component.on_mouse_leave then
        self.hovered_component:on_mouse_leave()
        self.hovered_component = nil
    end

    local is_pressed_in_workspace = false
    for _, comp in ipairs(self.workspace_interactables) do
        if comp == self.pressed_component then
            is_pressed_in_workspace = true
            break
        end
    end
    if is_pressed_in_workspace then
        self.pressed_component = nil
    end

    self.workspace_interactables = {}
end

function InteractionManager:_find_top_component_at(x, y)
    local top_component = nil
    local max_z = -math.huge

    local function search_in_list(list)
        for i = #list, 1, -1 do
            local comp = list[i]
            if comp and comp.get_hitbox and comp.get_z_index then
                local hitbox = comp:get_hitbox()
                local z = comp:get_z_index()

                if x >= hitbox.x and x <= hitbox.x + hitbox.w and
                    y >= hitbox.y and y <= hitbox.y + hitbox.h then
                    if z >= max_z then
                        max_z = z
                        top_component = comp
                    end
                end
            end
        end
    end

    search_in_list(self.permanent_interactables)
    search_in_list(self.workspace_interactables)

    return top_component
end

function InteractionManager:register_permanent(component)
    assert(component, "Cannot register a nil component")
    assert(component.get_hitbox, "Component must have a get_hitbox method")
    assert(component.get_z_index, "Component must have a get_z_index method")
    table.insert(self.permanent_interactables, component)
end

function InteractionManager:register_workspace(component)
    assert(component, "Cannot register a nil component")
    assert(component.get_hitbox, "Component must have a get_hitbox method")
    assert(component.get_z_index, "Component must have a get_z_index method")
    table.insert(self.workspace_interactables, component)
end

function InteractionManager:unregister(component)
    for i = #self.workspace_interactables, 1, -1 do
        if self.workspace_interactables[i] == component then
            table.remove(self.workspace_interactables, i)
            return
        end
    end

    for i = #self.permanent_interactables, 1, -1 do
        if self.permanent_interactables[i] == component then
            table.remove(self.permanent_interactables, i)
            return
        end
    end
end

function InteractionManager:handle_mouse_move(x, y)
    local current_top = self:_find_top_component_at(x, y)

    if self.pressed_component and self.pressed_component.on_mouse_drag then
        self.pressed_component:on_mouse_drag(x, y)
    end

    if current_top ~= self.hovered_component then
        if self.hovered_component and self.hovered_component.on_mouse_leave then
            self.hovered_component:on_mouse_leave()
        end

        if current_top and current_top.on_mouse_enter then
            current_top:on_mouse_enter()
        end

        self.hovered_component = current_top
    end
end

function InteractionManager:handle_mouse_press(button, x, y)
    local target_component = self:_find_top_component_at(x, y)

    if self.focused_component and self.focused_component ~= target_component and self.focused_component.on_blur then
        self.focused_component:on_blur()
        self.focused_component = nil
    end

    if not self.hovered_component then return end

    if self.hovered_component.on_mouse_down then
        self.hovered_component:on_mouse_down(button, x, y)
    end

    if self.hovered_component.on_focus then
        self.hovered_component:on_focus()
        self.focused_component = self.hovered_component
    end

    self.pressed_component = self.hovered_component
end

function InteractionManager:handle_mouse_release(button, x, y)
    if not self.pressed_component then return end

    if self.pressed_component == self.hovered_component then
        if self.pressed_component.on_click then
            self.pressed_component:on_click(button, x, y)
        end
    end

    if self.pressed_component.on_mouse_up then
        self.pressed_component:on_mouse_up(button, x, y)
    end

    self.pressed_component = nil
end

function InteractionManager:init()
    EventManager:on("mouse_move", function(x, y) self:handle_mouse_move(x, y) end)
    EventManager:on("mouse_up", function(btn, x, y) self:handle_mouse_release(btn, x, y) end)
    EventManager:on("mouse_down", function(btn, x, y) self:handle_mouse_press(btn, x, y) end)
end

function InteractionManager:draw_debug_hitboxes()
    local function draw_for_list(list, color)
        for _, comp in ipairs(list) do
            if comp and comp.get_hitbox and comp.get_z_index then
                local hitbox = comp:get_hitbox()
                local z = comp:get_z_index()
                engine.draw_rect({ x = hitbox.x, y = hitbox.y, width = hitbox.w, height = hitbox.h },
                    { r = color[1] * 255, g = color[2] * 255, b = color[3] * 255, a = 80 })
            end
        end
    end

    draw_for_list(self.permanent_interactables, { 1, 0, 0 }) -- Red

    draw_for_list(self.workspace_interactables, { 0, 1, 0 }) -- Green
end
