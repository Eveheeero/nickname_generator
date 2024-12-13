use crate::data_collector::opendict::{v1::OpendictData, Pos};
use std::collections::HashMap;

fn generate(pool: &[&OpendictData], queries: &[Pos]) -> Vec<String> {
    let mut data_per_pos = HashMap::new();
    for query in queries {
        if data_per_pos.contains_key(query) {
            continue;
        }
        let d = pool;
        data_per_pos.insert(query, d);
    }
    todo!()
}
