require("gui/globals")
require("gui/core/layout")
require("gui/core/size")
require("gui/components/prelude")

TopBarScope = Oscilloscope:new({ width = Size.absolute(90), height = Size.relative(1), thickness = 2, color = { r = 0, g = 190, b = 80, a = 255 }, border_radius = 8, })

return Layout:new({ x = 0, y = 0, height = 20, bg = { r = 30, g = 30, b = 30 } }, {
    Container:new(
        { x = 0, y = 0, width = Size.relative(1), height = Size.relative(1), bg = { r = 255, a = 0 }, padding = 5 },
        Row:new({

            children = {
                Container:new(
                    { x = 0, y = 0, width = Size.relative(0.75), height = Size.relative(1) },
                    Row:new({
                        children = {
                            Container:new(
                                { x = 0, y = 0, width = Size.absolute(38), height = Size.relative(1), padding = 9 },
                                Img:new({
                                    x = 0,
                                    y = 0,
                                    width = Size.relative(1),
                                    height = Size.relative(1),
                                    path =
                                    "1.png"
                                })

                            ),
                            Container:new(
                                { x = 0, y = 0, width = Size.absolute(38), height = Size.relative(1), padding = 9 },
                                Img:new({
                                    x = 0,
                                    y = 0,
                                    width = Size.relative(1),
                                    height = Size.relative(1),
                                    path =
                                    "2.png"
                                })

                            ),
                            Container:new(
                                { x = 0, y = 0, width = Size.absolute(38), height = Size.relative(1), padding = 9 },
                                Img:new({
                                    x = 0,
                                    y = 0,
                                    width = Size.relative(1),
                                    height = Size.relative(1),
                                    path =
                                    "3.png"
                                })

                            ),

                            Container:new(
                                { x = 0, y = 0, width = Size.absolute(19), height = Size.relative(1), padding = 9 }),
                            TopBarScope

                        }
                    })
                ),

                Container:new(
                    { x = 0, y = 0, width = Size.relative(0.25), height = Size.relative(1), bg = { b = 255, a = 50 } })

            }
        })
    )
})
