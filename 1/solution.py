file = open("day 1.txt", "r") 
lines = file.readlines()
counter = 0
for i in range(len(lines) - 1):
    if int(lines[i]) < int(lines[i+1]):
        counter += 1
        print (lines[i] + " " + lines[i+1])

print(counter)