require("gui/globals")
require("gui/core/layout")
require("gui/core/size")
require("gui/components/prelude")



return Layout:new({



}, {
    Container:new(
        { x = 0, y = 0, width = Size.absolute(100), height = Size.absolute(100), bg = { r = 45, g = 255, b = 45, a = 0 } })
})
