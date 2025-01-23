import numpy as np


def follows_rules(upx):

    upx.reverse()
    check = True
    sum = 0

    for i, val in np.ndenumerate(upx):
        intersect = np.intersect1d(
            upx[i[0] + 1::], rules[np.argwhere(rules[:, 0] == int(val)), 1])
        if len(intersect):
            check = False
            break
    if check:
        sum += upx[len(upx)//2]
    return check, sum


def sort_correctly(update, rules):

    while True:
        is_sorted = True
        for i in range(len(update) - 1):
            if (update[i+1], update[i]) in rules:
                is_sorted = False
                update[i], update[i+1] = update[i+1], update[i]

        if is_sorted:
            return update


if __name__ == "__main__":

    file = open("input_real.txt", 'r')
    Lines = file.readlines()

    rules = []
    updates = []

    for line in Lines:
        if "|" in line:
            rules.append((int(line[0:2]), int(line[3:5])))
        elif line == '\n':
            continue
        else:
            arr = line.split(",")
            list = []
            for elem in arr:
                list.append(int(elem.replace('\n', '')))
            updates.append(list)

    sum = 0
    rules_tuples = rules
    rules = np.array(rules)

    # Part 1

    for upx in updates:
        sum += follows_rules(upx)[1]

    print("Part 1: ", sum)
    sum = 0

    # Part 2
    for upx in updates:
        upx.reverse()
        if not follows_rules(upx)[0]:
            seq = sort_correctly(upx, rules_tuples)
            sum += seq[len(seq)//2]

    print("Part 2:", sum)
