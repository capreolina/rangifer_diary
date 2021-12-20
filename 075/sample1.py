import toytree
import toyplot.svg

tree = toytree.tree("./sample.tree")
canvas, _, _ = tree.draw(ts="n", tip_labels_align=True)
toyplot.svg.render(canvas, "./sample1.svg")
