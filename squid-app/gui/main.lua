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





local SCOPE_VIEW_WIDTH = 128
local TRIGGER_THRESHOLD = 0

local stabilized_buffer = {}

function waveform(data)
    if not data or #data < SCOPE_VIEW_WIDTH then return end

    local trigger_index = 1
    local found_trigger = false

    local search_limit = #data - SCOPE_VIEW_WIDTH

    for i = 1, search_limit do
        local current_val = data[i]
        local next_val = data[i + 1]

        if current_val <= TRIGGER_THRESHOLD and next_val > TRIGGER_THRESHOLD then
            trigger_index = i
            found_trigger = true
            break
        end
    end

    for k in pairs(stabilized_buffer) do stabilized_buffer[k] = nil end

    for i = 0, SCOPE_VIEW_WIDTH - 1 do
        stabilized_buffer[i + 1] = data[trigger_index + i] or 0
    end

    TopBarScope.data = stabilized_buffer
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
