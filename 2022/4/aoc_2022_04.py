import numpy as np

file = open("input.txt", 'r')

Lines = file.readlines()

counter1 = 0
counter2 = 0

for line in Lines:
    list = line.split(",")
    
    elf1 = list[0].split("-")
    elf2 = list[1].split("-")

    elf1_start = int(elf1[0])
    elf1_end = int(elf1[1])
    elf2_start = int(elf2[0])
    elf2_end = int(elf2[1])



    if (elf1_start >= elf2_start and elf1_end <= elf2_end) or (elf1_start <= elf2_start and elf1_end >= elf2_end):
        counter1 += 1
        
    elf1_range = np.linspace(elf1_start, elf1_end, elf1_end - elf1_start +1)
    elf2_range = np.linspace(elf2_start, elf2_end, elf2_end - elf2_start +1)

    check = np.in1d(elf1_range, elf2_range).any()
    
    if check:
        counter2 += 1


print(counter2)

