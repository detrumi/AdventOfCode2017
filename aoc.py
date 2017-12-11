import itertools
import math
import sys

from functools import reduce
from operator import xor

def day_1():
    digits = sys.argv[2]

    # Part 1
    repeating = [int(d1) for (d1,d2) in zip(digits, digits[-1:] + digits)
        if d1 == d2]
    print(sum(repeating))

    # Part 2
    half = len(digits) // 2
    repeating = [int(d1) for (d1,d2) in zip(digits, digits[-half:] + digits[:half])
        if d1 == d2]
    print(sum(repeating))

def day_2():
    checksum = 0
    checksum2 = 0
    for line in sys.stdin:
        if line == '\n': break
        nums = [int(i) for i in line.split()]
        checksum += max(nums) - min(nums)

        checksum2 += sum(a // b for (a,b) in itertools.combinations(nums, 2) if a % b == 0)
        checksum2 += sum(b // a for (a,b) in itertools.combinations(nums, 2) if b % a == 0)

    print(checksum)
    print(checksum2)

def day_3():
    n = int(sys.argv[2])

    # Part 1
    ring = math.ceil(math.sqrt(float(n))) // 2
    inner_grid_size = (ring * 2 - 1) ** 2
    right_centered = inner_grid_size + ring
    side_distance = abs(n - right_centered) % max(1, 2 * ring)
    print(ring + side_distance)

    # Part 2
    values = [1]
    i = 1
    while values[i - 1] <= n:
        i += 1

        ring = math.ceil(math.sqrt(float(i))) // 2 # 0,1,2...
        inner_grid_size = (ring * 2 - 1) ** 2 # 1,9,25...
        inner_side_length = 2 * (ring - 1) # 0,2,4,6...
        side, pos = divmod(i - (inner_grid_size + 1), 2 * ring) # (0..3, 0..2*ring)
        inner_corner = inner_grid_size + 1 - (4 - side) * inner_side_length # 1,2,4,6,8,10,14,18,22,27...

        val = values[i - 2] # last value
        if side > 0 and pos == 0: # corner
            val += values[i - 3]
        if side == 3 and pos >= inner_side_length: # first value of current ring
            val += values[inner_grid_size]
        if ring == 1 and i > 2:
            val += 1
        else:
            if pos > 0: # previous inner value
                if side == 0 and pos == 1:
                    val += values[i - 3]
                else:
                    val += values[inner_corner + pos - 3]
            if pos <= inner_side_length and (side > 0 or pos > 0): # inner value
                val += values[inner_corner + pos - 2]
            if pos <= inner_side_length - 1: # next inner value
                val += values[inner_corner + pos - 1]
        values.append(val)
    print(values[i - 1])

def day_4():
    num_valid_1 = 0
    num_valid_2 = 0
    for line in sys.stdin:
        if line == '\n': break
        words = line.split()
        if all(a != b for a,b in itertools.combinations(words, 2)):
            num_valid_1 += 1
        if all(sorted(list(a)) != sorted(list(b)) for a,b in itertools.combinations(words, 2)):
            num_valid_2 += 1
    print(num_valid_1)
    print(num_valid_2)

def day_5():
    jumps_1 = []
    jumps_2 = []
    for line in sys.stdin:
        if line == '\n': break
        jumps_1.append(int(line))
        jumps_2.append(int(line))

    pc = 0
    for steps_1 in range(sys.maxsize):
        jumps_1[pc] += 1
        pc += jumps_1[pc] - 1
        if pc >= len(jumps_1):
            break
    print(steps_1 + 1)

    pc = 0
    for steps_2 in range(sys.maxsize):
        old_pc = pc
        pc += jumps_2[old_pc]
        if jumps_2[old_pc] >= 3:
            jumps_2[old_pc] -= 1
        else:
            jumps_2[old_pc] += 1
        if pc >= len(jumps_2):
            break
    print(steps_2 + 1)

def day_6():
    banks = [int(bs) for bs in input().split()]
    seen_configurations = []
    for cycles in range(sys.maxsize):
        max_blocks = max(banks)
        max_index = banks.index(max_blocks)
        split, left = divmod(max_blocks, len(banks))
        banks[max_index] = 0
        banks = [b + split for b in banks]
        i = max_index
        while left > 0:
            i = (i + 1) % len(banks)
            banks[i] += 1
            left -= 1
        if banks in seen_configurations:
            print(cycles + 1)
            print(len(seen_configurations) - seen_configurations.index(banks))
            break
        seen_configurations.append(list(banks))

def day_7():
    def parse_node(line):
        parts = [s.strip() for s in line.split('->')]
        node = parts[0].split()
        children = []
        if len(parts) > 1:
            children = parts[1].split(', ')
        return node[0], int(node[1].strip('()')), { c: None for c in children }

    nodes = dict()
    for line in sys.stdin:
        if line == '\n': break
        name, weight, children = parse_node(line)
        for n in children.keys(): # Check nodes for children
            node = nodes.get(n)
            if node != None:
                children[n] = node
                del nodes[n]

        def place_node(ns): # check if node is child of other
            for cs in (n[1] for n in ns.values() if n != None):
                try:
                    if cs[name] == None:
                        cs[name] = (weight, children)
                        return True
                except KeyError:
                    if place_node(cs): return True
            return False

        if not place_node(nodes):
            nodes[name] = (weight, children)

    root = nodes.popitem()
    print(root[0]) # Part 1

    def find_imbalance(node):
        weights = [find_imbalance(n) for n in node[1][1].items()]
        for i, w in enumerate(weights):
            i_1 = (i + 1) % len(weights)
            if w != weights[i_1]:
                diff = w - weights[i_1]
                if w != weights[(i + 2) % len(weights)]: # w is odd one out
                    imbalanced_node = list(node[1][1].items())[i]
                    diff = -diff
                else: # next weight is odd one out
                    imbalanced_node = list(node[1][1].items())[i_1]
                print(imbalanced_node[1][0] + diff)
                break
        return node[1][0] + sum(weights)

    find_imbalance(root) # Part 2

def day_8():
    registers = dict()
    highest_seen = -sys.maxsize
    for line in sys.stdin:
        if line == '\n': break
        r,op,n,_,r2,op2,n2 = line.split()
        if eval(str(registers.get(r2, 0)) + op2 + n2):
            sign = 1
            if op == 'dec':
                sign = -1
            registers[r] = registers.get(r, 0) + sign * int(n)
            if registers[r] > highest_seen:
                highest_seen = registers[r]
    print(max(v for k,v in registers.items()))
    print(highest_seen)

def day_9():
    score = 0
    with open('day_9_input.txt') as f:
        line = f.readline()
    i = 0
    depth = 0
    garbage = False
    garbage_count = 0
    while i < len(line):
        c = line[i]
        if garbage:
            if c == '!':
                i += 1
            elif c == '>':
                garbage = False
            else:
                garbage_count += 1
        else:
            if c == '{':
                depth += 1
                score += depth
            elif c == '}':
                depth -= 1
            elif c == '<':
                garbage = True
        i += 1
    print(score)
    print(garbage_count)

def day_10():
    def knot_hash(lengths, rounds):
        values = list(range(256))
        i = 0
        skip = 0
        for round in range(rounds):
            for l in lengths:
                values = values[i:] + values[:i]
                values[0:l] = reversed(values[0:l])
                values = values[256-i:] + values[:256-i]
                i = (i + l + skip) % 256
                skip += 1
        return values

    if ',' in sys.argv[2]: # Part 1
        values = knot_hash((int(l) for l in sys.argv[2].split(',')), 1)
        print(values[0] * values[1])

    values = knot_hash(list(ord(l) for l in sys.argv[2]) + [17,31,73,47,23], 64)
    print(values)
    print(''.join(hex(reduce(xor, values[i:i+16]))[2:] for i in range(0, 256, 16)))

def day_11():
    def get_distance(x, y):
        distance = abs(y) + abs(x)
        if (x > 0 and y < 0) or (x < 0 and y > 0):
            distance -= min(abs(x), abs(y))
        return distance

    with open('day_11_input.txt') as f:
        steps = f.readline().strip().split(',')
    x = 0
    y = 0
    max_distance = 0
    for step in steps:
        if step == 'n':
            y += 1
        elif step == 'ne':
            x += 1
        elif step == 'se':
            y -= 1
            x += 1
        elif step == 's':
            y -= 1
        elif step == 'sw':
            x -= 1
        elif step == 'nw':
            y += 1
            x -= 1
        max_distance = max(max_distance, get_distance(x, y))
    print(get_distance(x, y))
    print(max_distance)


def main():
    day = sys.argv[1]
    globals()["day_" + day]()

main()
