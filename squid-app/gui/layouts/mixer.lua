require("gui/globals")
require("gui/core/layout")
require("gui/core/size")
require("gui/components/prelude")

local Knob = require("gui/components/knob")
local Col = require("gui/components/col")
local color = require("gui/helpers/color")

local LMeter = require("gui/components/level_meter")
Meter = LMeter:new({ x = 20, y = 20, width = Size.absolute(15), height = Size.absolute(200) });
local fader = VFader:new({ x = 50, y = 20, width = Size.absolute(25), height = Size.absolute(200) });
-- fader:on_change(function(value)
--     Meter:set_level(value)
-- end)

local MixerTrack = require("gui/components/mixer_track")

local trck_list = {}

for i = 0, 20 do
    table.insert(trck_list,
        Container:new({ width = Size.absolute(50), height = Size.relative(1) },
            Center:new({
                child = MixerTrack:new({
                    x = 0,
                    y = 0,
                    width = Size.relative(1),
                    height = Size.relative(0.98),
                    name = "C" ..
                        i
                })
            }))
    )
end

return Layout:new({},
    {
        Container:new({ width = Size.relative(1), height = Size.relative(1) }, Row:new({
            spacing = 2,
            children = trck_list
        }))
    })
