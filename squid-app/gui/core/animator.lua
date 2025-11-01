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
    tween.property = props.property
    tween.to_value = props.to
    tween.duration = props.duration or 1
    tween.delay = props.delay or 0

    tween.start_value = props.from or tween.target[tween.property]

    if tween.duration == 0 and tween.delay == 0 then
        tween.target[tween.property] = tween.to_value
        if props.onComplete then props.onComplete() end
        return
    end

    tween.change = tween.to_value - tween.start_value
    tween.easing_func = self.easing[props.ease or "linear"] or self.easing.linear
    tween.elapsed = 0
    tween.onComplete = props.onComplete

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
                tween.target[tween.property] = tween.to_value
                if tween.onComplete then
                    tween.onComplete()
                end
                table.remove(self.active_tweens, i)
            else
                local current_val = tween.easing_func(tween.elapsed, tween.start_value, tween.change, tween.duration)
                tween.target[tween.property] = current_val
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
