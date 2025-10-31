---@diagnostic disable: lowercase-global
require("gui/globals")
require("gui/core/layout")
require("gui/components/prelude")
require("gui/core/size")


local EventManager = require("gui/core/event_manager")
local event_manager = EventManager:new()

local oscilloscope = Oscilloscope:new({ x = 50, y = 50, width = 300, height = 100, thickness = 2, color = { r = 0, g = 190, b = 80, a = 255 }, border_radius = 8, })
my_vectorscope = Vectorscope:new({
    x = 50,
    y = 50,
    width = 400,
    height = 400,
    trail_duration = 0.1,
    max_trail_points = 100,
    lc = { r = 0, g = 255, b = 180, a = 255 }
})

keyToNote = require("gui/core/note_map")
event_manager:on_key_down(function(t)
    if keyToNote[tostring(t)] then
        engine.send_note_on_event(keyToNote[tostring(t)])
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

function update()
    local sw, sh = engine.get_screen_width(), engine.get_screen_height()
    topbar_layout.width = sw
    topbar_layout.height = 48
    topbar_layout:draw()



    event_manager:update()
end
