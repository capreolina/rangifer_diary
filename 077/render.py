import re
import sys
import toytree
import toyplot.svg

WHITESPACE = re.compile(r"\s+")
EXTENSION = re.compile(r"\.[^\.]+$")

OUTLAND_BEGINNERS = {"STRginner", "DEXginner", "LUKginner", "wandginner"}
ISLANDERS = {
    "STRlander",
    "DEXlander",
    "hybridlander",
    "magelander",
    "LUKlander",
}
WARRIORS = {
    "permawarrior",
    "HP warrior",
    "dagger warrior",
    "DEX warrior",
    "LUK warrior",
    "wand warrior",
}
MAGES = {"permamagician", "STR mage", "magelet", "DEX mage", "gish", "gishlet"}
ARCHERS = {"permarcher", "woods(wo)man", "bow-whacker", "bowginner"}
THIEVES = {
    "permarogue",
    "brigand",
    "grim reaper",
    "carpenter",
    "blood dit",
    "LUKless sin",
    "LUKless dit",
    "dagger sin",
    "clawginner",
    "claw-puncher",
}
PIRATES = {
    "permapirate",
    "swashbuckler",
    "begunner",
    "pugilist",
    "armed brawler",
    "summoner",
    "bullet bucc",
    "DEX brawler",
    "LUK bucc",
    "pistol-whipper",
    "bombadier",
    "punch slinger",
}


def rename_nodes(tn):
    if not tn.children:
        tn.name = tn.name.replace("_", " ").replace("[", "(").replace("]", ")")

    for c in tn.children:
        rename_nodes(c)


def name_color(name):
    if name == "camper":
        return "#686868"
    elif name in OUTLAND_BEGINNERS:
        return "#080808"
    elif name in ISLANDERS:
        return "#383838"
    elif name in WARRIORS:
        return "#a8221d"
    elif name in MAGES:
        return "#0d438c"
    elif name in ARCHERS:
        return "#1f8040"
    elif name in THIEVES:
        return "#905f0a"
    elif name in PIRATES:
        return "#9b59b6"

    raise ValueError(f"Odd job name not recognised: `{name}`")


if len(sys.argv) != 2:
    print(
        "Expected exactly one argument: the filename of the Newick data file",
        file=sys.stderr,
    )
    sys.exit(1)

input_filename = sys.argv[1]
with open(input_filename, "r", encoding="UTF-8") as f:
    newick_data = f.read()

newick_data = WHITESPACE.sub("", newick_data)
filename_prefix = EXTENSION.sub("", input_filename, count=1)

tree = toytree.tree(newick_data)
rename_nodes(tree.treenode)

tip_color_list = [name_color(tip_label) for tip_label in tree.get_tip_labels()]
inner_node_vals = tree.get_node_values(None, True, False)

canvas, _, _ = tree.draw(
    ts="n",
    tip_labels_align=False,
    tip_labels_colors=tip_color_list,
    node_labels=[str(i) if v else "" for i, v in enumerate(inner_node_vals)],
    node_sizes=14,
    width=512,
    height=1024,
)
toyplot.svg.render(canvas, f"{filename_prefix}-inner_labels.svg")

#### Re-rendering the usual diagrams, but now based on 00-01.tree ####

canvas, _, _ = tree.draw(
    ts="n",
    tip_labels_align=False,
    tip_labels_colors=tip_color_list,
    width=512,
    height=1024,
)
toyplot.svg.render(canvas, f"{filename_prefix}.svg")

canvas, _, _ = tree.draw(
    ts="c",
    scalebar=False,
    tip_labels=True,
    tip_labels_align=False,
    tip_labels_colors=tip_color_list,
    node_colors=[
        "#101820" if i else "#304860"
        for i in tree.get_node_values(None, True, False)
    ],
    width=1024,
    height=512,
)
toyplot.svg.render(canvas, f"{filename_prefix}-geometric.svg")

canvas, _, _ = tree.draw(
    ts="n",
    layout="c",
    edge_type="p",
    tip_labels=True,
    tip_labels_align=False,
    tip_labels_colors=tip_color_list,
    width=1024,
    height=1024,
)
toyplot.svg.render(canvas, f"{filename_prefix}-circular.svg")
