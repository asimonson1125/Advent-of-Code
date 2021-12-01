file = open("day 1.txt", "r") 
lines = file.readlines()
counter = 0
first = 0
second = 0
batches = []
for i in range(len(lines) - 2):
    print(lines[i])
    batch = 0
    for x in range(3):
        batch += int(lines[i+x])
    batches.append(batch)
for i in range(len(batches)-1):
    if(batches[i] < batches[i+1]):
        counter += 1

print(counter)