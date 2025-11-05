require("gui/globals")
require("gui/core/layout")
require("gui/core/size")
require("gui/components/prelude")

local Knob = require("gui/components/knob")
local Col = require("gui/components/col")

local LMeter = require("gui/components/level_meter")
Meter = LMeter:new({ x = 20, y = 20, width = Size.absolute(15), height = Size.absolute(200) });
local fader = VFader:new({ x = 50, y = 20, width = Size.absolute(25), height = Size.absolute(200) });
-- fader:on_change(function(value)
--     Meter:set_level(value)
-- end)

local MixerTrack = require("gui/components/mixer_track")

local trck_list = {}

for i = 0, 30 do
    table.insert(trck_list,
        MixerTrack:new({ x = 60 * i, y = 0, width = Size.absolute(60), height = Size.relative(1), name = "C" .. i }))
end

return Layout:new({}, trck_list)
