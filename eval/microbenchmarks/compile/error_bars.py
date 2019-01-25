import math

files = ["1.txt", "1_alt.txt", "1_alt2.txt", "1_alt3.txt", "2.txt", "2_alt.txt", "3.txt", "4.txt", "5.txt", "no_parse.txt", "nested_no_parse.txt", "nested.txt", "u8_u16_i16.txt"]

for file in files:
    f = open(file, "r")
    
    mean = 0
    vals = []

    print(file)

    for _ in range(100):
        start = f.readline()
        end = f.readline()
        nl = f.readline()

        val = int(end) - int(start)

        if file == "2_alt.txt":
            print(val)

        vals.append(val)
        mean += val

    mean /= 100
    var = 0

    for val in vals:
        var += (val - mean)**2

    var /= 99

    print(mean)
    print(math.sqrt(var))
    print("(", mean - math.sqrt(var), " - ", mean + math.sqrt(var), ")")
    print("-----------")
