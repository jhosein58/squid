local draw = {}


local NO_COLOR = { r = 0, g = 0, b = 0, a = 0 }

local core_draw = engine.draw_rect


function draw.rect(x, y, w, h, color)
    core_draw(
        x or 0, y or 0, w or 0, h or 0,
        0,
        0,
        color.r or 0, color.g or 0, color.b or 0, color.a or 255,
        NO_COLOR.r, NO_COLOR.g, NO_COLOR.b, NO_COLOR.a,
        "r"
    )
end

function draw.rounded_rect(x, y, w, h, radius, color)
    core_draw(
        x or 0, y or 0, w or 0, h or 0,
        radius or 0,
        0,
        color.r or 0, color.g or 0, color.b or 0, color.a or 255,
        NO_COLOR.r, NO_COLOR.g, NO_COLOR.b, NO_COLOR.a,
        "r"
    )
end

function draw.bordered_rounded_rect(x, y, w, h, radius, border_width, color, border_color)
    core_draw(
        x or 0, y or 0, w or 0, h or 0,
        radius or 0,
        border_width or 0,
        color.r or 0, color.g or 0, color.b or 0, color.a or 255,
        border_color.r or 0, border_color.g or 0, border_color.b or 0, border_color.a or 0,
        "r"
    )
end

function draw.bordered_rect(x, y, w, h, border_width, color, border_color)
    core_draw(
        x or 0, y or 0, w or 0, h or 0,
        0,
        border_width or 0,
        color.r or 0, color.g or 0, color.b or 0, color.a or 255,
        border_color.r or 0, border_color.g or 0, border_color.b or 0, border_color.a or 0,
        "r"
    )
end

function draw.capsule(x, y, w, h, color)
    core_draw(
        x or 0, y or 0, w or 0, h or 0,
        0, 0,
        color.r or 0, color.g or 0, color.b or 0, color.a or 255,
        NO_COLOR.r, NO_COLOR.g, NO_COLOR.b, NO_COLOR.a,
        "c"
    )
end

function draw.circle(x, y, radius, color)
    engine.draw_circle({
        x = x or 0,
        y = y or 0,
        radius = radius or 0,
    }, color or { r = 0, g = 0, b = 0, a = 255 })
end

function draw.line(x1, y1, x2, y2, thickness, color)
    engine.draw_line({
        x1 = x1 or 0,
        y1 = y1 or 0,
        x2 = x2 or 0,
        y2 = y2 or 0,
        thickness = thickness or 0,
    }, color or { r = 0, g = 0, b = 0, a = 255 })
end

return draw
