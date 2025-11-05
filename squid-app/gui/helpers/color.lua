local color = {}


local NO_COLOR = { r = 0, g = 0, b = 0, a = 0 }

function color.rgba(r, g, b, a)
    return { r = r or 0, g = g or 0, b = b or 0, a = a or 255 }
end

function color.rgb(r, g, b)
    return { r = r or 0, g = g or 0, b = b or 0, a = 255 }
end

function color.empty()
    return NO_COLOR
end

return color
