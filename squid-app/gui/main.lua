---@diagnostic disable: lowercase-global
require("gui/globals")
require("gui/core/layout")
require("gui/components/prelude")
require("gui/core/size")
require("gui/core/animator")
require("gui/core/update_manager")
require("gui/core/event/event_manager")

require("gui/core/event/handlers/keyboard_handler")
require("gui/core/event/handlers/mouse_handler")
require("gui/core/interaction_manager")

KeyboardHandler:init()
MouseHandler:init()
InteractionManager:init()

keyToNote = require("gui/core/note_map")

EventManager:on("key_down", function(t)
    if keyToNote[tostring(t)] then
        engine.send_note_on_event(keyToNote[tostring(t)])
    end
    if tostring(t) == "Right" then
        WorkSpaces:switch(WorkSpaces.current + 1)
    end
    if tostring(t) == "Left" then
        WorkSpaces:switch(WorkSpaces.current - 1)
    end
end)

EventManager:on("key_up", function(t)
    if keyToNote[tostring(t)] then
        engine.send_note_off_event(keyToNote[tostring(t)])
    end
end)



left = { 0, 0 }
right = { 0, 0 }
mono = { 0, 0 }
function waveform(data)
    local li, ri = 1, 1
    for i = 1, #data, 2 do
        left[li] = data[i]
        right[ri] = data[i + 1]
        mono[li] = (data[i] + (data[i + 1] or data[i])) / 2 -- fallback if missing
        li = li + 1
        ri = ri + 1
    end
    if #data == 0 then
        left = { 0, 0 }
        right = { 0, 0 }
        mono = { 0, 0 }
    end

    TopBarScope.data = mono
end

local topbar_layout = require("gui/layouts/topbar")
local workspace_layout = require("gui/layouts/workspace")

topbar_layout:register_permanent()



function update()
    local sw, sh = engine.get_screen_width(), engine.get_screen_height()



    topbar_layout:calculate_layout(0, 0, sw, 48)
    topbar_layout:update()
    topbar_layout:draw()

    workspace_layout:calculate_layout(0, 48, sw, sh - 48)
    topbar_layout:update()
    workspace_layout:draw()


    EventManager:on("app_quit", function()
        print("bay")
    end)


    --InteractionManager:draw_debug_hitboxes()
    UpdateManager:update(engine.get_delta_time())
end
