require("gui/core/event/event_manager")
require("gui/core/update_manager")

KeyboardHandler = {}

KeyboardHandler.previous_keys = {}

function KeyboardHandler:init()
    UpdateManager:register(self)
end

function KeyboardHandler:update(dt)
    local current_keys_set = {}
    local currently_pressed_list = engine.get_pressed_keys()

    for _, key in ipairs(currently_pressed_list) do
        current_keys_set[key] = true
    end

    for key, _ in pairs(current_keys_set) do
        if not self.previous_keys[key] then
            EventManager:emit("key_down", key)
        end
    end

    for key, _ in pairs(self.previous_keys) do
        if not current_keys_set[key] then
            EventManager:emit("key_up", key)
        end
    end

    self.previous_keys = current_keys_set
end
