import re
from operator import itemgetter


def input_precedence(message):
    if message == "falls asleep":
        return 2
    elif message == "wakes up":
        return 1
    else:
        return 0


def parse_input(filename):
    lines = open(filename).read().strip().split("\n")
    p = re.compile(r"\[(\d+)\-(\d+)\-(\d+) (\d+):(\d+)\] (.*)")
    events = []
    for line in lines:
        m = p.match(line).groups()
        timestamp, event_message = m[:5], m[5]
        events.append(
            (timestamp, event_message, input_precedence(event_message))
        )
    events.sort(key=itemgetter(0, 2))
    return events


def main():
    guard_events = parse_input("input.txt")
    # guard_events = parse_input("test.in")

    minutes_asleep = {}

    p = re.compile(r"Guard #(\d+)")
    current_guard = None
    falls_asleep_at = None
    for i, event in enumerate(guard_events):
        timestamp, event_message, _ = event
        guard_id = p.match(event_message)
        if guard_id:
            current_guard = guard_id.groups()[0]
            if not minutes_asleep.get(current_guard):
                minutes_asleep[current_guard] = {
                    "count": 0,
                    "minute_count": [0]*60,
                }

        if event_message == "falls asleep":
            falls_asleep_at = int(timestamp[-1])
        elif event_message == "wakes up":
            wakes_up_at = int(timestamp[-1])
            for i in range(falls_asleep_at, wakes_up_at):
                minutes_asleep[current_guard]["minute_count"][i] += 1
            falls_asleep_at = None

    max_asleep, max_guard, max_minute = 0, None, 0
    for guard in minutes_asleep:
        total_minutes_asleep = sum(minutes_asleep[guard]["minute_count"])
        minute_most_asleep = max(minutes_asleep[guard]["minute_count"])
        minute = minutes_asleep[guard]["minute_count"].index(minute_most_asleep)

        if total_minutes_asleep > max_asleep:
            max_asleep = total_minutes_asleep
            max_minute = minute
            max_guard = guard

    print("{} * {} = {}".format(
        max_guard, max_minute, int(max_guard) * int(max_minute))
    )

    max_guard, max_most_freq_minute, max_min = None, 0, 0
    for guard in minutes_asleep:
        minute_most_asleep = max(minutes_asleep[guard]["minute_count"])
        minute = minutes_asleep[guard]["minute_count"].index(minute_most_asleep)

        if minute_most_asleep > max_most_freq_minute:
            max_guard = guard
            max_min = minute
            max_most_freq_minute = minute_most_asleep

    print("{} * {} = {}".format(
        max_guard, max_min, int(max_guard) * int(max_min))
    )
    
    
if __name__ == "__main__":
    main()
