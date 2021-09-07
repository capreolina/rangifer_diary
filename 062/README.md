# rangifer’s diary: pt. lxiii

## Massive tangent

In “Taxonomising odd jobs, pt. iii: Exploring the space of possible taxonomies. §4”, I talked about [weak orderings](https://en.wikipedia.org/wiki/Weak_ordering). I mentioned that weak orderings are enumerated by the [ordered Bell numbers (a.k.a. Fubini numbers)](https://en.wikipedia.org/wiki/Ordered_Bell_number), that is to say, the total number of possible weak orderings over a [set][set] of 𝑛 elements is just the 𝑛th ordered Bell number. And I asked what the 45th ordered Bell number is, because at the time of writing, [the list of odd jobs on the **Oddjobs** website](https://oddjobs.codeberg.page/odd-jobs.html) had a length of 45. [The OEIS entry](https://oeis.org/A000670) only lists the ordered Bell numbers up to 𝑛 = 20, so I had to calculate the 45th element in the sequence myself. To do this, I looked at two of the formulae that the relevant English Wikipedia article gives, and wrote some [Python][python] functions based on those formulae. Each of the Python implementations has the same observable behaviour (input a [natural number](https://en.wikipedia.org/wiki/Natural_number) 𝑛, and the function spits out the 𝑛th ordered Bell number as its output), but they differ in their approaches and in their performance characteristics (time spent computing, memory usage). Although it’s obviously not relevant to what the series is trying to do (taxonomise odd jobs), I did some very informal benchmarking to compare the Python implementations of such a `fubini(n)` function, because I’m into that kind of thing, I guess. I dunno, I was just curious.

After finishing that entry, I allowed my curiosity to get the better of me, and I continued looking at implementations of this rather simple mathematical function. Out of the Python implementations that I wrote, the implementation that most programmers would likely reach for (for its familiar use of [for loops](https://en.wikipedia.org/wiki/For_loop) and its predictable performance) would be the straightforward [imperative](https://en.wikipedia.org/wiki/Imperative_programming) implementation, which I called simply `fubini`. Knowing that Python, as a characteristically [interpreted][interpreter] language, does not [optimise](https://en.wikipedia.org/wiki/Optimizing_compiler) programs before executing them (we’re just using [CPython](https://en.wikipedia.org/wiki/CPython) here, sorry for any [PyPy](https://en.wikipedia.org/wiki/PyPy) fans out there\*), I wanted to optimise the `(-1) ** (k - j)` expression that occurs within the definition of `fubini`. I called this new function `fubini_pow`:

```python
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
```

Obviously, taking −1 to an [integer](https://en.wikipedia.org/wiki/Integer) power returns −1 if the power is odd, and +1 if the power is even. So `(-1) ** (k - j)` can be optimised to `-1 if (k - j) % 2 else 1`. But there is actually a better way to check for the [parity][parity] of `k - j`: testing the [least significant bit (LSB)](https://en.wikipedia.org/wiki/Bit_numbering#Least_significant_bit). So we can optimise to `-1 if (k - j) & 0x01 else 1`. But, because we only care about the LSB of `k - j`, we don’t really need to perform the full subtraction of `j` from `k`. A few napkin-scribbles later, and I figured out that taking the [bitwise XOR](https://en.wikipedia.org/wiki/Exclusive_or#Bitwise_operation) of `k` and `j` will have the same effect on the LSB as subtracting them. So we can optimise to `-1 if (k ^ j) & 0x01 else 1`. And, instead of multiplying `choose(k, j)` by this result (which is always either `-1` or `1`), we can avoid multiplication entirely by inlining `choose(k, j)` into each branch of the `if` statement, and using [unary numeric negation](https://en.wikipedia.org/wiki/Additive_inverse) (`-`) instead. As we will see, this series of optimisations does indeed speed up the program, although as you’d expect, the performance gain is modest.

I also changed how `fubini_rec` performs its [memoisation](https://en.wikipedia.org/wiki/Memoization). In the previous diary entry, I (somewhat embarrassingly) immediately reached for a [hash map](https://en.wikipedia.org/wiki/Hash_table), writing `memo = {0: 1}`. This is perfectly correct, and results in neat-looking code as a result of hash maps being built-in objects in Python. However, it’s kind of a dumb idea, because we know exactly what the set of keys in this map is going to be: {`0`, `1`, `2`, …, `n - 1`, `n`}. So what we actually want is an [array](https://en.wikipedia.org/wiki/Array_data_structure)! This changes the code only slightly, resulting in the following implementation:

```python
def fubini_rec(n):
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
```

And I used [`timeit`](https://docs.python.org/3/library/timeit.html) to write up a simple — but still more proper than last time — benchmarking program for these Python implementations.

I was curious to see how these functions might perform if they were written using properly-optimised and [compiled](https://en.wikipedia.org/wiki/Compiler) code, compiled down to some [x86-64](https://en.wikipedia.org/wiki/X86-64) [machine code](https://en.wikipedia.org/wiki/Machine_code) to be run natively by my CPU. I’m no expert in numeric computing, so I’m sure some folks can do better, but in my case I just reached for [Rust][rust] and tried to reasonably translate my Python code into that language. Under my belt, I had the [rug](https://crates.io/crates/rug) crate, allowing me to easily harness the power of [GMP](https://en.wikipedia.org/wiki/GNU_Multiple_Precision_Arithmetic_Library) to work with [arbitrary-precision](https://en.wikipedia.org/wiki/Arbitrary-precision_arithmetic) integers. Arbitrary precision is necessary here, as the ordered Bell numbers are a [combinatorial](https://en.wikipedia.org/wiki/Combinatorics) sequence, so they grow [really quickly](https://en.wikipedia.org/wiki/Large_numbers). In Python, we didn’t have to think about it, because Python will automatically switch to using arbitrary-precision integers under the hood, any time that an integer exceeds the capacity of a single [word][word].

In the end, my Rust implementations looked something like:

<details>
<summary>lib.rs</summary>

```rust
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
```

</details>

Naturally, the Rust implementations look a good deal noisier than their corresponding Python implementations, as we now have essentially total control over [memory management](https://en.wikipedia.org/wiki/Memory_management). One thing that we can immediately note is that the Rust implementation of `fubini_pow` isn’t very useful. It’s left there for reference, but in reality, optimising “−1 to an integer power” is a trivial task for any optimising compiler ([LLVM](https://en.wikipedia.org/wiki/LLVM), in our case). The manual optimisation that we did with Python is done here for us automatically, making the Rust versions of `fubini` and `fubini_pow` effectively identical, after [code generation][codegen].

Everything else is pretty self-explanatory here, considering that they are more or less one-to-one translations of the Python implementations. I did, however, add the `fubini_par` and `fubini_par_hack` functions, which make use of [rayon](https://crates.io/crates/rayon) to turn `fubini_gen` into a [multithreaded][multithreading] version of the same thing, with very minimal changes to the code. The low effort required to make `fubini_gen` [parallel](https://en.wikipedia.org/wiki/Parallel_computing) by slapping rayon on it was enough to [tempt me to do it](https://en.wikipedia.org/wiki/Impulsivity). So I did. That’s not to say that rayon produces a good and reasonably-close-to-optimal parallel computation of ordered Bell numbers here… again, I was just goofing off. I also realised just now (oops) that the [cloning](https://doc.rust-lang.org/std/clone/trait.Clone.html) occuring in `return memo_val.clone();` (within `fubini_rec`) is probably not necessary. Oh well.

In any case, I benchmarked these functions. As mentioned before, `timeit` was used for benchmarking the Python code, and for the Rust code, [criterion](https://crates.io/crates/criterion) was used. This probably skews the results in favour of Python a bit, as criterion will spit out its best estimation of a “representative” timing (criterion uses fairly rigourous statistical methodology), whereas our Python benchmarking just takes the best time of five trials (each trial being the [arithmetic mean](https://en.wikipedia.org/wiki/Arithmetic_mean) of usually a few hundred iterations). That being said, the results are at least reasonably comparable. The benchmarking was done on an [Intel](https://en.wikipedia.org/wiki/Intel) i7-4510U CPU running at 2\.00 G[Hz](https://en.wikipedia.org/wiki/Hertz); this is a [laptop](https://en.wikipedia.org/wiki/Laptop) processor, although I did not have anything else of significance (not even any [display server](https://en.wikipedia.org/wiki/Windowing_system) whatsoever — completely [headless](https://en.wikipedia.org/wiki/Headless_computer) and accessed only via [SSH](https://en.wikipedia.org/wiki/Secure_Shell)) running concurrently with the benchmarks.

<details>
<summary>Benchmarking results</summary>

The x-axis here is the `n` value, and the y-axis is the runtime (in [ms](https://en.wikipedia.org/wiki/Millisecond)) of `f(n)`, for the given implementation of `f`. All graphs were generated using [gnuplot](https://en.wikipedia.org/wiki/Gnuplot), with the [Qt][qt] backend.

![Fubini benchmarks (both Python and Rust)](fubini.svg "Fubini benchmarks (both Python and Rust)")

Looking at the Python implementations, `fubini` and `fubini_gen` appear to be tied — neither one is consistently faster than the other, and they always are very similar. This may come as no surprise, as they are essentially the same implementation, but it’s good to know that using generators in this way has no overhead in Python! Coming in consistently faster (albeit only mildly faster) is `fubini_pow`, proving that the hand-optimised −1-exponentiation did help a bit. And then, considerably faster is `fubini_rec`, running roughly ≈1\.26 times faster than `fubini_pow` for `n = 250`.

Looking at the Rust implementations, it’s immediately clear that they are (as expected) far faster than the corresponding Python ones. In fact, they’re so much faster that it’s difficult to distinguish the Rust implementations from one another in this graph. For that, we have the same graph, but with the Python implementations removed:

![Fubini benchmarks (Rust only)](fubini-rust.svg "Fubini benchmarks (Rust only)")

Like with Python, `fubini` and `fubini_gen` are tied. Again, not surprising, but it’s good to know that [rustc](https://doc.rust-lang.org/rustc/what-is-rustc.html) & LLVM are collectively smart enough to compile the two functions down to the same thing. And, lo & behold, our parallel rayon-based implementations of `fubini_gen` (viz. `fubini_par` & `fubini_par_hack`) do achieve some measurable speedup compared to their sequential counterparts. The speedup over `fubini_gen` for `n = 250` is a factor of roughly ≈1\.49 for `fubini_par` and ≈1\.57 for `fubini_par_hack`. Keep in mind that the number of CPUs here is four. And, as I kind of suspected, `fubini_par_hack` does seem to be a bit faster than `fubini_par`, even though it just re-implements [`.sum()`](https://docs.rs/rayon/1.5.1/rayon/iter/trait.ParallelIterator.html#method.sum). I would have to actually investigate to see why, but I assume that rug’s implementation of [the `Sum` trait](https://doc.rust-lang.org/std/iter/trait.Sum.html) is not playing nicely with what rayon is trying to do. But, in any case, it seems that just slapping an [`.into_par_iter()`](https://docs.rs/rayon/1.5.1/rayon/iter/trait.IntoParallelIterator.html#tymethod.into_par_iter) or two onto `fubini_gen` is not enough (at least, not with just four CPUs) to beat `fubini_rec`. Like with Python, `fubini_rec` reigns supreme.

</details>

Without comparing all of the numbers here, we can compare the two implementations (Python vs. Rust) of `fubini_rec` (the fastest version within either language) at `n = 250` to get a vague idea of how much time (not to mention memory) we save by switching from Python to Rust: a factor of roughly ≈21\.9. Whew!

Alright, enough useless [software engineering](https://en.wikipedia.org/wiki/Software_engineering) nonsense for now.

<details>
<summary>Footnotes for “Massive tangent”</summary>

\*Testing with PyPy does (as expected) give faster results across the board. Interestingly enough, the performance gains are considerably higher for `fubini_rec` in particular, for smaller values of `n`. It seems PyPy finds some good optimisation that ends up being dominated by the bulk of the arithmetic computation, as `n` gets larger. Maybe some inlining to reduce function call overhead, or better implementation of the cache array. Who knows! As it turns out, PyPy is roughly ≈1\.33 times faster than CPython for `fubini_rec(250)`. For `fubini_rec(50)`, this factor is ≈3\.33!

</details>

## Taxonomising odd jobs, pt. iii: Exploring the space of possible taxonomies. §5

In the previous section (§4) of this part (pt. iii), I talked about weak orderings, and had this to justify it:

> The reason for considering weak orderings is that when we hand-craft our “[phylogenetic tree(s)](https://en.wikipedia.org/wiki/Phylogenetic_tree)”, we may want to start with the most skeletal structure possible: ordering our objects (our odd jobs) in roughly “chronological order” (or rather, some notion of “primitiveness”). This will certainly end up being a weak ordering, as there should be many pairs of objects where we just don’t know if one “came first”, the other one “came first”, or they emerged at “the same time”. If 𝑂 is our set of odd jobs, and we had such a pair of objects (𝑎, 𝑏) [∈][element] 𝑂 [×](https://en.wikipedia.org/wiki/Cartesian_product) 𝑂, then under our weak ordering (𝑂, ≤), it would be true that 𝑎 ≤ 𝑏 [∧](https://en.wikipedia.org/wiki/Logical_conjunction) 𝑏 ≤ 𝑎. We can then impose this ordering, later, onto our [tree][tree]… or whatever it is. We shall investigate this next time, I guess.

Well, it is now “next time”, so I want to cover this, at least somewhat.

### Trees are for treehouses

What is a tree? Well, a [tree](https://en.wikipedia.org/wiki/Tree) is a [perennial plant](https://en.wikipedia.org/wiki/Perennial_plant) with an elongated [stem/trunk][trunk], supporting [branches](https://en.wikipedia.org/wiki/Branch), and — in most species — [leaves](https://en.wikipedia.org/wiki/Leaf).

Just kidding. A tree is actually a [connected][connectivity] a[cyclic][cycle] undirected graph. Unfortunately (or fortunately…?), this means that we are going to end up in “[graph theory](https://en.wikipedia.org/wiki/Graph_theory) 101” territory before we can talk about our leafy friends. So let’s break this down a bit:

A **graph** is, informally, a bunch of _vertices_ (which are basically just… objects) that may or may not be joined to one another. Two vertices are _joined_ when there is an _edge_ joining them. Usually, when we visually represent graphs, we represent a vertex with a circle, and we represent an edge with a line that connects two circles. But graphs are really abstract, so they can represent a lot of things, not just circles and lines. Formally, a graph 𝐺 is a pair 𝐺 = (𝑉, 𝐸), where 𝑉 is a set of objects called _vertices_, and 𝐸 is a set whose members are sets of [cardinality](https://en.wikipedia.org/wiki/Cardinality) exactly 2, where each member of such a 2-set is also a member of 𝑉. These 2-sets are called _edges_.

An **undirected graph** is the same thing as what I defined as a “graph” above. The reason for specifying that a graph is “undirected” is to clearly distinguish from a _directed_ graph, which is a graph whose edges have a specified direction, from one vertex to the other. When we visually represent directed graphs, we usually show the direction of an edge by adding an arrowhead to the corresponding one of its ends (so it looks like “→” or “←”). Formally, a directed graph is defined similarly to an undirected graph, except that instead of defining 𝐸 in terms of 2-sets, 𝐸 [⊆](https://en.wikipedia.org/wiki/Subset) {(𝑥, 𝑦) ∈ 𝑉 × 𝑉 [|](https://en.wikipedia.org/wiki/Set-builder_notation) 𝑥 ≠ 𝑦}.

An **acyclic graph** is a graph that has no cycles. If you have a vertex 𝑥, and you can walk from 𝑥 back to 𝑥 along one or more edges of the graph, without walking along any given edge more than once, that’s called a _cycle_. So if you’re walking along an acyclic graph, then you cannot leave your current vertex, and then return back to that vertex, without re-treading any edges. Formally, a _walk_ on a graph 𝐺 = (𝑉, 𝐸) is a sequence of edges ⟨𝑒₁, 𝑒₂, …, 𝑒ₙ₋₂, 𝑒ₙ₋₁⟩ (where 𝑒ᵢ ∈ 𝐸) such that there exists a sequence of vertices ⟨𝑣₁, 𝑣₂, …, 𝑣ₙ₋₁, 𝑣ₙ⟩ (where 𝑣ᵢ ∈ 𝑉), so that for all 𝑖 \< 𝑛, 𝑒ᵢ = {𝑣ᵢ, 𝑣ᵢ₊₁}. (For directed graphs, we can just change that last equality to 𝑒ᵢ = (𝑣ᵢ, 𝑣ᵢ₊₁).) A _trail_ is a walk where 𝑖 ≠ 𝑗 [→](https://en.wikipedia.org/wiki/Material_conditional) 𝑒ᵢ ≠ 𝑒ⱼ. A _circuit_ is a nonempty trail where 𝑣₁ = 𝑣ₙ. A _cycle_ is a circuit where 𝑣ᵢ = 𝑣ⱼ [iff](https://en.wikipedia.org/wiki/If_and_only_if) 𝑣ᵢ = 𝑣ⱼ = 𝑣₁ = 𝑣ₙ. And, since we’re talking about trails, a _path_ is a trail where 𝑖 ≠ 𝑗 [→](https://en.wikipedia.org/wiki/Material_conditional) 𝑣ᵢ ≠ 𝑣ⱼ.

A **connected graph** is a graph in which every vertex is connected to every other vertex. A pair of vertices is _connected_ if you can walk, along edges, from one to the other. If a graph has one or more pairs of vertices that are not connected (that are _disconnected_), then the graph is a _disconnected graph_. Every graph is either connected, or else disconnected. Formally, two vertices (𝑣ᵢ, 𝑣ⱼ) ∈ 𝑉 × 𝑉 of a graph 𝐺 = (𝑉, 𝐸) are _connected_ when there exists a walk on 𝐺 whose vertex sequence is ⟨𝑣ᵢ, …, 𝑣ⱼ⟩. Two vertices (𝑣ᵢ, 𝑣ⱼ) ∈ 𝑉 × 𝑉 of a graph 𝐺 = (𝑉, 𝐸) are _disconnected_ iff they are not connected. A graph 𝐺 = (𝑉, 𝐸) is a _connected graph_ when, for all (𝑣ᵢ, 𝑣ⱼ) ∈ 𝑉 × 𝑉, (𝑣ᵢ, 𝑣ⱼ) is connected. A graph is a _disconnected graph_ iff it is not a connected graph. For directed graphs, we can define a weaker version of connectivity, _weakly connected_, which describes two vertices that would be connected if all edges in the graph became undirected (via a function that maps any edge (𝑣ᵢ, 𝑣ⱼ) [↦][map] {𝑣ᵢ, 𝑣ⱼ}).

When a graph is acyclic, as defined above, there is at most one path between any given pair of vertices. The reason is that, if there were two distinct paths 𝑝₁ ≠ 𝑝₂ between a pair of vertices (𝑣₁, 𝑣₂), we could take 𝑝₁ from 𝑣₁ to 𝑣₂, and then 𝑝₂ to go from 𝑣₂ to 𝑣₁. And in there somewhere, would be a cycle! An informal proof of this might look like: If the two paths share no edges, then chaining (i.e. connecting end-to-end) the paths would form a cycle (or, if it forms a circuit instead, that’s okay — every circuit contains at least one cycle). If the two paths do share edges, then we can manipulate our graph 𝐺 to produce a new graph 𝐺′ in which all of these shared edges have been [contracted](https://en.wikipedia.org/wiki/Edge_contraction). Edge contraction preserves paths in the sense that any path in 𝐺 that connects two of its vertices also exists in 𝐺′, albeit with some edges possibly removed from the sequence — remember that empty (i.e. length 0) paths are perfectly valid. And there are still two distinct paths 𝑝₁′ ≠ 𝑝₂′ that connect our vertices (𝑣₁′, 𝑣₂′), as the paths were distinct before, so they could not possibly have shared all of their edges. But now, thanks to the edge contraction, chaining 𝑝₁′ with 𝑝₂′ definitely forms a cycle — 𝑝₁′ and 𝑝₂′ share no edges. This means that 𝐺′ is cyclic. If 𝐺′ is cyclic, then 𝐺 must be cyclic too, as the only thing that we did to go from 𝐺 to 𝐺′ is contract some edges, which cannot create new cycles, as it does not create new trails (it just shortens them). [By contradiction](https://en.wikipedia.org/wiki/Proof_by_contradiction), any acyclic graph must have at most one path between any given pair of vertices.

If a graph is acyclic _and_ connected (i.e. a tree), then we can strengthen “at most one path between any given pair of vertices” to “_exactly_ one path between any given pair of vertices”. Having less than one (i.e. zero) paths between a pair of vertices is impossible, as all such pairs are connected! This stronger condition is, essentially, what it means for a graph to be a tree.

### Treerarchy

As they are, trees are a tad bit abstract. We can, however, impose some additional structure.

A _rooted tree_ is a tree in which exactly one of the vertices has been designated as the _root_. Every rooted tree 𝐺 = (𝑉, 𝐸) has an associated _tree-order_, which is a [partial order](https://en.wikipedia.org/wiki/Partially_ordered_set) over 𝑉 — call it (𝑉, ≤) — such that 𝑣₁ ≤ 𝑣₂ iff the path from the root to 𝑣₂ passes through 𝑣₁. This path from the root is guaranteed to exist and to be unique, because of the properties of trees discussed above.

Another ordering that we can impose on a rooted tree is that of depth. The _depth_ of a vertex is the length of the path between it and the root. Naturally, this produces a weak ordering ((𝑉, ≲) such that 𝑣₁ ≲ 𝑣₂ iff the depth of 𝑣₁ is less than or equal to the depth of 𝑣₂), as depth values are natural numbers, and two distinct vertices can possibly have the same depth. In the special case of phylogenetic trees, this ordering can have a special meaning when paired with a [molecular clock](https://en.wikipedia.org/wiki/Molecular_clock) hypothesis: each depth value (or rather, the [equivalence class](https://en.wikipedia.org/wiki/Equivalence_class) associated with such a depth value) is essentially an evolutionary generation, and larger values correspond to times that are further in the future (see: [ultrametric](https://en.wikipedia.org/wiki/Ultrametric_space) trees). However, in our case (taxonomising odd jobs), this depth-order won’t really have a meaning (especially not for our hand-crafted trees).

Although depth-order may not be quite so useful, the tree-order is what we want to combine with the hypothetical weak ordering of our set of odd jobs that was mentioned in §4 (and was quoted above). Let 𝑂 be our set of odd jobs, and let (𝑂, ≲) be our weak ordering of odd jobs by “primitiveness” (or whatever), where 𝑜₁ ≲ 𝑜₂ (𝑜ᵢ ∈ 𝑂) is interpreted as “𝑜₁ is more ‘primitive’ than 𝑜₂”. Suppose that we produce a single rooted tree 𝑇 = (𝑂, 𝐸), which has a tree-order (𝑂, ≤). **Then we want to maintain the following invariant:** 𝑜₁ ≤ 𝑜₂ → 𝑜₁ ≲ 𝑜₂. I should stress that this is a one-way implication, **not** an iff.

### Forestry

We can lift the connectedness requirement on our definition of trees, resulting in simply an acyclic undirected graph (that is not necessarily connected). This is known as a _forest_, because such a graph can be thought of as the [disjoint union](https://en.wikipedia.org/wiki/Disjoint_union_of_graphs) of zero or more trees. Formally, the _disjoint union_ of two graphs 𝐺₁ = (𝑉₁, 𝐸₁) and 𝐺₂ = (𝑉₂, 𝐸₂) is another graph 𝐺₁ + 𝐺₂ = (𝑉₁ [⊔](https://en.wikipedia.org/wiki/Disjoint_union) 𝑉₂, 𝐸₁ ⊔ 𝐸₂). Taking the disjoint union of two or more nonempty trees necessarily results in a disconnected graph, despite the fact that each tree is itself, by definition, connected.

Forests could be useful for our case, because they allow for two objects to be well and truly unrelated, by simply being disconnected within the graph representation. This would correspond, in the biological case, to two species who have no [common ancestor(s)](https://en.wikipedia.org/wiki/Common_descent) whatsoever. In the biological case, there is, empirically, apparently no such thing — it is commonly understood that all known lifeforms ultimately decend from a [last universal common ancestor](https://en.wikipedia.org/wiki/Last_universal_common_ancestor), which makes every species part of a colossal phylogenetic tree spanning some 3 to 5 billion years or so. In our case, however, it may very well make sense to have a pair of odd jobs be completely unrelated.

Because we want our trees to be rooted, we will end up with a _rooted forest_, which is just a disjoint union of zero or more rooted trees. Let 𝑇₁, 𝑇₂, …, 𝑇ₙ₋₁, 𝑇ₙ be our rooted trees, let ℐ = {𝑖 ∈ ℕ | 1 ≤ 𝑖 ≤ 𝑛} be the [index set](https://en.wikipedia.org/wiki/Index_set) for our collection of trees, and let 𝐹 = ⨆𝑇ᵢ (for each 𝑖 ∈ ℐ) be our rooted forest. Each rooted tree 𝑇ᵢ = (𝑂ᵢ, 𝐸ᵢ) has a vertex set 𝑂ᵢ ⊆ 𝑂, as well as a tree-order (𝑂ᵢ, ≤ᵢ). Then we can generalise the invariant above to: [∀](https://en.wikipedia.org/wiki/Universal_quantification)𝑖 ∈ ℐ ∀(𝑜, 𝑝) ∈ 𝑂ᵢ[²](https://en.wikipedia.org/wiki/Cartesian_product) \[𝑜 ≤ᵢ 𝑝 → 𝑜 ≲ 𝑝\].

[set]: https://en.wikipedia.org/wiki/Set_(mathematics)
[python]: https://en.wikipedia.org/wiki/Python_(programming_language)
[interpreter]: https://en.wikipedia.org/wiki/Interpreter_(computing)
[parity]: https://en.wikipedia.org/wiki/Parity_(mathematics)
[rust]: https://en.wikipedia.org/wiki/Rust_(programming_language)
[word]: https://en.wikipedia.org/wiki/Word_(computer_architecture)
[multithreading]: https://en.wikipedia.org/wiki/Multithreading_(computer_architecture)
[codegen]: https://en.wikipedia.org/wiki/Code_generation_(compiler)
[qt]: https://en.wikipedia.org/wiki/Qt_(software)
[element]: https://en.wikipedia.org/wiki/Element_(mathematics)
[tree]: https://en.wikipedia.org/wiki/Tree_(graph_theory)
[connectivity]: https://en.wikipedia.org/wiki/Connectivity_(graph_theory)
[cycle]: https://en.wikipedia.org/wiki/Cycle_(graph_theory)
[trunk]: https://en.wikipedia.org/wiki/Trunk_(botany)
[intersection]: https://en.wikipedia.org/wiki/Intersection_(set_theory)
[map]: https://en.wikipedia.org/wiki/Map_(mathematics)
