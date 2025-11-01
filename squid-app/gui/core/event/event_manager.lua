EventManager = {}

EventManager.listeners = {}
EventManager.event_handlers = {}

function EventManager:subscribe(eventName, callback)
    if not self.listeners[eventName] then
        self.listeners[eventName] = {}
    end
    table.insert(self.listeners[eventName], callback)
end

EventManager.on = EventManager.subscribe

function EventManager:emit(eventName, ...)
    if not self.listeners[eventName] then
        return
    end

    local listenersForEvent = {}
    for _, cb in ipairs(self.listeners[eventName]) do
        table.insert(listenersForEvent, cb)
    end

    for _, callback in ipairs(listenersForEvent) do
        callback(...)
    end
end

function EventManager:unsubscribe(eventName, callback)
    if not self.listeners[eventName] then
        return
    end

    local listeners = self.listeners[eventName]
    for i = #listeners, 1, -1 do
        if listeners[i] == callback then
            table.remove(listeners, i)
        end
    end
end

function EventManager:register_event_handler(handler)
    table.insert(self.event_handlers, handler)
end

function EventManager:update(dt)
    for _, handler in ipairs(self.event_handlers) do
        if handler.update then
            handler.update(dt)
        end
    end
end
