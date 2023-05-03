use std::{
    collections::HashMap,
    collections::hash_map::Entry,
    hash::BuildHasherDefault,
};
use rustc_hash::{
    FxHashMap,
    FxHasher,
};

use csv::Reader;


/// Takes in a csv file of edges and returns a hashmap of the form:
/// - {node: \[\[node, similarity, disiimilarity], ...], ...}
pub fn read_edges(file: String) -> HashMap<u32, Vec<Vec<f32>>, BuildHasherDefault<FxHasher>> {
    let mut rdr = Reader::from_path(file).unwrap();
    let mut records = vec![];

    for rec in rdr.records() {
        let rec = rec.unwrap();
        records.push(rec.iter().map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>());
    }

    let mut map: FxHashMap<u32, Vec<Vec<f32>>> = FxHashMap::default();

    for values in records {
        let id_1 = values[0];
        let id_2 = values[1];

        for (first, second) in [(id_1, id_2), (id_2, id_1)].into_iter() {
            match map.entry(first as u32) {
                Entry::Vacant(e) => {
                    e.insert(vec![vec![second, 0.0, 0.0]]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(vec![second, 0.0, 0.0]);
                }
            }
        }

    }
    
    return map;
    
}


/// Takes in a csv file of features and returns a hashmap of the form:
/// - {node: \[views, mature, life_time, dead_account, language, affiliate], ...}
pub fn read_features(file: String) -> HashMap<u32, Vec<String>, BuildHasherDefault<FxHasher>> {
    let mut rdr = Reader::from_path(file).unwrap();
    let mut records: Vec<Vec<String>> = vec![];

    for rec in rdr.records() {
        let rec = rec.unwrap();
        records.push(rec.iter().map(|x| x.parse::<String>().unwrap()).collect::<Vec<String>>());
    }

    let mut map: FxHashMap<u32, Vec<String>> = FxHashMap::default();

    // The hashmap becomes:
    // [views, mature, life_time, dead_account, language, affiliate]
    for mut values in records {
        let key = values[5].parse::<u32>().unwrap();

        values.drain(3..=5); // drops created_at, updated at, and id

        match map.entry(key) {
            Entry::Vacant(e) => {
                e.insert(values);
            }
            Entry::Occupied(e) => {
                panic!("Duplicate key: {}", e.key());
            }
        }

    }

    return map;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_edges() {
        let temp = read_edges("data/large_twitch_edges.csv".to_string());
        for (i, (key, value)) in temp.iter().enumerate() {
            println!("{}: {:?}\n", key, value);
            if i == 4 {
                break;
            }
        }
    }

    #[test]
    fn test_features() {
        let temp = read_features("data/large_twitch_features.csv".to_string());
        for (i, (key, value)) in temp.iter().enumerate() {
            println!("{}: {:?}\n", key, value);
            if i == 4 {
                break;
            }
        }
    }
}
