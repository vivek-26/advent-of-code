def solve(msg, n):  # sliding window technique
    unique_chars = {}  # char -> count mapping

    def add(char):
        if char not in unique_chars:
            unique_chars[char] = 1
        else:
            unique_chars[char] += 1

    def remove(char):
        unique_chars[char] -= 1
        if unique_chars[char] == 0:
            unique_chars.pop(char)

    for i in range(n):
        add(msg[i])

    if len(unique_chars) == n:
        return n

    start = 0
    for i in range(n, len(msg)):
        add(msg[i])
        remove(msg[start])

        start += 1
        if len(unique_chars) == n:
            return i + 1


if __name__ == "__main__":
    inputs = []
    with open("input.txt", "r") as f:
        inputs.append(f.read())

    print(solve(inputs[0], 4))
    print(solve(inputs[0], 14))
