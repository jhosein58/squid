EventManager = {}
EventManager.__index = EventManager

function EventManager:new()
    local cls = self
    local obj = setmetatable({}, cls)

    obj.keys = {}
    obj.on_key_down_cb = {}
    obj.on_key_up_cb = {}

    return obj
end

function EventManager:is_key_down(key)
    return key and self.keys[key] == true
end

function EventManager:is_key_up(key)
    return not self:is_key_down(key)
end

function EventManager:on_key_down(func)
    table.insert(self.on_key_down_cb, func)
end

function EventManager:on_key_up(func)
    table.insert(self.on_key_up_cb, func)
end

function EventManager:key_pressed(key)
    for _, func in ipairs(self.on_key_down_cb) do
        func(key)
    end
end

function EventManager:key_released(key)
    for _, func in ipairs(self.on_key_up_cb) do
        func(key)
    end
end

function EventManager:update()
    local next_frame_keys = {}
    local currently_pressed_list = engine.get_pressed_keys()

    for _, key in ipairs(currently_pressed_list) do
        if not self.keys[key] then
            self:key_pressed(key)
        end

        next_frame_keys[key] = true
    end

    for key, _ in pairs(self.keys) do
        if not next_frame_keys[key] then
            self:key_released(key)
        end
    end

    self.keys = next_frame_keys
end

return EventManager
