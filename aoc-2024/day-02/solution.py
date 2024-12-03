with open("input.txt") as input_file:
    reports = [list(map(int, line.strip().split(" "))) for line in input_file]


def check_safe(report: list[int], mistakes_allowed: bool):
    direction = 1 if report[1] > report[0] else -1

    for x in range(1, len(report)):
        dif = (report[x] - report[x - 1]) * direction
        if dif < 1 or dif > 3:
            if mistakes_allowed:
                arr1 = report[:x] + report[x + 1 :]
                arr2 = report[: x - 1] + report[x:]
                arr3 = report[: x - 2] + report[x - 1 :]
                return (
                    check_safe(arr1, False)
                    or check_safe(arr2, False)
                    or check_safe(arr3, False)
                )
            else:
                return False

    return True


# part 1
safe_reports = 0
for report in reports:
    if check_safe(report, False):
        safe_reports += 1

print(safe_reports)


# part 2
safe_reports = 0
for report in reports:
    if check_safe(report, True):
        safe_reports += 1

print(safe_reports)
