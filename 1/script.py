numbers = []

with open("input.txt", "r") as f:
    for line in f.readlines():
        numbers.append(int(line))

for n1 in numbers:
    for n2 in numbers:
        if (n1 + n2) == 2020:
            print(n1 * n2)
