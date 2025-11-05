local BaseComponent = require("gui/core/base_component")
local Size = require("gui/core/size")
local color = require("gui/helpers/color")

local Container = require("gui/components/container")
local Row = require("gui/components/row")
local Col = require("gui/components/col")
local VFader = require("gui/components/vfader")
local LMeter = require("gui/components/level_meter")
local Center = require("gui/components/center")

MixerTrack = BaseComponent:extend()

function MixerTrack:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)

    obj.name = prop.name or ""

    obj.child = Container:new({
        x = obj.x,
        y = obj.y,
        width = obj.width,
        height = obj.height,
        bg = color.rgba(0, 0, 0, 235),

    }, Col:new({
        children = {
            Container:new({ width = Size.relative(1), height = Size.relative(0.25), },
                Center:new({
                    child =
                        Container:new({
                            width = Size.relative(0.5),
                            height = Size.relative(0.9),

                        }, Row:new({
                            children = {
                                LMeter:new({ width = Size.relative(0.35), height = Size.relative(1) }),
                                Container:new({ width = Size.relative(0.3), height = Size.relative(1) }),
                                LMeter:new({ width = Size.relative(0.35), height = Size.relative(1) })
                            }
                        }))

                })),
            Container:new({ width = Size.relative(1), height = Size.relative(0.6), },
                Center:new({
                    child = VFader:new({
                        width = Size.relative(0.5),
                        height = Size.relative(1)
                    })

                })),
            Container:new({ width = Size.relative(1), height = Size.relative(0.15), },
                Col:new({
                    reverse_order = true,

                    children = {
                        Container:new({
                            width = Size.relative(1),
                            height = Size.relative(0.5),
                            --bg = color.rgb(25, 25, 25),

                        }, Center:new({
                            child = Text:new({ text = obj.name })
                        })),
                    }
                })
            )
        }
    }))

    return obj
end

function MixerTrack:register_permanent()
    self.child:register_permanent()
end

function MixerTrack:register_interactive()
    self.child:register_interactive()
end

function MixerTrack:calculate_layout(...)
    self.child:calculate_layout(...)
end

function MixerTrack:draw()
    self.child:draw()
end

return MixerTrack
