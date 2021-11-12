use crate::am::AdjacencyMatrix;

/// **WARNING:** INCREDIBLY SHITTY HEURISTICS! But, you know. You could do much
/// worse.
pub fn guess_order(am: &AdjacencyMatrix) -> Vec<usize> {
    let mut v_pair_ranking =
        Vec::with_capacity(am.stride() * (am.stride() - 1) / 2);

    for i in 0..am.stride() {
        for j in (i + 1)..am.stride() {
            let ij_deg = am[(i, j)].len();
            v_pair_ranking.push((i, j, ij_deg));
        }
    }
    v_pair_ranking
        .sort_unstable_by(|(_, _, d0), (_, _, d1)| d0.cmp(d1).reverse());

    let mut order = Vec::with_capacity(am.stride());
    for (i, j, _) in v_pair_ranking {
        if order.len() >= am.stride() {
            assert_eq!(order.len(), am.stride());
            break;
        }

        let i_ix = order.iter().position(|x| x == &i);
        let j_ix = order.iter().position(|x| x == &j);
        match (i_ix, j_ix) {
            (Some(_), Some(_)) => (),
            (Some(i_ix), _) => order.insert(i_ix + 1, j),
            (_, Some(j_ix)) => order.insert(j_ix, i),
            _ => {
                order.push(i);
                order.push(j);
            }
        }
    }

    order
}
