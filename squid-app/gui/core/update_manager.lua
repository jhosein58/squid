UpdateManager = {}

UpdateManager.updatables = {}

function UpdateManager:register(system)
    for _, s in ipairs(self.updatables) do
        if s == system then return end
    end
    table.insert(self.updatables, system)
end

function UpdateManager:unregister(system)
    for i = #self.updatables, 1, -1 do
        if self.updatables[i] == system then
            table.remove(self.updatables, i)
            return
        end
    end
end

function UpdateManager:update(dt)
    local current_updatables = {}
    for _, s in ipairs(self.updatables) do
        table.insert(current_updatables, s)
    end

    for _, system in ipairs(current_updatables) do
        if system.update then
            system:update(dt)
        end
    end
end
