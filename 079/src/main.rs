use rand::{
    distributions::{weighted::WeightedIndex, Distribution},
    SeedableRng,
};
use rand_chacha::ChaCha20Rng;

// The subject of βcervineβs Themeβ has 20 notes in it.
const N: u64 = 20;

fn main() {
    // Ξ©
    let mutation_functions = [
        "π<sub>t</sub>",
        "π<sub>p</sub>",
        "π<sub>p</sub>",
        "π<sub>5</sub>",
        "π<sub>7</sub>",
        "π<sup>1</sup>",
        "π<sup>β1</sup>",
        "π<sup>2</sup>",
        "π<sup>β2</sup>",
        "π<sup>3</sup>",
        "π<sup>β3</sup>",
        "π<sup>4</sup>",
        "π<sup>β4</sup>",
        "π<sup>5</sup>",
        "π<sup>β5</sup>",
        "π<sup>6</sup>",
        "π<sup>β6</sup>",
        "π<sub>β</sub>",
        "π<sub>ΒΎ</sub>",
        "π<sub>4β3</sub>",
        "π<sub>3β2</sub>",
        "π<sub>2</sub>",
        "π<sub>Β½</sub>",
    ];
    let mut mutation_functions: Vec<_> = mutation_functions
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    for k in 1..N {
        mutation_functions.push(format!("π<sub>t</sub><sup>{}</sup>", k));
        mutation_functions.push(format!("π<sub>p</sub><sup>{}</sup>", k));
    }

    // 1β§Έπ({Ο}) for all Ο β Ξ©
    let mut p_inverses = vec![
        16, 16, 8, 16, 16, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 176, 176,
        32, 32, 32, 32, 16, 16,
    ];
    let c = 8 * N - 8;
    for _ in 1..N {
        p_inverses.push(c);
        p_inverses.push(c);
    }

    // Just making sure...
    assert_eq!(mutation_functions.len(), p_inverses.len());

    // `WeightedIndex` expects the inverse of what we have in `p_inverses`, so
    // we have to find the least common multiple of the elements of
    // `p_inverses` first.
    let p_inverses_lcm = lcm(&[8, 16, 32, 88, 176, c]);
    let weights: Vec<_> = p_inverses
        .into_iter()
        .map(|p_inv| p_inverses_lcm / p_inv)
        .collect();

    // Setting up the distribution and CSPRNG...
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = ChaCha20Rng::from_seed(
        // The last six bytes here are the UTF-8 encoding of U+2764 U+fe0f, the
        // codepoint sequence for the red heart emoji.
        *b"cervine is an I/L magelet \xe2\x9d\xa4\xef\xb8\x8f",
    );

    // Let βer rip...!
    for _ in 0..16 {
        println!("- {}", mutation_functions[dist.sample(&mut rng)]);
    }
}

/// NaΓ―ve algorithm to calculate the least common multiple of some `u64`s.
fn lcm(xs: &[u64]) -> u64 {
    assert!(!xs.is_empty());

    let mut buf = xs.to_owned();
    loop {
        let mut least = None;
        let mut all_eq = true;
        for (i, x) in buf.iter().enumerate() {
            match least {
                None => least = Some((i, x)),
                Some((_, y)) => {
                    if x < y {
                        least.replace((i, x));
                    }
                    if x != y {
                        all_eq = false;
                    }
                }
            }
        }

        if all_eq {
            return buf[0];
        }

        let (least_ix, _) = least.unwrap();
        buf[least_ix] += xs[least_ix];
    }
}
