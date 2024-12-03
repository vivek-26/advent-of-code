import re

with open("input.txt") as input_file:
    inp = input_file.read().strip()

# part 1
matches = re.finditer(r"mul\((\d{1,3}),(\d{1,3})\)", inp, re.MULTILINE)
result = sum([(int(x) * int(y)) for x, y in (match.groups() for match in matches)])
print(result)


# part 2
matches = re.finditer(
    r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))", inp, re.MULTILINE
)
mul_enabled = True

result = 0
for x, y, do, dont in (match.groups() for match in matches):
    if do:
        mul_enabled = True
    elif dont:
        mul_enabled = False
    elif x and y:
        if mul_enabled:
            result += int(x) * int(y)
    else:
        raise ValueError("Invalid input")

print(result)
