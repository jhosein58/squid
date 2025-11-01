require("gui/core/event/event_manager")
require("gui/core/update_manager")

MouseHandler = {}

MouseHandler.state = {
    x = 0,
    y = 0,
    left_key = false,
    right_key = false,
    middle_key = false
}

function MouseHandler:init()
    UpdateManager:register(self)
end

function MouseHandler:update(dt)
    local mx, my = engine.get_mouse_pos()
    local left_key_now = engine.is_mouse_down("left")
    local right_key_now = engine.is_mouse_down("right")
    local middle_key_now = engine.is_mouse_down("middle")

    if self.state.left_key ~= left_key_now then
        if left_key_now then
            EventManager:emit("mouse_down", "left", mx, my)
        else
            EventManager:emit("mouse_up", "left", mx, my)
        end
        self.state.left_key = left_key_now
    end

    if self.state.right_key ~= right_key_now then
        if right_key_now then
            EventManager:emit("mouse_down", "right", mx, my)
        else
            EventManager:emit("mouse_up", "right", mx, my)
        end
        self.state.right_key = right_key_now
    end

    if self.state.middle_key ~= middle_key_now then
        if middle_key_now then
            EventManager:emit("mouse_down", "middle", mx, my)
        else
            EventManager:emit("mouse_up", "middle", mx, my)
        end
        self.state.middle_key = middle_key_now
    end


    if self.state.x ~= mx or self.state.y ~= my then
        EventManager:emit("mouse_move", mx, my)
        self.state.x = mx
        self.state.y = my
    end
end
