#!/usr/bin/env python3

from math import comb as choose


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

    memo = {0: 1}

    def a(m):
        if m in memo:
            return memo[m]

        fubini_m = sum((choose(m, i) * a(m - i)) for i in range(1, m + 1))
        memo[m] = fubini_m

        return fubini_m

    return a(n)


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
    assert fubini_gen(n) == a, f"fubini_gen({n}) != {a}"
    # comment out the following line if the test runs too slowly lmfao
    assert fubini_rec_naive(n) == a, f"fubini_rec_naive({n}) != {a}"
    assert fubini_rec(n) == a, f"fubini_rec({n}) != {a}"
