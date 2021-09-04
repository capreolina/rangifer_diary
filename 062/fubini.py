#!/usr/bin/env python3

from math import comb as choose
import sys
import timeit


def fubini(n):
    """
    ``fubini(n)`` is the ``n``th Fubini number (a.k.a. the ``n``th ordered Bell
    number).
    """

    a = 0
    for k in range(0, n + 1):
        for j in range(0, k + 1):
            a += (-1) ** (k - j) * choose(k, j) * j ** n

    return a


def fubini_pow(n):
    """
    ``fubini``, but with some optimisations focussed around calculating
    exponents.
    """

    a = 0
    for k in range(0, n + 1):
        for j in range(0, k + 1):
            #   (-1) ** (k - j)
            # = -1 if (k - j) % 2    else 1
            # = -1 if (k - j) & 0x01 else 1
            # = -1 if (k ^ j) & 0x01 else 1
            a += (-choose(k, j) if (k ^ j) & 0x01 else choose(k, j)) * j ** n

    return a


def fubini_gen(n):
    """
    ``fubini`` defined by ``sum``ming generators, instead of using explicit
    loops.
    """

    return sum(
        sum(((-1) ** (k - j) * choose(k, j) * j ** n) for j in range(0, k + 1))
        for k in range(0, n + 1)
    )


def fubini_rec_naive(n):
    """
    Do **not** use this function. It is only here for illustrative purposes,
    and is completely useless for any significantly large values of ``n``.

    Implements ``fubini`` using a naïve recursive method. I think the runtime
    is Ω(n!).
    """

    if n == 0:
        return 1

    return sum(
        (choose(n, i) * fubini_rec_naive(n - i)) for i in range(1, n + 1)
    )


def fubini_rec(n):
    """
    Implements ``fubini`` using recursion, but memoises, in order to make the
    performance (read: asymptotic runtime behaviour) reasonable.

    <https://en.wikipedia.org/wiki/Dynamic_programming>
    """

    memo = [None] * (n + 1)
    memo[0] = 1

    def a(m):
        memo_val = memo[m]
        if memo_val is not None:
            return memo_val

        fubini_m = sum((choose(m, i) * a(m - i)) for i in range(1, m + 1))
        memo[m] = fubini_m

        return fubini_m

    return a(n)


if len(sys.argv) > 1 and sys.argv[1] == "--test":
    # <https://oeis.org/A000670/list>
    oeis_list = [
        1,
        1,
        3,
        13,
        75,
        541,
        4683,
        47293,
        545835,
        7087261,
        102247563,
        1622632573,
        28091567595,
        526858348381,
        10641342970443,
        230283190977853,
        5315654681981355,
        130370767029135901,
        3385534663256845323,
        92801587319328411133,
        2677687796244384203115,
    ]

    for n, a in enumerate(oeis_list):
        assert fubini(n) == a, f"fubini({n}) != {a}"
        assert fubini_pow(n) == a, f"fubini_pow({n}) != {a}"
        assert fubini_gen(n) == a, f"fubini_gen({n}) != {a}"
        # comment out the following line if the test runs too slowly lmfao
        assert fubini_rec_naive(n) == a, f"fubini_rec_naive({n}) != {a}"
        assert fubini_rec(n) == a, f"fubini_rec({n}) != {a}"

    sys.exit(0)

# Warmup.
timeit.repeat(lambda: fubini(100), repeat=2, number=20)
timeit.repeat(lambda: fubini_pow(100), repeat=2, number=20)
timeit.repeat(lambda: fubini_gen(100), repeat=2, number=20)
timeit.repeat(lambda: fubini_rec(100), repeat=2, number=20)

for n, number in [
    (20, 500),
    (50, 400),
    (75, 300),
    (100, 200),
    (150, 100),
    (200, 50),
    (250, 30),
]:
    # Because the result of each ``timeit.repeat`` is the number of seconds it
    # took to execute the ``f(n)`` call ``number`` times (``number=number``),
    # we divide by ``number`` to get the seconds elapsed for a single call. And
    # then we multiply by 1000 to convert that to ms (milliseconds).
    ms_multi = 1000.0 / number

    t_fubini = (
        min(timeit.repeat(lambda: fubini(n), repeat=5, number=number))
        * ms_multi
    )
    t_fubini_pow = (
        min(timeit.repeat(lambda: fubini_pow(n), repeat=5, number=number))
        * ms_multi
    )
    t_fubini_gen = (
        min(timeit.repeat(lambda: fubini_gen(n), repeat=5, number=number))
        * ms_multi
    )
    t_fubini_rec = (
        min(timeit.repeat(lambda: fubini_rec(n), repeat=5, number=number))
        * ms_multi
    )

    print(f"fubini({n}):     {t_fubini:.4f} ms")
    print(f"fubini_pow({n}): {t_fubini_pow:.4f} ms")
    print(f"fubini_gen({n}): {t_fubini_gen:.4f} ms")
    print(f"fubini_rec({n}): {t_fubini_rec:.4f} ms")
    print()

"""
t_fubini_rec_naive = min(
    timeit.repeat(lambda: fubini_rec_naive(20), repeat=3, number=40)
)
t_fubini_rec = min(timeit.repeat(lambda: fubini_rec(20), repeat=3, number=40))

print(f"fubini_rec_naive: {t_fubini_rec_naive}")
print(f"fubini_rec:       {t_fubini_rec}")

# fubini_rec_naive: 18.2337024580047
# fubini_rec:        0.0033497690019430593
"""
