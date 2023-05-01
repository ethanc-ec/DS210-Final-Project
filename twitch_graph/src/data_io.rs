use std::{
    collections::HashMap,
    collections::hash_map::Entry,
};

use csv;

/// Takes in a csv file of edges and returns a hashmap of the form:
/// - {node: [[node, similarity, disiimilarity], ...], ...}
pub fn twitch_edges_data() -> HashMap<u32, Vec<Vec<u32>>> {
    let mut rdr = csv::Reader::from_path("data/large_twitch_edges.csv").unwrap();
    let mut records = vec![];

    for rec in rdr.records() {
        let rec = rec.unwrap();
        records.push(rec.iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>());
    }

    let mut map: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();

    for values in records {
        match map.entry(values[0]) {
            Entry::Vacant(e) => {
                e.insert(vec![vec![values[1], 0, 0]]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(vec![values[1], 0, 0]);
            }
        }

    }
    
    return map;
    
}