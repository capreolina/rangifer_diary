import toytree
import toyplot.svg

tree = toytree.tree("./sample_weighted.tree")

canvas, _, _ = tree.draw(
    ts="c",
    scalebar=False,
    tip_labels=True,
    tip_labels_align=False,
    node_colors=[
        "#3360a0" if i else "#6655a0"
        for i in tree.get_node_values(None, True, False)
    ],
)
toyplot.svg.render(canvas, "./sample_funsies.svg")

canvas, _, _ = tree.draw(ts="n", edge_type="b", layout="d")
toyplot.svg.render(canvas, "./sample_funsies1.svg")


def name_color(name):
    if "land" in name:
        return "#a06033"
    if "camp" in name:
        return "#a03333"
    return "#3360a0"


tip_color_list = [name_color(tip_label) for tip_label in tree.get_tip_labels()]
canvas, _, _ = tree.draw(
    ts="n",
    tip_labels=True,
    tip_labels_align=False,
    node_sizes=[
        0 if i else 8 for i in tree.get_node_values(None, True, False)
    ],
    node_markers="s",
    node_colors="#202224",
    tip_labels_colors=tip_color_list,
)
toyplot.svg.render(canvas, "./sample_funsies2.svg")

tip_color_list = [name_color(tip_label) for tip_label in tree.get_tip_labels()]
canvas, _, _ = tree.draw(
    ts="n",
    layout="c",
    edge_type="p",
    tip_labels=True,
    tip_labels_align=False,
    tip_labels_colors=tip_color_list,
)
toyplot.svg.render(canvas, "./sample_funsies3.svg")
