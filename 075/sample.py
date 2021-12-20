import toytree
import toyplot.svg

tree = toytree.tree("./sample.tree")
canvas, _, _ = tree.draw(ts="n", tip_labels_align=False)
toyplot.svg.render(canvas, "./sample.svg")
