#[macro_use]
extern crate timeit;

use std::{
    collections::HashMap,
    hash::BuildHasherDefault, u128,
};
use rustc_hash::{
    FxHasher,
};

use utils::{
    median,
    ordinal_similarity,
    read_edges,
    read_features,
};

mod utils;



fn main() {
    timeit!({
        let scores = TwitchData::new().max_score();
        println!("Max similarity score: {:?}, between vertex {} and vertex {}", scores[0].2, scores[0].0, scores[0].1);
        println!("Max dissimilarity score: {:?}, between vertex {} and vertex {}", scores[1].2, scores[1].0, scores[1].1);
    });
}


#[derive(Debug, Clone)]
struct TwitchData {
    nodes_edges: HashMap<u32, Vec<u32>, BuildHasherDefault<FxHasher>>,
    features: HashMap<u32, Vec<String>, BuildHasherDefault<FxHasher>>, 
}


impl TwitchData {
    fn new() -> TwitchData {
        TwitchData {
            nodes_edges: read_edges("data/large_twitch_edges.csv".to_string()),
            features: read_features("data/large_twitch_features.csv".to_string()),
        }
    }

    fn max_score(&self) -> [(u32, u32, f64); 2] {
        // array[(node_1, node_2, score), ...]
        // where the first tuple is the max similarity score and the second is the max dissimilarity score
        let mut max = [(0, 0, 0.0); 2];
        let mut seen: Vec<u32> = vec![];

        // for each node, compare it to every other node
        for node in self.nodes_edges.keys() {
            for j in self.nodes_edges.get(node).unwrap() {
                // if we've already seen this node, skip it
                if seen.contains(&*node) {
                    continue;
                }

                let score = self.similarity_score(*node, *j);

                if score > max[0].2 {
                    max[0] = (*node, *j, score);
                } else if 1.0 - score > max[1].2 {
                    max[1] = (*node, *j, 1.0 - score);
                }
            }

            // A similarity/dissimilarity of 99.99% is significant enough to stop the search
            if max[0].2 >= 0.9999 && max[1].2 >= 0.9999 {
                println!("\nSearch stopped due to similarity and dissimilarity greater than 99.99%\n");
                break;
            }

            seen.push(*node);

        };

        return max
    }

    fn similarity_score(&self, node_one: u32, node_two: u32) -> f64 {
        let edge_one = self.nodes_edges.get(&node_one).unwrap();
        let edge_two = self.nodes_edges.get(&node_two).unwrap();

        if edge_one.len() == 0 || edge_two.len() == 0 {
            return 0.0;
        }

        let views = {
            let mut one = edge_one.iter().map(|x| self.features.get(x).unwrap()[0].parse::<u128>().unwrap()).collect::<Vec<u128>>();
            let mut two = edge_two.iter().map(|x| self.features.get(x).unwrap()[0].parse::<u128>().unwrap()).collect::<Vec<u128>>();
            ordinal_similarity(median(&mut one), median(&mut two))
        };

        return views;
    }
}



#[test]
fn test_twitch_data() {
    timeit!({
        let data = TwitchData::new();
        let scores = data.max_score();
        println!("Max similarity score: {:?}, between vertex {} and vertex {}", scores[0].2, scores[0].0, scores[0].1);
        println!("Max dissimilarity score: {:?}, between vertex {} and vertex {}", scores[1].2, scores[1].0, scores[1].1);
    });
}