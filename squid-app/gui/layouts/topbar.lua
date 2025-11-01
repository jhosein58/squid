require("gui/globals")
require("gui/core/layout")
require("gui/core/size")
require("gui/components/prelude")
require("gui/layouts/workspace")

TopBarScope = Oscilloscope:new({ width = Size.absolute(120), height = Size.relative(1), thickness = 2, color = { r = 0, g = 190, b = 80, a = 255 }, border_radius = 8, })

return Layout:new({ x = 0, y = 0, bg = { r = 30, g = 30, b = 30 } }, {
    Container:new(
        { x = 0, y = 0, width = Size.relative(1), height = Size.relative(1), bg = { r = 255, a = 0 }, padding = 8 },
        Row:new({

            children = {
                Container:new(
                    { x = 0, y = 0, width = Size.relative(0.75), height = Size.relative(1) },
                    Row:new({
                        spacing = 12,
                        children = {
                            Container:new(
                                { width = Size.absolute(32), height = Size.relative(1), padding = 7 },
                                Img:new({

                                    width = Size.relative(1),
                                    height = Size.relative(1),
                                    path =
                                    "4.png"
                                })),
                            Container:new(
                                { width = Size.absolute(32 * 3), height = Size.relative(1), bg = { r = 0, g = 0, b = 0, a = 150 }, radius = 8 },
                                Row:new({
                                    children = { Container:new(
                                        { x = 0, y = 0, width = Size.absolute(32), height = Size.relative(1), padding = 10 },
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
                                            { x = 0, y = 0, width = Size.absolute(32), height = Size.relative(1), padding = 10 },
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
                                            { x = 0, y = 0, width = Size.absolute(32), height = Size.relative(1), padding = 8 },
                                            Img:new({
                                                x = 0,
                                                y = 0,
                                                width = Size.relative(1),
                                                height = Size.relative(1),
                                                path =
                                                "3.png"
                                            })

                                        ), }
                                })),


                            TopBarScope

                        }
                    })
                ),

                Container:new(
                    { x = 0, y = 0, width = Size.relative(0.25), height = Size.relative(1) },
                    Row:new({
                        align_y_center = true,
                        spacing = 10,
                        reverse_order = true,
                        children = {
                            WsPoint:new({
                                width = Size.absolute(10),
                                height = Size.absolute(10),
                                num = 1,
                                workspaces = WorkSpaces
                            }),
                            WsPoint:new({
                                width = Size.absolute(10),
                                height = Size.absolute(10),
                                num = 2,
                                workspaces = WorkSpaces
                            }),
                            WsPoint:new({
                                width = Size.absolute(10),
                                height = Size.absolute(10),
                                num = 3,
                                workspaces = WorkSpaces
                            }),
                            WsPoint:new({
                                width = Size.absolute(10),
                                height = Size.absolute(10),
                                num = 4,
                                workspaces = WorkSpaces
                            }),
                            Container:new({ width = Size.absolute(2), height = Size.absolute(2) })
                        }
                    })),


            }
        })
    )
})
