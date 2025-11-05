Size = {}

function Size.absolute(value)
    return { type = 'absolute', value = value }
end

function Size.relative(percentage)
    return { type = 'relative', value = percentage }
end

function Size.fill()
    return { type = 'fill' }
end

function Size.fill_minus(value)
    return { type = 'fill_minus', value = value }
end

return Size
