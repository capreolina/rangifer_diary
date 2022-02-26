from math import gcd


def necklace_count(n, k):
    """https://oeis.org/A054631"""
    return sum(k ** gcd(j, n) for j in range(1, n + 1)) // n


def bracelet_count(n, k):
    """
    * https://oeis.org/A081720
    * https://en.wikipedia.org/wiki/Necklace_(combinatorics)#Number_of_bracelets
    """
    if n % 2 == 0:
        return (necklace_count(n, k) + ((k + 1) * (k ** (n // 2))) // 2) // 2
    else:
        return (necklace_count(n, k) + k ** ((n + 1) // 2)) // 2


if __name__ == "__main__":
    # See "EXAMPLE" at <https://oeis.org/A081720>
    for n in range(1, 9):
        print(
            ", ".join(str(bracelet_count(n, k)) for k in range(1, n + 1)) + ";"
        )
    print()

    print(bracelet_count(20, 12))
    print(bracelet_count(20, 7))
    print(bracelet_count(20, 3))
    print(bracelet_count(20, 20))
    print(bracelet_count(12, 2))
