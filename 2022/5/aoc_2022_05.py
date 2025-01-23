import numpy as np
import pandas as pd

df = pd.read_csv("input.txt", )

file = open("input.txt", 'r')

Lines = file.readlines()

layout = Lines[0:8][::-1]

stacks = {
    1 : [],
    2 : [],
    3 : [],
    4 : [],
    5 : [],
    6 : [],
    7 : [],
    8 : [],
    9 : []
}

for level in layout:
    chunks, chunk_size = len(level), 4
    j = 1
    for i in range(0, chunks, chunk_size):
        crate = level[i:i+chunk_size]
        stacks[j].append(crate)
        j+= 1


for key in stacks:
    for val in stacks[key]:
         if val == '\n':
            stacks[key].remove('\n')

print(stacks)

instructions = Lines[10:]

for instruc in instructions:
    list = instruc.split(" ")
    num = list[1]
    origin = stacks[list[3]]
    dest = stacks[list[5]]



    dest.append()   
