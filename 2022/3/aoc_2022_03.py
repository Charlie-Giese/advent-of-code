import numpy as np
import string

file = open("input.txt", 'r')

Lines = file.readlines()

#Problem 1
common_list = []
for line in Lines:
    linelen = len(line)
    word1 = line[0:linelen//2]
    word2 = line[linelen//2:linelen]

    temp = []

    for s in word1:
        for c in word2:
            if s == c:
                temp.append(s)
    
    common_list.append(temp[0])
                
values = np.linspace(1, 52, 52)
alphabet  = list(string.ascii_lowercase)+list(string.ascii_uppercase)
dict = dict(zip(alphabet, values))
sum = 0
for char in common_list:
    sum += dict[char]

#Problem 2
counter = 0
bags = []
badges = []
for line in Lines:
    bags.append(line)
    if counter == 2:
        print(bags)
        a=list(set(bags[0])&set(bags[1])&set(bags[2]))
        for i in a:
            if i != "\n":
                badges.append(i)
        bags = []
        counter = 0
    else:
        counter += 1
    
    

sum = 0
for badge in badges:
    sum += dict[badge]

print(sum)


