require("gui/globals")
local BaseComponent = require("gui/core/base_component")

Vectorscope = BaseComponent:extend()

function Vectorscope:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)

    obj.bg = prop.bg or { r = 20, g = 22, b = 24, a = 255 }
    obj.lc = prop.lc or { r = 100, g = 255, b = 150, a = 255 }
    obj.thickness = prop.thickness or 2.0

    obj.trail_duration = prop.trail_duration or 0.5
    obj.max_trail_points = prop.max_trail_points or 1000

    obj.trail_points = {}
    obj.points_buffer = {}


    return obj
end

function Vectorscope:add_stereo_sample(l, r)
    local point = {
        x = self.x + (l * 0.5 + 0.5) * self.width,
        y = self.y + (r * 0.5 + 0.5) * self.height,
    }
    table.insert(self.points_buffer, point)
end

function Vectorscope:add_stereo_buffer(left_channel_data, right_channel_data)
    for i, left_sample in ipairs(left_channel_data) do
        local right_sample = right_channel_data[i]
        self:add_stereo_sample(left_sample or 0, right_sample or 0)
    end
end

function Vectorscope:update(dt)
    for i = #self.trail_points, 1, -1 do
        local point = self.trail_points[i]
        point.life = point.life - dt
        if point.life <= 0 then
            table.remove(self.trail_points, i)
        end
    end

    for _, p in ipairs(self.points_buffer) do
        local new_trail_point = {
            x = p.x,
            y = p.y,
            life = self.trail_duration
        }
        table.insert(self.trail_points, new_trail_point)
    end

    local excess = #self.trail_points - self.max_trail_points
    if excess > 0 then
        for i = 1, excess do
            table.remove(self.trail_points, 1)
        end
    end

    self.points_buffer = {}
end

function Vectorscope:draw()
    engine.draw_rect({ x = self.x, y = self.y, width = self.width, height = self.height }, self.bg)
    local center_x = self.x + self.width / 2
    local center_y = self.y + self.height / 2
    local axis_color = { r = 255, g = 255, b = 255, a = 40 }
    engine.draw_line({ x1 = self.x, y1 = center_y, x2 = self.x + self.width, y2 = center_y, thickness = 1 }, axis_color)
    engine.draw_line({ x1 = center_x, y1 = self.y, x2 = center_x, y2 = self.y + self.height, thickness = 1 }, axis_color)
    if #self.trail_points < 2 then return end
    for i = 1, #self.trail_points - 1 do
        local p1 = self.trail_points[i]
        local p2 = self.trail_points[i + 1]
        local alpha_multiplier = p2.life / self.trail_duration
        local line_alpha = self.lc.a * alpha_multiplier
        local segment_color = { r = self.lc.r, g = self.lc.g, b = self.lc.b, a = line_alpha }
        engine.draw_line({ x1 = p1.x, y1 = p1.y, x2 = p2.x, y2 = p2.y, thickness = self.thickness }, segment_color)
    end
    local head_point = self.trail_points[#self.trail_points]
    if head_point then
        engine.draw_circle({ x = head_point.x, y = head_point.y, radius = self.thickness }, self.lc)
    end
end
