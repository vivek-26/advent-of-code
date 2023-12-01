def move_tail(H, T):
    dr = (H[0] - T[0])
    dc = (H[1] - T[1])

    if abs(dr) <= 1 and abs(dc) <= 1:
        pass
    elif abs(dr) >= 2 and abs(dc) >= 2:  # diagonal
        T = (H[0] - 1 if T[0] < H[0] else H[0] + 1, H[1] - 1 if T[1] < H[1] else H[1] + 1)
    elif abs(dr) >= 2:  # same column, move row
        T = (H[0] - 1 if T[0] < H[0] else H[0] + 1, H[1])
    elif abs(dc) >= 2:  # same row, move column
        T = (H[0], H[1] - 1 if T[1] < H[1] else H[1] + 1)
    return T


def solve():
    H = (0, 0)
    T = [(0, 0) for _ in range(9)]

    DR = {'L': 0, 'U': -1, 'R': 0, 'D': 1}
    DC = {'L': -1, 'U': 0, 'R': 1, 'D': 0}

    visited_part_one = {T[0]}
    visited_part_two = {T[8]}

    for line in lines:
        d, amt = line.split()
        amt = int(amt)
        for _ in range(amt):
            H = (H[0] + DR[d], H[1] + DC[d])
            T[0] = move_tail(H, T[0])
            for i in range(1, 9):
                T[i] = move_tail(T[i - 1], T[i])
            visited_part_one.add(T[0])
            visited_part_two.add(T[8])

    print(len(visited_part_one))
    print(len(visited_part_two))


if __name__ == "__main__":
    lines = []
    with open("input.txt", "r") as f:
        for ln in f.readlines():
            lines.append(ln.strip())

    solve()
