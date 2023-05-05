#![allow(unused)]

use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
};
use rustc_hash::{
    FxHasher,
};

mod data_io;

fn main() {
    let twitch_data = TwitchData::new().max_score();
    for i in twitch_data.iter() {
        println!("{:?}", i);
    }
}

#[derive(Debug, Clone)]
struct TwitchData {
    nodes_edges: HashMap<u32, Vec<u32>, BuildHasherDefault<FxHasher>>,

    // [views, mature, life_time, dead_account, language, affiliate]
    features: HashMap<u32, Vec<String>, BuildHasherDefault<FxHasher>>, 
}

impl TwitchData {
    fn new() -> TwitchData {
        TwitchData {
            nodes_edges: data_io::read_edges("data/large_twitch_edges.csv".to_string()),
            features: data_io::read_features("data/large_twitch_features.csv".to_string()),
        }
    }

    fn max_score(&self) -> [(u32, u32, f32); 2] {
        // vec![(node_1, node_2, score), ...]
        // where the first tuple is the max similarity score and the second is the max dissimilarity score
        let mut max = [(0, 0, 0.0); 2];

        let num_nodes = self.nodes_edges.len();

        for i in 0..100 {
            for j in i+1..100 {
                let score = self.similarity_score(i as u32, j as u32);
                if score > max[0].2 {
                    max[0] = (i, j, score);
                } else if 1.0 - score > max[1].2 {
                    max[1] = (i, j, 1.0 - score);
                }
            }
        }

        return max
    }

    fn similarity_score(&self, node_one: u32, node_two: u32) -> f32 {
        let mut edge_one = self.nodes_edges.get(&node_one).unwrap().clone();
        let mut edge_two = self.nodes_edges.get(&node_two).unwrap().clone();

        // Remove the node from the edge list to avoid similarity between origin nodes
        if edge_one.contains(&node_two) {
            edge_one.remove(edge_one.iter().position(|x| *x == node_two).expect("not found")); 
            edge_two.remove(edge_two.iter().position(|x| *x == node_one).expect("not found"));
        };

        let views = {
            let one = edge_one.iter().map(|x| self.features.get(x).unwrap()[0].parse::<u32>().unwrap()).collect::<Vec<u32>>();
            let two = edge_two.iter().map(|x| self.features.get(x).unwrap()[0].parse::<u32>().unwrap()).collect::<Vec<u32>>();

            fn median (vec: Vec<u32>) -> u32 {
                let mut vec = vec;
                vec.sort();
                
                let mid = vec.len() / 2;
                if vec.len() == 0 {
                    return 0;
                } else if vec.len() % 2 == 0 {
                    (vec[mid] + vec[mid - 1]) / 2
                } else {
                    vec[mid]
                }
                
            }
            (median(one), median(two))
        };

        return ordinal_similarity(views.0 as f32, views.1 as f32);
    }
}

fn ordinal_similarity(one:f32, two:f32) -> f32 {
    return 1.0 - (one - two).abs() / (one + two);
}

fn jaccard_index() {
    todo!()
}


#[test]
fn test_twitch_data() {
    todo!()
}