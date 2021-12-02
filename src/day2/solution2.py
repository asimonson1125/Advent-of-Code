file = open("input.txt", "r") 
lines = file.readlines()
forward = 0
depth = 0
aim = 0
for i in lines:
    commands = i.split()
    if commands[0] == "forward":
        forward += int(commands[1])
        depth += int(commands[1]) * aim
    elif commands[0] == "down":
        aim += int(commands[1])
    elif commands[0] == "up":
        aim -= int(commands[1])
    else:
        print("Unknown command: " + commands[0])
print(forward, depth, forward*depth)