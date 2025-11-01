---@diagnostic disable: lowercase-global
require("gui/globals")
require("gui/core/layout")
require("gui/components/prelude")
require("gui/core/size")
require("gui/core/animator")

local EventManager = require("gui/core/event_manager")
local event_manager = EventManager:new()

keyToNote = require("gui/core/note_map")
event_manager:on_key_down(function(t)
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

event_manager:on_key_up(function(t)
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




function update()
    local sw, sh = engine.get_screen_width(), engine.get_screen_height()



    topbar_layout:calculate_layout(0, 0, sw, 48)
    topbar_layout:update()
    topbar_layout:draw()

    workspace_layout:calculate_layout(0, 48, sw, sh - 48)
    topbar_layout:update()
    workspace_layout:draw()



    Animator:update(engine.get_delta_time())
    event_manager:update()
end
