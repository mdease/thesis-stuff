import math

files = ["1.txt", "2.txt", "3.txt", "3_alt.txt", "4.txt", "5.txt", "nested.txt", "u8_u16_i16.txt"]

for file in files:
    f = open(file, "r")

    mean = 0
    vals = []

    print(file)

    for _ in range(200):
        val = int(f.readline())
        vals.append(val)
        mean += val

    mean /= 200
    var = 0

    for val in vals:
        var += (val - mean)**2

    var /= 199

    print(mean)
    print(math.sqrt(var))
    print("(", mean - math.sqrt(var), " - ", mean + math.sqrt(var), ")")
    print("-----------")
