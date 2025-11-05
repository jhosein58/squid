require("gui/globals")
require("gui/core/base_component");
require("gui/core/event/event_manager");
require("gui/components/text");

Center = require("gui/components/center")
local ContinuousAnimator = require("gui/core/animation/continuous_animator")
local draw = require("gui/helpers/drawing")

local LevelMeter = BaseComponent:extend()

function LevelMeter:new(prop)
    prop = prop or {}
    local obj = BaseComponent.new(self, prop)

    obj.bg = prop.bg or { r = 15, g = 15, b = 15, a = 255 }
    obj.green_color = prop.green_color or { r = 45, g = 180, b = 60, a = 255 }
    obj.yellow_color = prop.yellow_color or { r = 255, g = 190, b = 0, a = 255 }
    obj.red_color = prop.red_color or { r = 230, g = 40, b = 40, a = 255 }
    obj.head_color = prop.head_color or { r = 255, g = 255, b = 255, a = 255 }

    obj.yellow_threshold = prop.yellow_threshold or 0.6
    obj.red_threshold = prop.red_threshold or 0.9

    obj.value = prop.value or math.random()
    obj.radius = prop.radius or 6
    obj.head_value = math.random()

    obj.peak_hold_delay = prop.peak_hold_delay or 1.0
    obj.peak_hold_timer = 0

    obj.head_tracker = ContinuousAnimator:new({
        target = obj,
        property = "head_value",
        speed = 2.0
    })

    return obj
end

function LevelMeter:register_permanent() end

function LevelMeter:register_interactive() end

function LevelMeter:set_level(value)
    self.value = math.max(0, math.min(1, value))

    if self.value >= self.head_value then
        ContinuousAnimator:set_value(self.head_tracker, self.value)
        self.peak_hold_timer = self.peak_hold_delay
    end
end

function LevelMeter:update(dt)
    if self.peak_hold_timer > 0 then
        self.peak_hold_timer = self.peak_hold_timer - dt
    else
        ContinuousAnimator:set_target(self.head_tracker, self.value)
    end
end

function LevelMeter:draw()
    self:update(engine.get_delta_time())
    draw.rounded_rect(self.computed_x, self.computed_y, self.computed_width, self.computed_height, self.radius, self.bg)

    if self.value > 0 then
        local val = self.value
        local ch = self.computed_height
        local cx = self.computed_x
        local cy = self.computed_y
        local cw = self.computed_width
        local rad = self.radius

        local green_val_to_draw = math.min(val, self.yellow_threshold)
        local green_h = ch * green_val_to_draw
        if green_h > 0 then
            local green_y = cy + ch - green_h
            draw.rounded_rect(cx, green_y, cw, green_h, rad, self.green_color)
            local flatten_h = math.min(rad, green_h)
            draw.rect(cx, green_y, cw, flatten_h, self.green_color)
        end

        if val > self.yellow_threshold then
            local yellow_val_to_draw = math.min(val, self.red_threshold) - self.yellow_threshold
            local yellow_h = ch * yellow_val_to_draw
            local yellow_y = cy + ch * (1.0 - self.yellow_threshold) - yellow_h
            draw.rect(cx, yellow_y, cw, yellow_h, self.yellow_color)
        end

        if val > self.red_threshold then
            local red_val_to_draw = val - self.red_threshold
            local red_h = ch * red_val_to_draw
            if red_h > 0 then
                local red_y = cy + ch * (1.0 - val)
                draw.rounded_rect(cx, red_y, cw, red_h, rad, self.red_color)
                local flatten_h = math.min(rad, red_h)
                draw.rect(cx, red_y + red_h - flatten_h, cw, flatten_h, self.red_color)
            end
        end
    end

    local temp_head_color = { r = self.head_color.r, g = self.head_color.g, b = self.head_color.b }
    local fade_threshold = 0.2
    local fade_t = math.max(0, math.min(1, self.head_value / fade_threshold))
    temp_head_color.a = 255 * fade_t

    draw.rounded_rect(self.computed_x,
        self.computed_y + self.computed_height * (1 - self.head_value),
        self.computed_width, 3, 100, temp_head_color
    )
end

return LevelMeter
