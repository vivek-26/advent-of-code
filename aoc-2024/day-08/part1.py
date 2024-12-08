# https://adventofcode.com/2024/day/8 -- Part 1

import itertools


def read_map(filename):
    """
    Returns a tuple (antennas, map) where...

    antennas is a dict whose keys are the frequency identifiers (a-z, A-Z, 0-9)
    and whose values are a list of (row, column) tuples indicating the
    position of the antennas of that frequency in the map.

    e.g. {'0': [(1, 8), (2, 5), (3, 7), (4, 4)], 'A': [(5, 6), (8, 8), (9, 9)]}

    map is a dict whose keys are (row, column) tuples and whose value
    is the character in the map (either '.' or one of frequency characters)

    e.g. {(0, 0): '.', (0, 1): '.', (0, 2): '.', ... }
    """
    with open(filename) as file:
        map = {}
        antennas = {}
        for row, line in enumerate(file):
            for col, value in enumerate(line.strip()):
                if value != ".":
                    if value in antennas:
                        l = antennas[value]
                        l.append((row, col))
                    else:
                        antennas[value] = [(row, col)]
                map[(row, col)] = value
        return (antennas, map)


def get_antinodes(c1, c2, m):
    """
    c1 and c2 are the coords of a pair of antennae.
    m is a map.
    Returns the list of coords for the antinodes of that pair of antennae.
    """
    drow = c1[0] - c2[0]  # delta of the row values between c1 and c2
    dcol = c1[1] - c2[1]  # delta of the column values between c1 and c2

    antinodes = []

    an1 = (c1[0] + drow, c1[1] + dcol)
    if an1 in m:
        antinodes.append(an1)

    an2 = (c2[0] - drow, c2[1] - dcol)
    if an2 in m:
        antinodes.append(an2)

    return antinodes


(ant, map) = read_map("input.txt")
antinodes = set()

for freq, positions in ant.items():
    for p in itertools.combinations(positions, 2):
        antinodes.update(get_antinodes(*p, map))

print(f"Number of antinodes: {len(antinodes)}")
