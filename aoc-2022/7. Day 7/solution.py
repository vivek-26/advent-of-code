class Directory:
    def __init__(self, name):
        self.size = 0
        self.name = name
        self.parent = None
        self.sub_dirs = {}


class Solution:
    def __init__(self):
        self.root = Directory("/")
        self.lines = []

        with open("input.txt") as f:
            for line in f:
                self.lines.append(line.strip())

        self.parse_tree()
        self.root.size = self.total_size(self.root)

    def parse_tree(self):
        curr = self.root

        for line in self.lines[1:]:
            if line.startswith("$ ls"):
                continue
            elif line.startswith("dir"):
                dir_name = line.replace("dir", "").strip()
                sub_dir = Directory(dir_name)
                sub_dir.parent = curr
                curr.sub_dirs[dir_name] = sub_dir
            elif line.startswith("$ cd"):
                path = line.replace("$ cd", "").strip()
                if path != "..":
                    curr = curr.sub_dirs[path]
                else:
                    curr = curr.parent
            else:
                curr.size += int(line.split(" ")[0])

    def total_size(self, node):
        if len(node.sub_dirs) == 0:
            return node.size

        for sd in node.sub_dirs.values():
            node.size += self.total_size(sd)

        return node.size

    def part_one(self):
        total = 0

        def dfs(node):
            nonlocal total
            if node.size <= 100000:
                total += node.size

            for sd in node.sub_dirs.values():
                dfs(sd)

        dfs(self.root)
        print(total)

    def part_two(self):
        space_needed = 30000000 - (70000000 - self.root.size)
        smallest_size = self.root.size

        def dfs(node):
            nonlocal space_needed
            nonlocal smallest_size

            if node.size >= space_needed:
                smallest_size = min(smallest_size, node.size)

            for sd in node.sub_dirs.values():
                dfs(sd)

        dfs(self.root)
        print(smallest_size)


Solution().part_one()
Solution().part_two()
