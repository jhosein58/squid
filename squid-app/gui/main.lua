---@diagnostic disable: lowercase-global
require("gui/globals")

local VF = require("gui/components/vfader")
local Text = require("gui/components/text")

local fader = VF:new({ x = 425, y = 100, width = 25, height = 100 })
local text = Text:new({ text = "Sin Waveform", x = 100, y = 80, size = 25})
local t2 = Text:new({ text = "Sin Waveform", x = 100, y = 220, size = 22, color = {r = 255, g = 0, b = 0, a = 255} })
local x = 0;
local t = 0


phase = phase or 0
p_s = 0;
fader:on_change(function (v) p_s = v end)
function update()
    local data = {}
    local sample_count = 100
    local freq = 3
 local speed = 20 * p_s


    phase = phase + get_delta_time() * speed

    for i = 1, sample_count do
        local t = (i / sample_count) * (math.pi * 2 * freq)
        data[i] = math.sin(t + phase)
    end
    draw_rect(
      { x = 100, y = 100, width = 300, height = 100 },
      { r = 0, g = 0, b = 0, a = 255 }
    )

    draw_waveform(
      {
        x = 100,
        y = 100,
        width = 300,
        height = 100,
        thickness = 3,
        data = data,
      },
      { r = 0, g = 220, b = 120, a = 255 }
    )

    fader:update()
    fader:draw()
    fader:draw_overlay()

    text:draw()
    t2.text = "current speed: " .. speed
    t2:draw()
end
