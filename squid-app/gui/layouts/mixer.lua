require("gui/globals")
require("gui/core/layout")
require("gui/core/size")
require("gui/components/prelude")


local f = {}

for i = 1, 4 do
    f[#f + 1] = VFader:new({ x = 20 + (i - 1) * 40, y = 20, width = Size.absolute(20), height = Size.absolute(180), value = 0.75 })
end


return Layout:new({}, f)
