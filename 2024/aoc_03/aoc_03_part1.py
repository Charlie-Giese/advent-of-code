import re

if __name__ == "__main__":

    sum = 0

    with open("input.txt") as file:
        for line in file:
            matches = re.findall(r"mul\((\d+),(\d+)\)", line)
            for match in matches:
                sum += int(match[0]) * int(match[1])
    print(sum)
