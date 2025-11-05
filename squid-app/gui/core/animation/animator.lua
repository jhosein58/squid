require("gui/core/update_manager")

Animator = {
    active_tweens = {},
}

local pow = math.pow

Animator.easing = {
    linear = function(t, b, c, d) return c * t / d + b end,
    inOutQuad = function(t, b, c, d)
        t = t / d * 2; if t < 1 then return c / 2 * t * t + b end; t = t - 1; return -c / 2 * (t * (t - 2) - 1) + b
    end,
    outBack = function(t, b, c, d, s)
        if not s then s = 1.70158 end; t = t / d - 1; return c * (t * t * ((s + 1) * t + s) + 1) + b
    end,
    inExpo = function(t, b, c, d) if t == 0 then return b else return c * pow(2, 10 * (t / d - 1)) + b end end,
    outExpo = function(t, b, c, d) if t == d then return b + c else return c * (-pow(2, -10 * t / d) + 1) + b end end,
    inOutExpo = function(t, b, c, d)
        if t == 0 then return b end; if t == d then return b + c end; t = t / d * 2; if t < 1 then
            return c / 2 *
                pow(2, 10 * (t - 1)) + b
        end; t = t - 1; return c / 2 * (-pow(2, -10 * t) + 2) + b
    end,
}

UpdateManager:register(Animator)

function Animator:new(props)
    local tween = {}

    tween.target = props.target
    tween.duration = props.duration or 1
    tween.delay = props.delay or 0
    tween.easing_func = self.easing[props.ease or "linear"] or self.easing.linear
    tween.elapsed = 0
    tween.onComplete = props.onComplete

    tween.properties = {}

    if type(props.to) ~= "table" then
        print("Animator Error: 'to' property must be a table.")
        return nil
    end

    if tween.duration == 0 and tween.delay == 0 then
        for prop_name, to_value in pairs(props.to) do
            tween.target[prop_name] = to_value
        end
        if props.onComplete then props.onComplete() end
        return
    end

    for prop_name, to_value in pairs(props.to) do
        local start_value
        if props.from and props.from[prop_name] ~= nil then
            start_value = props.from[prop_name]
        else
            start_value = tween.target[prop_name]
        end

        tween.properties[prop_name] = {
            start = start_value,
            to = to_value,
            change = to_value - start_value,
        }
    end

    table.insert(self.active_tweens, tween)
    return tween
end

function Animator:update(dt)
    for i = #self.active_tweens, 1, -1 do
        local tween = self.active_tweens[i]

        if tween.delay > 0 then
            tween.delay = tween.delay - dt
        else
            tween.elapsed = tween.elapsed + dt

            if tween.elapsed >= tween.duration then
                for prop_name, prop_data in pairs(tween.properties) do
                    tween.target[prop_name] = prop_data.to
                end

                if tween.onComplete then
                    tween.onComplete()
                end
                table.remove(self.active_tweens, i)
            else
                for prop_name, prop_data in pairs(tween.properties) do
                    local current_val = tween.easing_func(tween.elapsed, prop_data.start, prop_data.change,
                        tween.duration)
                    tween.target[prop_name] = current_val
                end
            end
        end
    end
end

function Animator:stop(tween_to_stop)
    if not tween_to_stop then return end
    for i = #self.active_tweens, 1, -1 do
        if self.active_tweens[i] == tween_to_stop then
            table.remove(self.active_tweens, i)
            break
        end
    end
end
