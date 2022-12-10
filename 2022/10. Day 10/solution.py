x = 1
cnt = 0
sm = 0
crt = [[" " for _ in range(40)] for y in range(6)]


def cycle():
    global cnt, x, sm, crt
    cnt += 1
    if cnt in [20, 60, 100, 140, 180, 220]:
        sm += cnt * x
    if abs((cnt - 1) % 40 - x) < 2:
        crt[(cnt - 1) // 40][(cnt - 1) % 40] = "#"


def solve(lines):
    global x, sm, crt
    for line in lines:
        if line.startswith("noop"):
            cycle()
        else:
            amt = int(line.split(" ")[1])
            cycle()
            cycle()
            x += amt
    print(sm)
    for line in crt:
        print("".join(line))


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        lines = [s.strip() for s in f.readlines()]

    solve(lines)
