---@diagnostic disable: lowercase-global
require("gui/globals")
require("gui/core/layout")
require("gui/components/prelude")
require("gui/core/size")
require("gui/core/animation/animator")
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
    local sum_sq = 0
    for i = 1, #mono do
        local s = mono[i]
        sum_sq = sum_sq + s * s
    end

    local rms = math.sqrt(sum_sq / #mono)
    Meter:set_level(rms)
end

local topbar_layout = require("gui/layouts/topbar")
local workspace_layout = require("gui/layouts/workspace")

topbar_layout:register_permanent()

local draw = require("gui/helpers/drawing")


--------------------------------------


package.path    = package.path .. ";gui/lib/lua_midi/?.lua;gui/lib/lua_midi/?/init.lua"
local LuaMidi   = require("LuaMidi")
local Track     = LuaMidi.Track
local NoteEvent = LuaMidi.NoteEvent
local Writer    = LuaMidi.Writer
local function play_note_on(note, velocity)
    engine.send_note_on_event(note)
end

local function play_note_off(note)
    engine.send_note_off_event(note)
end

local bpm = 128
local seconds_per_beat = 60 / bpm

local events = engine.read_midi_file("test.mid")

for i, ev in ipairs(events) do
    ev.time_seconds = ev.time_beats * seconds_per_beat
end

table.sort(events, function(a, b) return a.time_seconds < b.time_seconds end)

local current_time = 0.0
local event_index  = 1

function update_midi(dt)
    current_time = current_time + dt
    while event_index <= #events and events[event_index].time_seconds <= current_time do
        local ev = events[event_index]
        if ev.kind == "on" then
            engine.send_note_on_event(ev.note, ev.velocity or 100)
        else
            engine.send_note_off_event(ev.note)
        end
        event_index = event_index + 1
    end
end

------------------------------------






function update()
    --update_midi(engine.get_delta_time())
    local sw, sh = engine.get_screen_width(), engine.get_screen_height()


    topbar_layout:calculate_layout(0, 0, sw, 48)

    topbar_layout:draw()

    workspace_layout:calculate_layout(0, 48, sw, sh - 48)
    workspace_layout:draw()


    --InteractionManager:draw_debug_hitboxes()
    UpdateManager:update(engine.get_delta_time())
end
