use crate::am::AdjacencyMatrix;
use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

pub fn from_reader<R: Read>(
    rdr: R,
) -> ron::Result<(
    Vec<String>,
    HashMap<String, HashSet<String>>,
    AdjacencyMatrix,
)> {
    let raw: HashMap<String, HashSet<String>> = ron::de::from_reader(rdr)?;
    let vs = {
        let mut vs = Vec::with_capacity(raw.len());
        for he in raw.values() {
            for v in he {
                if !vs.contains(v) {
                    vs.push(v.clone());
                }
            }
        }
        vs.sort_unstable();

        vs
    };

    let mut am = AdjacencyMatrix::empty(vs.len());
    for (he_name, he) in raw.iter() {
        for v0 in he {
            let v0_id =
                vs.binary_search(v0).unwrap_or_else(|_| unreachable!());
            for v1 in he {
                let v1_id =
                    vs.binary_search(v1).unwrap_or_else(|_| unreachable!());
                am[(v0_id, v1_id)].insert(he_name.clone());
                am[(v1_id, v0_id)].insert(he_name.clone());
            }
        }
    }

    Ok((vs, raw, am))
}
