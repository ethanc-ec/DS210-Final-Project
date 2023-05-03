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
    edges: HashMap<u32, Vec<Vec<f32>>, BuildHasherDefault<FxHasher>>,
    features: HashMap<u32, Vec<String>, BuildHasherDefault<FxHasher>>,
    max_similarity: Vec<f32>,
    max_dissimilarity: Vec<f32>,
}

impl TwitchData {
    fn new() -> TwitchData {
        TwitchData {
            edges: data_io::read_edges("data/large_twitch_edges.csv".to_string()),
            features: data_io::read_features("data/large_twitch_features.csv".to_string()),
            max_similarity: vec![0.0, 0.0, 0.0],
            max_dissimilarity: vec![0.0, 0.0, 0.0],
        }
    }

    fn find_scores(&mut self) {
        for (key, value) in self.edges.clone().iter() {
            for (index, values) in value.iter().enumerate() {
                let node_one = key.clone();
                let node_two = values[0];
                let (similarity, dissimilarity) = self.score_calc(node_one, node_two as u32);
                self.edges.get_mut(&node_one).unwrap()[index][1] = similarity;
                self.edges.get_mut(&node_one).unwrap()[index][2] = dissimilarity;
            }
        }
    }

    /// Calculates the similarity and dissimilarity scores between two nodes.
    /// - \[views, mature, life_time, dead_account, language, affiliate]
    fn score_calc(&mut self, node_one: u32, node_two: u32) -> (f32, f32){
        let features_one = self.features.get(&node_one).unwrap();
        let features_two = self.features.get(&node_two).unwrap();
        let mut similarity = 0.0;

        for i in 0..features_one.len() {
            if vec![0, 2].contains(&i) {
                let num_1 = features_one[i].parse::<f32>().unwrap();
                let num_2 = features_two[i].parse::<f32>().unwrap();
                similarity += 1.0 - (num_1 - num_2).abs() / (num_1 + num_2);
            } else {
                if features_one[i] == features_two[i] {
                    similarity += 1.0;
                }
            }
        }

        let dissimilarity = 1.0 - similarity/features_one.len() as f32;
        let similarity = similarity/features_one.len() as f32;


        if similarity > self.max_similarity[2] as f32 {
            self.max_similarity = vec![node_one as f32, node_two as f32, similarity];
        } 

        if dissimilarity > self.max_dissimilarity[2] as f32 {
            self.max_dissimilarity = vec![node_one as f32, node_two as f32, dissimilarity];
        }

        return (similarity, dissimilarity);
    }
}

#[test]
fn test_twitch_data() {
    let mut temp = TwitchData::new();
    temp.find_scores();
    for (i, (key, value)) in temp.edges.iter().enumerate() {
        println!("{}: {:?}\n", key, value);
        if i == 4 {
            break;
        }
    }
    println!("Max Similarity: {:?}", temp.max_similarity);
    println!("Max Dissimilarity: {:?}", temp.max_dissimilarity);
}