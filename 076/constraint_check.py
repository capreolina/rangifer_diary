import sys
import toytree


def node_distances(nds, tn, parent_dist):
    this_dist = parent_dist + tn.dist
    nds[tn.name] = this_dist

    for c in tn.children:
        node_distances(nds, c, this_dist)


if len(sys.argv) != 3:
    print(
        "Expected exactly two arguments: the weak ordering RON filename, and the Newick data filename",
        file=sys.stderr,
    )
    sys.exit(1)

weak_ordering_filename, tree_data_filename = sys.argv[1], sys.argv[2]

with open(weak_ordering_filename, "r", encoding="UTF-8") as f:
    # Cheap & fragile hack... don't @ me
    raw_weak_ordering = eval(f.read().replace("//", "#"))
    # Converting to a better data structure for this purpose
    weak_ordering = {}
    for i, equiv_class in enumerate(raw_weak_ordering):
        for job in equiv_class:
            weak_ordering[
                # Converting the job names into their Newick-safe versions...
                job.replace(" ", "_").replace("(", "[").replace(")", "]")
            ] = i

with open(tree_data_filename, "r", encoding="UTF-8") as f:
    tree = toytree.tree(f.read().replace("\n", " "))

nds = {}
node_distances(nds, tree.treenode, 0)

jobs = list(weak_ordering)
for job0 in jobs:
    for job1 in jobs:
        if (
            weak_ordering[job0] < weak_ordering[job1]
            and not nds[job0] <= nds[job1]
        ):
            print(f"{job0}, {job1} ({nds[job0]}, {nds[job1]})")
