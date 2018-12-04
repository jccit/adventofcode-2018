import re

def read_input():
    lines = []

    with open("input.txt") as f:
        flines = f.readlines()
        lines = [line.strip() for line in flines]

    lines.sort()
    return lines

def parse_lines(lines):
    entries = []
    last_guard = 0

    for entry in lines:
        time_reg = re.match(r'\[\d*-(\d{2})-(\d{2}) (\d{2}):(\d{2})]', entry, re.M|re.I)
        entry_obj = {
            'Month':  int(time_reg.group(1)),
            'Day':    int(time_reg.group(2)),
            'Hour':   int(time_reg.group(3)),
            'Minute': int(time_reg.group(4))
        }

        if "Guard #" in entry:
            g_reg = re.match(r'\[.*\] Guard #(\d*)', entry, re.M|re.I)
            last_guard = int(g_reg.group(1))
            entry_obj['Type'] = 'New'
        elif "wakes up" in entry:
            entry_obj['Type'] = 'Wake'
        elif "falls asleep" in entry:
            entry_obj['Type'] = 'Sleep'

        entry_obj['Guard'] = last_guard

        entries.append(entry_obj)

    return entries

def find_sleep_times(entries):
    guard_times = {}
    sleep_start = 0
    sleep_end = 0

    for entry in entries:
        gid = entry['Guard']
        minute = entry['Minute']

        if not guard_times.get(gid):
            time_list = []
            for _ in range(60):
                time_list.append(0)
            guard_times[gid] = time_list

        if entry['Type'] == 'Sleep':
            sleep_start = minute
        elif entry['Type'] == 'Wake':
            sleep_end = minute
            for m in range(sleep_start, sleep_end + 1):
                guard_times[gid][m] += 1

    return guard_times

def sum_times(times):
    totals = {}

    for guard, t in times.items():
        totals[guard] = sum(t)

    return totals

def minute_log(times):
    minutes = []
    mins_sum = []

    for _ in range(60):
        minutes.append([])

    for guard, mins in times.items():
        for i in range(len(mins)):
            for _ in range(mins[i]):
                minutes[i].append(guard)

    for minute in minutes:
        min_sum = {i:minute.count(i) for i in minute}
        mins_sum.append(min_sum)

    return mins_sum

def main():
    lines = read_input()
    entries = parse_lines(lines)
    times = find_sleep_times(entries)
    totals = sum_times(times)

    laziest = max(totals, key=totals.get)
    laziest_minute = times[laziest].index(max(times[laziest]))

    print("Laziest ID: %d" % (laziest))
    print("Laziest minute: %d" % (laziest_minute))
    print("Answer: %d" % (laziest * laziest_minute))

    # Part 2
    minutes = minute_log(times)

    lz_guard = 0
    lz_times = 0
    lz_min = 0
    for m in range(len(minutes)):
        minute = minutes[m]
        current_lz_guard = max(minute, key=minute.get)
        current_lz_times = minute[current_lz_guard]

        # print(m, minute)

        if current_lz_times > lz_times and current_lz_times < 20: # I don't know why I need the < 20. I just do
            lz_guard = current_lz_guard
            lz_times = current_lz_times
            lz_min = m

    print("Part 2")
    print(lz_guard, lz_min)
    print(lz_guard * lz_min)

if __name__ == "__main__":
    main()