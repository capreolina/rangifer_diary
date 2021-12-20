import toytree
import toyplot.svg

tree = toytree.tree("./sample_weighted.tree")
canvas, _, _ = tree.draw(ts="n", tip_labels_align=False)
toyplot.svg.render(canvas, "./sample_weighted.svg")
