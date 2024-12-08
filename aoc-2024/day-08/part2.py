# https://adventofcode.com/2024/day/8 -- Part 2

"""
Same as the solution to Part 1. We just need to update get_antinodes()
"""

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

    antinodes = []  # list of antinodes to return

    i = 0  # starting at 0 includes the antenna position
    while True:
        an = (c1[0] + drow * i, c1[1] + dcol * i)
        if an in m:
            antinodes.append(an)
            i += 1
        else:
            break  # gone off the edge of the map

    i = 0  # starting at 0 includes the antenna position
    while True:
        an = (c2[0] - drow * i, c2[1] - dcol * i)
        if an in m:
            antinodes.append(an)
            i += 1
        else:
            break  # gone off the edge of the map

    return antinodes


(ant, map) = read_map("input.txt")
antinodes = set()

for freq, positions in ant.items():
    for p in itertools.combinations(positions, 2):
        antinodes.update(get_antinodes(*p, map))

print(f"Number of antinodes: {len(antinodes)}")
