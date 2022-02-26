use rand::{
    distributions::{weighted::WeightedIndex, Distribution},
    SeedableRng,
};
use rand_chacha::ChaCha20Rng;

// The subject of “cervine’s Theme” has 20 notes in it.
const N: u64 = 20;

fn main() {
    // Ω
    let mutation_functions = [
        "𝐑<sub>t</sub>",
        "𝐑<sub>p</sub>",
        "𝐈<sub>p</sub>",
        "𝐌<sub>5</sub>",
        "𝐌<sub>7</sub>",
        "𝐓<sup>1</sup>",
        "𝐓<sup>−1</sup>",
        "𝐓<sup>2</sup>",
        "𝐓<sup>−2</sup>",
        "𝐓<sup>3</sup>",
        "𝐓<sup>−3</sup>",
        "𝐓<sup>4</sup>",
        "𝐓<sup>−4</sup>",
        "𝐓<sup>5</sup>",
        "𝐓<sup>−5</sup>",
        "𝐓<sup>6</sup>",
        "𝐓<sup>−6</sup>",
        "𝐒<sub>⅔</sub>",
        "𝐒<sub>¾</sub>",
        "𝐒<sub>4⁄3</sub>",
        "𝐒<sub>3⁄2</sub>",
        "𝐄<sub>2</sub>",
        "𝐄<sub>½</sub>",
    ];
    let mut mutation_functions: Vec<_> = mutation_functions
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    for k in 1..N {
        mutation_functions.push(format!("𝐂<sub>t</sub><sup>{}</sup>", k));
        mutation_functions.push(format!("𝐂<sub>p</sub><sup>{}</sup>", k));
    }

    // 1⧸𝑃({ω}) for all ω ∈ Ω
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

    // Let ’er rip...!
    for _ in 0..16 {
        println!("- {}", mutation_functions[dist.sample(&mut rng)]);
    }
}

/// Naïve algorithm to calculate the least common multiple of some `u64`s.
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
