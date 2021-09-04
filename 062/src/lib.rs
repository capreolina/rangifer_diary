use rayon::prelude::*;
use rug::{Assign, Complete, Integer};

/// `fubini(n)` is the `n`th Fubini number (a.k.a. the `n`th ordered Bell
/// number).
#[inline]
pub fn fubini(n: u32) -> Integer {
    let mut a = Integer::new();
    for k in 0..=n {
        for j in 0..=k {
            a += (-1i32).pow(k - j)
                * Integer::binomial_u(k, j).complete()
                * Integer::u_pow_u(j, n).complete();
        }
    }

    a
}

/// `fubini`, but with some optimisations focussed around calculating
/// exponents.
#[inline]
pub fn fubini_pow(n: u32) -> Integer {
    let mut a = Integer::new();
    for k in 0..=n {
        for j in 0..=k {
            a += if (k ^ j) & 0x01 == 0 {
                Integer::binomial_u(k, j).complete()
            } else {
                -Integer::binomial_u(k, j).complete()
            } * Integer::u_pow_u(j, n).complete();
        }
    }

    a
}

/// `fubini` defined by `sum`ming iterators, instead of using explicit loops.
#[inline]
pub fn fubini_gen(n: u32) -> Integer {
    (0..=n)
        .map(|k| (0..=k).map(move |j| (k, j)))
        .flatten()
        .map(|(k, j)| {
            (-1i32).pow(k - j)
                * Integer::binomial_u(k, j).complete()
                * Integer::u_pow_u(j, n).complete()
        })
        .sum()
}

/// `fubini_gen`, but MOAR THREDZ!!!
#[inline]
pub fn fubini_par(n: u32) -> Integer {
    (0..=n)
        .into_par_iter()
        .map(|k| (0..=k).into_par_iter().map(move |j| (k, j)))
        .flatten()
        .map(|(k, j)| {
            (-1i32).pow(k - j)
                * Integer::binomial_u(k, j).complete()
                * Integer::u_pow_u(j, n).complete()
        })
        .sum()
}

/// `fubini_par`, but I re-implement `sum`?
#[inline]
pub fn fubini_par_hack(n: u32) -> Integer {
    (0..=n)
        .into_par_iter()
        .map(|k| (0..=k).into_par_iter().map(move |j| (k, j)))
        .flatten()
        .map(|(k, j)| {
            (-1i32).pow(k - j)
                * Integer::binomial_u(k, j).complete()
                * Integer::u_pow_u(j, n).complete()
        })
        .reduce(|| Integer::new(), |acc, x| acc + x)
}

/// Do **not** use this function. It is only here for illustrative purposes,
/// and is completely useless for any significantly large values of `n`.
///
/// Implements `fubini` using a naïve recursive method. I think the runtime is
/// Ω(n!).
#[inline]
pub fn fubini_rec_naive(n: u32) -> Integer {
    if n == 0 {
        1.into()
    } else {
        let mut a = Integer::new();
        for i in 1..=n {
            a += Integer::binomial_u(n, i).complete() * fubini_rec_naive(n - i)
        }

        a
    }
}

/// Implements `fubini` using recursion, but memoises, in order to make the
/// performance (read: asymptotic runtime behaviour) reasonable.
///
/// <https://en.wikipedia.org/wiki/Dynamic_programming>
#[inline]
pub fn fubini_rec(n: u32) -> Integer {
    let mut memo = vec![Integer::new(); (n + 1) as usize];
    memo[0].assign(1);

    fn a(m: u32, memo: &mut [Integer]) -> Integer {
        let memo_val = &memo[m as usize];
        if memo_val != &0 {
            return memo_val.clone();
        }

        let mut fubini_m = Integer::new();
        for i in 1..=m {
            fubini_m += Integer::binomial_u(m, i).complete() * a(m - i, memo);
        }

        memo[m as usize] = fubini_m.clone();

        fubini_m
    }

    return a(n, &mut memo);
}

#[cfg(test)]
mod tests {
    use super::{
        fubini, fubini_gen, fubini_par, fubini_par_hack, fubini_pow,
        fubini_rec, fubini_rec_naive,
    };
    use rug::{Assign, Integer};

    #[test]
    fn oeis_list() {
        /// <https://oeis.org/A000670/list>
        static OEIS_LIST: &[&'static str] = &[
            "1",
            "1",
            "3",
            "13",
            "75",
            "541",
            "4683",
            "47293",
            "545835",
            "7087261",
            "102247563",
            "1622632573",
            "28091567595",
            "526858348381",
            "10641342970443",
            "230283190977853",
            "5315654681981355",
            "130370767029135901",
            "3385534663256845323",
            "92801587319328411133",
            "2677687796244384203115",
        ];

        let mut a = Integer::new();

        for (n, a_str) in
            OEIS_LIST.iter().enumerate().map(|(n, s)| (n as u32, s))
        {
            a.assign(Integer::parse(a_str).unwrap());

            assert_eq!(fubini(n), a);
            assert_eq!(fubini_pow(n), a);
            assert_eq!(fubini_gen(n), a);
            assert_eq!(fubini_par(n), a);
            assert_eq!(fubini_par_hack(n), a);
            // comment out the following line if the test runs too slowly lmfao
            assert_eq!(fubini_rec_naive(n), a);
            assert_eq!(fubini_rec(n), a);
        }
    }
}
