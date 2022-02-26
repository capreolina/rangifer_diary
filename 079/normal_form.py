def invert(bits):
    inverted = 0
    for i in range(12):
        inverted |= ((bits >> i) & 1) << (11 - i)

    return inverted


def transpose(semitones, bits):
    semitones = semitones % 12
    s = 12 - semitones
    transposed = (bits & (0xFFF << s)) >> s
    transposed |= bits << semitones
    transposed &= 0xFFF

    return transposed


def bits_to_list(bits):
    return [i for i in range(12) if bits & (1 << i)]


def to_bits(iterable):
    bits = 0
    for i in iterable:
        bits |= 1 << (i % 12)

    return bits


def normal_form(pc_set, invertible=True):
    bits = True
    try:
        pc_set_inv = invert(pc_set)
    except:
        bits = False
        pc_set = to_bits(pc_set)
        pc_set_inv = invert(pc_set)

    best = 0xFFF
    pcss = [pc_set, pc_set_inv] if invertible else [pc_set]
    for pcs in pcss:
        for semis in range(12):
            tr = transpose(semis, pcs)
            for i in range(11, -1, -1):
                mask = 1 << i
                tr_bit, best_bit = tr & mask, best & mask
                if best_bit and not tr_bit:
                    best = tr
                elif tr_bit and not best_bit:
                    break

    if bits:
        return best
    else:
        return bits_to_list(best)


if __name__ == "__main__":
    assert normal_form([1, 4, 5, 8]) == [0, 3, 4, 7]
    assert normal_form([0, 1, 2, 4, 5, 7, 9, 10]) == [0, 1, 3, 4, 5, 7, 8, 10]
    assert normal_form([0, 1, 3, 7, 8]) == [0, 1, 5, 6, 8]
    assert normal_form([]) == []
    assert normal_form([0, 2, 4, 5, 7, 9, 11]) == [0, 1, 3, 5, 6, 8, 10]
    assert normal_form([17]) == [0]
    assert normal_form([17, 16]) == [0, 1]
    assert normal_form([0, 4, 7, 11]) == [0, 1, 5, 8]
    assert normal_form([2, 4, 6, 8, 6, 10, 12, 14]) == [0, 2, 4, 6, 8, 10]
    assert normal_form([0, 2, 5, 6, 9]) == [0, 1, 4, 6, 9]
    assert normal_form([0, 2, 5, 6, 9], False) == [0, 2, 5, 6, 9]
    assert normal_form([0, 3, 5, 8, 9], False) == [0, 2, 5, 6, 9]
    assert normal_form([0, 3, 4, 5, 7], False) == [0, 3, 4, 5, 7]
    assert normal_form([0, 3, 4, 5, 7]) == [0, 2, 3, 4, 7]

    print(normal_form([0, 2, 4, 6, 8, 10] + [1, 5, 8]))
