use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
};
use rustc_hash::{
    FxHasher,
};

mod data_io;

fn main() {

}

#[derive(Debug, Clone)]
struct TwitchData {
    edges: HashMap<u32, Vec<u32>, BuildHasherDefault<FxHasher>>,
    features: HashMap<u32, Vec<String>, BuildHasherDefault<FxHasher>>,
}

impl TwitchData {
    fn new() -> TwitchData {
        TwitchData {
            edges: data_io::read_edges("data/large_twitch_edges.csv".to_string()),
            features: data_io::read_features("data/large_twitch_features.csv".to_string()),
        }
    }
}

#[test]
fn test_twitch_data() {
    todo!()
}