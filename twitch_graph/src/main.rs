use data_io::twitch_edges_data;
mod data_io;

#[test]
fn testing() {
    let mut rdr = csv::Reader::from_path("data/large_twitch_edges.csv").unwrap();
    let mut records = vec![];

    for rec in rdr.records() {
        let rec = rec.unwrap();
        records.push(rec.iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>());
    }


    println!("{:?}", records[0..10].to_vec());
}

fn main() {
    let map = twitch_edges_data();
    for (index, (key, value)) in map.iter().enumerate() {
        println!("{}: {:?}\n", key, value);
        if index == 10 {
            break;
        }
    }
}
