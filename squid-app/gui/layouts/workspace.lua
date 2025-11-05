require("gui/globals")
require("gui/core/layout")
require("gui/core/size")
require("gui/components/prelude")

WorkSpaces = Workspaces:init({
    children = {
        require("gui/layouts/mixer"),
        require("gui/layouts/l1"),
        require("gui/layouts/l2"),
        require("gui/layouts/l3"),
        require("gui/layouts/l4")
    }
});

return Layout:new({
    bg = { r = 45, g = 45, b = 45, a = 255 },
    height = Size.relative(1),
    width = Size.relative(1),

}, {
    WorkSpaces
})
