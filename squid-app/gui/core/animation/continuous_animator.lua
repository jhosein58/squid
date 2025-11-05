require("gui/core/update_manager")

local ContinuousAnimator = {
    active_trackers = {}
}

UpdateManager:register(ContinuousAnimator)

function ContinuousAnimator:new(props)
    local tracker = {}
    tracker.target_object = props.target
    tracker.property_name = props.property
    tracker.speed = props.speed or 5.0
    tracker.precision = props.precision or 0.001

    tracker.current_value = tracker.target_object[tracker.property_name]
    tracker.target_value = tracker.current_value

    table.insert(self.active_trackers, tracker)
    return tracker
end

function ContinuousAnimator:set_target(tracker, new_target)
    tracker.target_value = new_target
end

function ContinuousAnimator:set_value(tracker, new_value)
    tracker.current_value = new_value
    tracker.target_value = new_value
    tracker.target_object[tracker.property_name] = new_value
end

function ContinuousAnimator:update(dt)
    for i = #self.active_trackers, 1, -1 do
        local tracker = self.active_trackers[i]
        local distance = tracker.target_value - tracker.current_value

        if math.abs(distance) < tracker.precision then
            if tracker.current_value ~= tracker.target_value then
                tracker.current_value = tracker.target_value
                tracker.target_object[tracker.property_name] = tracker.current_value
            end
        else
            tracker.current_value = tracker.current_value + distance * tracker.speed * dt
            tracker.target_object[tracker.property_name] = tracker.current_value
        end
    end
end

function ContinuousAnimator:stop(tracker_to_stop)
    if not tracker_to_stop then return end
    for i = #self.active_trackers, 1, -1 do
        if self.active_trackers[i] == tracker_to_stop then
            table.remove(self.active_trackers, i)
            break
        end
    end
end

return ContinuousAnimator
