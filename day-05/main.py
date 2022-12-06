b = [[None]*5]*10

for y in range(10):
    for x in range(5):
        b[y][x]=(y,x)
        print(f"{y},{x}")
    print("\n")
