PITCH_NAMES = [
    "C",
    "C♯/D♭",
    "D",
    "D♯/E♭",
    "E",
    "F",
    "F♯/G♭",
    "G",
    "G♯/A♭",
    "A",
    "A♯/B♭",
    "B",
]
PITCH_INTS = {
    "C": 0,
    "C♯/D♭": 1,
    "C♯": 1,
    "D♭": 1,
    "C#": 1,
    "Db": 1,
    "D": 2,
    "D♯/E♭": 3,
    "D♯": 3,
    "E♭": 3,
    "D#": 3,
    "Eb": 3,
    "E": 4,
    "F": 5,
    "F♯/G♭": 6,
    "F♯": 6,
    "G♭": 6,
    "F#": 6,
    "Gb": 6,
    "G": 7,
    "G♯/A♭": 8,
    "G♯": 8,
    "A♭": 8,
    "G#": 8,
    "Ab": 8,
    "A": 9,
    "A♯/B♭": 10,
    "A♯": 10,
    "B♭": 10,
    "A#": 10,
    "Bb": 10,
    "B": 11,
}


def icv(ps):
    """
    Returns the interval-class vector, in 12-TET, of a finite iterable of
    integers ``ps``.
    """
    ps = list({p % 12 for p in ps})
    v = [0] * 6

    for i in range(len(ps)):
        for j in range(i + 1, len(ps)):
            ic = abs(ps[i] - ps[j])
            if ic > 6:
                ic = 12 - ic
            v[ic - 1] += 1

    return v


def ints_to_pitches(offset, flat, ints):
    try:
        offset = PITCH_INTS[offset]
    except:
        pass

    def accidental(flat, slash_p):
        if flat:
            try:
                return slash_p.split("/")[1]
            except:
                return slash_p
        return slash_p.split("/")[0]

    return [accidental(flat, PITCH_NAMES[(i + offset) % 12]) for i in ints]


def pitches_to_ints(offset, ps):
    try:
        offset = PITCH_INTS[offset]
    except:
        pass

    return [(PITCH_INTS[p] - offset) % 12 for p in ps]


def M(k, ps):
    return [(p * k) % 12 for p in ps]


def C(k, xs):
    k = -(k % len(xs))
    return xs[k:] + xs[:k]


if __name__ == "__main__":
    # 16th notes in the prime form of the subject
    prime_ds = [1, 2, 3, 1, 2, 1, 2, 2, 2, 1, 2, 1, 1, 2, 3, 1, 2, 1, 1, 1]
    assert sum(prime_ds) == 32

    cycled8 = C(8, prime_ds)
    print(cycled8)
    assert sum(cycled8) == 32

    # Pitches immediately after the first Rₚ
    print()
    ps = [2, 1, 2, 4, 2, 0, 9, 2, 4, 2, 1, 4, 1, 6, 2, 1, 11, 4, 9, 1]
    assert len(ps) == 20
    m5 = M(5, ps)
    assert len(m5) == 20

    print(m5)
    print(", ".join(ints_to_pitches("G#", False, m5)))

    # Interval-class vectors
    print()
    print(icv([0, 1, 4, 5, 7, 9, 10]))  # Harmonic major scale
    print(icv([0, 1, 3, 4, 6, 7, 9, 10]))  # Diminished scale
    print(icv([0, 2, 4, 5, 7, 9, 11]))  # Diatonic scale
    print(icv([0, 1, 4, 6]))  # All-interval tetrachord (4-z15)
    print(icv([0, 1, 3, 7]))  # All-interval tetrachord (4-z29)

    # 16th notes immediately after the first M₅
    print()
    cycled16 = C(16, cycled8)
    print(cycled16)
    assert sum(cycled16) == 32

    # Pitches immediately after the first Rₜ
    print()
    m7 = M(7, m5)
    assert len(m7) == 20

    print(m7)
    print(", ".join(ints_to_pitches("G#", False, m7)))
