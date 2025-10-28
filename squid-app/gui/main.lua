---@diagnostic disable: lowercase-global
require("gui/globals")

local EventManager = require("gui/core/event_manager")
local event_manager = EventManager:new()

local Oscilloscope = require("gui/components/oscilloscope")
local oscilloscope = Oscilloscope:new({ x = 100, y = 80, width = 500, height = 200, thickness = 3, color = { r = 255, g = 190, b = 0 } })

local keyToNote = {
    ["Z"] = 60,
    ["S"] = 61,
    ["X"] = 62,
    ["D"] = 63,
    ["C"] = 64,
    ["V"] = 65,
    ["G"] = 66,
    ["B"] = 67,
    ["H"] = 68,
    ["N"] = 69,
    ["J"] = 70,
    ["M"] = 71,

    ["Q"] = 72,
    ["2"] = 73,
    ["W"] = 74,
    ["3"] = 75,
    ["E"] = 76,
    ["R"] = 77,
    ["5"] = 78,
    ["T"] = 79,
    ["6"] = 80,
    ["Y"] = 81,
    ["7"] = 82,
    ["U"] = 83,
    ["I"] = 84,
    ["9"] = 85,
    ["O"] = 86,
    ["0"] = 87,
    ["P"] = 88,
};


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

function waveform(d)
    oscilloscope.data = d;
end

function update()
    local mx, my = engine.get_mouse_pos();
    oscilloscope.x = mx;
    oscilloscope.y = my
    oscilloscope:draw()

    event_manager:update()
end
