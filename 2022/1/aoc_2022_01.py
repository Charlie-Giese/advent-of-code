import numpy as np

file = open("input.txt", 'r')

Lines = file.readlines()
count = 0

len_list = []

for line in Lines:
    if line.strip() == "":
        len_list.append(count)
    count+=1

len_list.append(2254)

arr = np.loadtxt("input.txt")



elf_calories = np.zeros(shape = [254])
elf_count = 0

print(np.shape(arr))

for i in range(0, len(len_list)-1):
    if i == 0:
        elf_calories[i] =  np.sum(arr[0:6])
        elf_count += 1
    else:
        start_index = len_list[(i-1)]- elf_count+1
        end_index = len_list[(i ) ]- elf_count-1    
        elf_calories[i] =  np.sum(arr[start_index:end_index+1])
        print(start_index, end_index)
        print(arr[start_index], arr[end_index])
        elf_count += 1

print((np.sort(elf_calories)[::-1]))
