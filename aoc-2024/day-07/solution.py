with open("input.txt") as input_file:
    equations = [
        [int(line.split(": ")[0])] + list(map(int, line.split(": ")[1].split()))
        for line in input_file.read().strip().split("\n")
    ]


valid: set[int] = set()


def backtrack(
    _id: int,
    target: int,
    candidates: list[int],
    operators: list[str],
    allow_concatenation_operator: bool = False,
):
    if _id in valid:
        return

    if len(candidates) - 1 == len(operators):
        result = candidates[0]
        for idx in range(1, len(candidates)):
            if operators[idx - 1] == "+":
                result += candidates[idx]
            elif operators[idx - 1] == "*":
                result *= candidates[idx]
            elif operators[idx - 1] == "||" and allow_concatenation_operator:
                result = int(f"{result}{candidates[idx]}")
            else:
                raise Exception(f"invalid operator: {operators[idx-1]}")

        if result == target:
            valid.add(_id)
        return

    operators.append("+")
    backtrack(_id, target, candidates, operators, allow_concatenation_operator)
    operators.pop()

    operators.append("*")
    backtrack(_id, target, candidates, operators, allow_concatenation_operator)
    operators.pop()

    if allow_concatenation_operator:
        operators.append("||")
        backtrack(_id, target, candidates, operators, allow_concatenation_operator)
        operators.pop()


# part 1
for idx in range(len(equations)):
    backtrack(idx, equations[idx][0], equations[idx][1:], [])

total_calibration_result = sum([equations[idx][0] for idx in valid])
print(total_calibration_result)


# part 2
for idx in range(len(equations)):
    backtrack(idx, equations[idx][0], equations[idx][1:], [], True)

total_calibration_result = sum([equations[idx][0] for idx in valid])
print(total_calibration_result)
