import math

def load():
    files = ["load.txt", "load_no_parse.txt"]

    for file in files:
        f = open(file, "r")

        for _ in range(700):
            f.readline()
            f.readline()
            data = f.readline()

            start, end = data.split(", ")
            diff = int(end) - int(start)

            if diff != 0:
                print(diff)

        print("-------")

def ble():
    files = ["ble.txt", "ble_100.txt", "ble_no_parse.txt", "ble_no_parse_100.txt"]

    for file in files:
        f = open(file, "r")

        mean = 0
        vals = []

        print(file)

        for _ in range(100):
            data = f.readline().split(": ")[1]

            val = int(data)

            vals.append(val)
            mean += val

        mean /= 100
        var = 0

        for val in vals:
            var += (val - mean)**2

        var /= 990

        print(mean)
        print(math.sqrt(var))
        print("(", mean - math.sqrt(var), " - ", mean + math.sqrt(var), ")")
        print("-------")

def syscall():
    files = ["syscall.txt", "syscall_parse.txt", "syscall_create.txt", "syscall_100.txt", "syscall_no_parse.txt", "syscall_no_parse_100.txt"]

    for file in files:
        f = open(file, "r")

        mean = 0
        vals = []

        print(file)

        for _ in range(100):
            data = f.readline().split(": ")[1]

            val = int(data)

            vals.append(val)
            mean += val

        mean /= 100
        var = 0

        for val in vals:
            var += (val - mean)**2

        var /= 990

        print(mean)
        print(math.sqrt(var))
        print("(", mean - math.sqrt(var), " - ", mean + math.sqrt(var))
        print("-------")

# load()
ble()
syscall()
