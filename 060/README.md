# rangifer’s diary: pt. lx

## Taxonomising odd jobs, pt. iii: Exploring the space of possible taxonomies. §3

### Huh?

Good question. In the previous section of this part (pt. iii, §2), I talked about [a whole lot of abstract garbage](https://en.wikipedia.org/wiki/Waste) and didn’t have much of a point. In lieu of excusing my own [poor writing](https://en.wikipedia.org/wiki/Nonsense) and the generally [soup-like consistency][sol] of my [brain matter](https://en.wikipedia.org/wiki/Grey_matter), I offer this quotation of myself from pt. i, §1:

> I do not have the contents of this entire “Taxonomising odd jobs” series already written or planned out. This is in my diary (serialised as sections, with one section per diary entry) because of the informal, stream-of-consciousness nature of the series.

My intent is not _really_ to bog down something seemingly simple (taxonomising a handful of MapleStory jobs) with a bunch of garbage, and to turn this into a maths lesson for some god-forsaken reason. But, for better or worse, there’s heaps of garbage floating around in my mind, and [so that’s what comes out](https://en.wikipedia.org/wiki/Garbage_in,_garbage_out). Applying some mathematical methods might(‽) turn out to be interesting, and because there’s obviously no single/unique way to encode odd jobs into our desired ontology (some kind of vaguely linear space, with a metric, yadda yadda &c.), there’s a lot of abstract garbage that may (or may not) turn out to be useful. Lazily encoding things into something that [looks like](https://en.wikipedia.org/wiki/Bullshit) a linear space with a metric (even if the space is… mAyBE not actually a [vector space](https://en.wikipedia.org/wiki/Vector_space), and/or the distance function is maYbE not actually a [metric][metric]…) is pretty standard stuff in [data-mining](https://en.wikipedia.org/wiki/Data_mining)/[machine-learning](https://en.wikipedia.org/wiki/Machine_learning) contexts, as far as [I know](https://en.wikipedia.org/wiki/Hubris). In any case, it’s good enough for our purposes, and when it turns out horribly, I will be the only one to blame.

However, I like to give at least _some_ reasonable explanation, and keep things accessible to any readers who are confident enough in their [English](https://en.wikipedia.org/wiki/English_language) skills. Even in the main body of my diary entries (you know, the actual diary part…), I try to put in some redundant information to help out readers (I suspect — but cannot prove — that at least one person who isn’t me does read some of my diary entries) who aren’t scrutinising every single word that I’ve ever written in any diary entry. And, furthermore, I try to keep myself to a reasonably high standard of [accuracy](https://en.wikipedia.org/wiki/Truth). Sometimes, all of these things conspire to produce [too many words](https://en.wikipedia.org/wiki/Verbosity)!

### “““Vector space”””

So, about the whole “accuracy” thing… When I introduced vector spaces in the previous section (§2), I kind of glossed over what vector spaces actually are, and that was intentional — getting caught up in a [linear algebra](https://en.wikipedia.org/wiki/Linear_algebra) course\* is very much besides the point here. But also, I feel a little bad because I made it sound like a vector space _only_ needs vector addition and [scalar multiplication](https://en.wikipedia.org/wiki/Scalar_multiplication), subject to some constraints on how these operations work. But, to be a bit more precise, the set of scalars needs to form a [field][field] in order for the space to actually be a vector space. A field is basically† just a [set][set] whose elements can be added, subtracted, multiplied, and divided by one another in the same way that [rational numbers](https://en.wikipedia.org/wiki/Rational_number) and [real numbers](https://en.wikipedia.org/wiki/Real_number) can.

There are weaker (i.e. more general) versions of vector spaces, though. [Modules][modules] are like vector spaces, except instead of being over a field, they are over a [ring][ring] — so multiplication doesn’t have to [commute](https://en.wikipedia.org/wiki/Commutative_property), and [division][division] doesn’t have to be possible (i.e. multiplicative inverses are not required). And you can generalise further by generalising the ring, e.g. to a [semiring](https://en.wikipedia.org/wiki/Semiring) (thus a [semimodule](https://en.wikipedia.org/wiki/Semimodule)). Will we have to use these more general versions at some point? Iunno.

<details>
<summary>Footnotes for “‘‘‘Vector space’’’”</summary>

\*Linear algebra forms the basic core of… probably _most_ mathematics, and most mathematically-inclined scientific/engineering disciplines as well. As a result, you could find approximately 6 gazillion perfectly good linear algebra textbooks lying around the WWW, if you were not already familiar with it, and actually did want to learn about it. Like [this one](https://personal.math.ubc.ca/~carrell/NB.pdf), for example, which I just found in roughly 1.5 seconds by searching the WWW for “linear algebra textbook”.

†bASiCalLy

</details>

### Clustering

So, basically the entire point of defining some kind of space (hopefully linear) that has a distance function (hopefully a metric) is so that we can measure the distance between our objects (our odd jobs), and then _cluster_ them in some way, to produce our taxonomy. Or rather, one of our taxonomies.

So let’s look at just a few (I promise — just a few, there’s a ton of stuff out there…) different methods, ya?

#### Neighbour-joining

One way of producing our desired [dendrogram](https://en.wikipedia.org/wiki/Dendrogram) is with [neighbour-joining](https://en.wikipedia.org/wiki/Neighbor-joining). This method is bottom-up (“agglomerative”) because it effectively starts with each object in its own cluster, and then starts joining the objects together until it materialises a fully-resolved [tree][tree] of those objects.

The algorithm starts with all of the objects connected to a central node (that is, in a star formation). Then, we calculate a special [matrix][matrix] that gives us a kind of “relative distance”, if you will, between each pair of objects — this “relative distance” takes into account not just the raw distance (defined by our handy-dandy distance function) between the two objects, but also how far the two objects are from the rest of the objects in the set. Then, we choose the pair of objects that has the lowest “relative distance”; this pair gets connected to its own brand-new node, and the new node is connected to the central node (instead of the two objects each individually being connected to the central node). Then we just rinse & repeat, calculating our special-sauce “relative distance” matrix by ignoring our newly-paired objects themselves, and instead considering their new node as effectively a new object (for the purpose of calculating the matrix). This process repeats until the tree is “fully resolved”, meaning that every object has actually been paired at some point (i.e. no objects remain connected to the unnamed “central node”).

It’s not super clear to me how to make a dendrogram out of the resulting tree, but the result _is_ definitely a tree (and one with weighted edges, to boot, because each edge traverses a well-defined distance), so it’s probably sufficient to keep track of the order in which pairs are formed, and then use edge weights to space out the dendrogram’s objects, once their arrangement reflects the order.

This algorithm has to calculate the “relative distance” value for each pair of 𝑘 objects/nodes (and there are, of course, 𝑘² such pairs)\*. This is for each pair-joining, but we have to perform 𝑛 − 3 (or 𝑛 − 2? I dunno, 𝑛 minus some constant, it doesn’t matter) such joins where 𝑛 is the number of objects. And while 𝑘 does shrink by 1 after each iteration (as each joined pair effectively gets replaced by just one new node), this basically just means that 𝑘 is roughly 𝑛 ÷ 2 on average, which means that 𝑘 = [Θ][big-small](𝑛). As a result, running the algorithm as described results in a runtime of Θ(𝑘² ⋅ 𝑛) = Θ(𝑛³). This is [polynomial][p], which [is good](https://en.wikipedia.org/wiki/Cobham%27s_thesis), although [cubic](https://en.wikipedia.org/wiki/Cubic_function) isn’t _stellar_. There are less naïve ways of doing this that run significantly faster in practice, by using heuristics, but just knowing that the naïve implementation runs in 𝑂(𝑛³) is good enough for us — it’s not like our 𝑛 is going to be all that large!

One advantage of neighbour-joining is that, while it is primarily used in biology, it doesn’t really have any evolutionary assumptions/model(s) baked in — only one distance function (one between the objects of study) is required, and that function doesn’t even have to be a metric! One weird thing, though, is that it seems to be possible for an edge to get assigned a negative weight(?)

<details>
<summary>Footnotes for “Neighbour-joining”</summary>

\*At first glance, it might look like this makes each iteration have a runtime of Θ(𝑘³), rather than Θ(𝑘²) (and thus the entire algorithm would have a runtime of Θ(𝑛⁴)). Calculating the “relative distance” between two objects/nodes 𝐚 and 𝐛 means calculating the distance between 𝐚 and _every_ object/node, and the same for 𝐛 as well — thus, each “relative distance” calculation itself should run in Θ(𝑘), yes? Well, no, because we can calculate the distance-from-every-object/node for each object/node beforehand, which takes Θ(𝑘²) time, _and then_ calculate all pairwise “relative distances” in Θ(𝑘²). So we do indeed have Θ(𝑘² + 𝑘²) = Θ(𝑘²). Hooray for [memoisation](https://en.wikipedia.org/wiki/Memoization)!

</details>

#### Linkage-based clustering

Yet another way to obtain our desired dendrogram is to look at more general [hierarchical clustering](https://en.wikipedia.org/wiki/Hierarchical_clustering) methods that rely on generic notions of “linkage”. Linkage is basically a way of deciding which clusters to merge together (or how a cluster should be split, if taking a top-down/divisive approach). We already assume that we can supply some distance function between the objects of study. But when we want to merge clusters, it seems that we need some kind of distance function between clusters, too, so that we can find the two clusters that are the “closest” to each other and merge them! In reality, what we need is not necessarily a distance function _per se_, but is referred to as a “linkage criterion”. Once we decide on our distance function, and our linkage criterion, performing the clustering (and thus generating what we need for our dendrogram) is pretty simple, in principle. Assuming we take a bottom-up approach, every object starts in its own cluster of size 1. Then, at each iteration, we merge the pair of clusters that has the strongest linkage of all cluster pairs. And we just keep doing this until everything is in one big cluster! If we keep track of all of the clusters that we generated along the way, we can arrange them into our dendrogram.

One of the downsides of these linkage-based clustering methods, in general, is that they can have a hard time dealing with [noise][noise] in the dataset. Noisy objects (i.e. [outliers](https://en.wikipedia.org/wiki/Outlier)) tend to get grouped into their own clusters, and there is no way to incorporate them as outliers of a larger cluster without inadvertently linking the desirably large clusters into undesirably coarse (very large) clusters. For our purposes, though, this likely isn’t actually a problem at all, because we are not necessarily trying to classify data into a single, crisp [partition](https://en.wikipedia.org/wiki/Partition_of_a_set) — a dendrogram may already be good enough for our purposes. Neighbour-joining is similar enough to theoretically have this problem as well, but again, the point of neighbour-joining is not to produce a single partition anyways.

The computational complexity is similar to neighbour-joining, which is perhaps unsurprising. We get a time complexity of Θ(𝑛³) in the general naïve case; this can, apparently, be improved to Θ(𝑛² log 𝑛) (which is [*Õ*][o~](𝑛²)) via the use of a [heap][heap], at the cost of considerably higher memory usage. There are two special cases (two special linkage criteria) in which more efficient algorithms are known, with Θ(𝑛²) runtimes.

Luckily for us, the English Wikipedia article for hierarchical clustering has a handy-dandy table of common linkage criteria! Let’s check some of them out.

##### Single-linkage

Perhaps the simplest linkage criterion is [single-linkage clustering](https://en.wikipedia.org/wiki/Single-linkage_clustering), which simply looks at pairs of objects — one from each cluster — and chooses the one that has the smallest (i.e. [minimum](https://en.wikipedia.org/wiki/Maxima_and_minima)) distance. This distance is then used as the “distance” between the clusters. Pretty simple, right? The main problem with this linkage criterion is that it tends to produce long & thin clusters, where clusters get grouped together just because they get close to each other at _some point_, regardless of how distant the clusters might be in other regions.

##### Complete-linkage

A similarly simple linkage criterion is defined [dually][dual] to single-linkage clustering: [complete-linkage clustering](https://en.wikipedia.org/wiki/Complete-linkage_clustering). This is the same as single-linkage clustering, except that we replace the minimum with the maximum. So the “distance” between two clusters is simply the distance between the furthest pair of members between them. This avoids the long & thin chain clusters that single-linkage clustering produces! Instead, complete-linkage clustering tends to produce similar-looking and basically “round”-ish clusters — complete-linkage always picks the clustering that has the smallest [diameter](https://en.wikipedia.org/wiki/Diameter). This is nice, although it makes complete-linkage far more sensitive to outliers; single-linkage clustering is (for better or worse) quite unaffected by outliers, as it only looks at the closest possible point of contact, whereas complete-linkage clustering will tend to _only_ look at outliers when they are present, since the outliers will tend to produce the largest distances!

##### WPGMA

**W**eighted **P**air **G**roup **M**ethod with **A**rithmetic mean (a.k.a. [WPGMA](https://en.wikipedia.org/wiki/WPGMA)) goes beyond the aforementioned two linkage criteria by taking into account not just one pair of objects between two clusters, but all such pairs. The WPGMA distance between two clusters that both have a size of 1 is obvious, and the same as the previous two linkage criteria: it’s just the distance between the two objects. Then, when we want the distance between a larger (≥2 objects) cluster 𝐂₁ [∪][union] 𝐂₂, and some other cluster 𝐂₃, we just take the [arithmetic mean](https://en.wikipedia.org/wiki/Arithmetic_mean) of the distance between 𝐂₁ and 𝐂₃, and the distance between 𝐂₂ and 𝐂₃. That is, \[𝑑(𝐂₁, 𝐂₃) + 𝑑(𝐂₂, 𝐂₃)\] ÷ 2, where 𝑑 is the WPGMA distance. Basically, the WPGMA distance between two clusters is the (arithmetic) average inter-cluster (i.e. _not_ within a cluster, but only across clusters) distance between their members.

Although this seems like an obvious linkage criterion to try, it leans somewhat more towards the realm of biology. The reason for this is that WPGMA always generates so-called “[ultrametric](https://en.wikipedia.org/wiki/Ultrametric_space) trees”, which are trees in which the distance from the root to any leaf is always the same. In particular, the term “ultrametric” has to do with the fact that any triple of [vertices][vertex] (𝑟, 𝑙₁, 𝑙₂), where 𝑟 is the root and 𝑙 is a leaf, forms an [isosceles triangle](https://en.wikipedia.org/wiki/Isosceles_triangle). Indeed, 𝑟 can more generally be any vertex such that 𝑟 is a “common ancestor” (or whatever you want to call it, in general) of 𝑙₁ and 𝑙₂. In a [phylogenetic](https://en.wikipedia.org/wiki/Phylogenetics) context, this models a [molecular clock hypothesis](https://en.wikipedia.org/wiki/Molecular_clock), in which changes to a particular [character][character] (or [gene](https://en.wikipedia.org/wiki/Gene), whatever) happen at a constant rate across all lineages.

WPGMA has the obvious advantage over single- or complete-linkage that it takes all pairwise distances between the clusters into account, instead of just one. This largely avoids the long & thin clusters of single-linkage, and the over-sensitivity to outliers of complete-linkage. But having the ultrametricity assumption built-in could potentially be a blessing… or a curse.

##### UPGMA

**U**nweighted **P**air **G**roup **M**ethod with **A**rithmetic mean (a.k.a. [UPGMA](https://en.wikipedia.org/wiki/UPGMA)) is, as the name implies, very similar to WPGMA. It shares the following characteristics with WPGMA:

- It operates via arithmetic average inter-cluster distances.
- It produces ultrametric trees.
- It does not suffer from the downfalls of single- nor complete-linkage, although it does not necessarily possess the strengths of either.

The difference with UPGMA is that, in the explanation of WPGMA above, the averaging was very simple:

> 𝑑(𝐂₁ ∪ 𝐂₂, 𝐂₃) = \[𝑑(𝐂₁, 𝐂₃) + 𝑑(𝐂₂, 𝐂₃)\] ÷ 2

But with UPGMA, we take into account the cluster sizes when performing the average:

> 𝑑(𝐂₁ ∪ 𝐂₂, 𝐂₃) = \[[|](https://en.wikipedia.org/wiki/Cardinality)𝐂₁[|](https://en.wikipedia.org/wiki/Cardinality) ⋅ 𝑑(𝐂₁, 𝐂₃) + |𝐂₂| ⋅ 𝑑(𝐂₂, 𝐂₃)\] ÷ (|𝐂₁| + |𝐂₂|)

So, it would seem at first glance that “WPGMA” and “UPGMA” are misnomers… WPGMA is “**w**eighted”, and yet it does not weight the average — instead it just adds the two cluster distances and divides by two. UPGMA is “**u**nweighted”, and yet it _does_ weight the average. But these terms are actually named for their results, not the methods by which they are achieved; because UPGMA weights its averages, the resulting linkages give equal weight (hence, unweighted) to all distances among objects.

In any case, it is unlikely that we will use WPGMA at all, because UPGMA exists. I’m not aware of any real advantages of WPGMA over UPGMA in particular, but, they are both usually mentioned alongside one another, and UPGMA can be(?) thought of as a more sophisticated WPGMA, so I thought I would introduce WPGMA first.

[sol]: https://en.wikipedia.org/wiki/Sol_(colloid)
[metric]: https://en.wikipedia.org/wiki/Metric_(mathematics)
[field]: https://en.wikipedia.org/wiki/Field_(mathematics)
[set]: https://en.wikipedia.org/wiki/Set_(mathematics)
[module]: https://en.wikipedia.org/wiki/Module_(mathematics)
[ring]: https://en.wikipedia.org/wiki/Ring_(mathematics)
[division]: https://en.wikipedia.org/wiki/Division_(mathematics)
[tree]: https://en.wikipedia.org/wiki/Tree_(graph_theory)
[matrix]: https://en.wikipedia.org/wiki/Matrix_(mathematics)
[big-small]: https://en.wikipedia.org/wiki/Big_O_notation#Family_of_Bachmann%E2%80%93Landau_notations
[p]: https://en.wikipedia.org/wiki/P_(complexity)
[heap]: https://en.wikipedia.org/wiki/Heap_(data_structure)
[o~]: https://en.wikipedia.org/wiki/Big_O_notation#Extensions_to_the_Bachmann%E2%80%93Landau_notations
[noise]: https://en.wikipedia.org/wiki/Noise_(signal_processing)
[dual]: https://en.wikipedia.org/wiki/Duality_(mathematics)
[union]: https://en.wikipedia.org/wiki/Union_(set_theory)
[vertex]: https://en.wikipedia.org/wiki/Vertex_(graph_theory)
[character]: https://en.wikipedia.org/wiki/Character_(biology)
