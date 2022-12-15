use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    //read file to vec
    let file = fs::read_to_string("graph.txt")
        .expect("Error reading file");

    //nodes into hashmap kvp
    let mut friends_hashmap: HashMap<i32, Vec<i32>> = HashMap::new();


    //read vec into hashmap
    for line in file.lines() {
        let mut split = line.split_whitespace();
        let node = split.next().expect("Error parsing node")
            .parse().expect("Error parsing node");

        let friends: Vec<i32> = split.map(|x| x.parse().expect("Error parsing friend"))
            .collect();

        friends_hashmap.insert(node, friends);
    }

    //calculate distance = sum/count

    let mut sum = 0;
    let mut count = 0;

    for (node1, friends1) in friends_hashmap.iter() {
        for (node2, friends2) in friends_hashmap.iter() {

            if node1 < node2 {
                let distance = distance(friends1, friends2);
                sum += distance;
                count += 1;
            }
        }
    }

    let avg = sum as f64 / count as f64;

    println!("avg: {}", avg);
}


fn distance(first: &Vec<i32>, second: &Vec<i32>) -> i32 {

    let friends_hashset: HashSet<&i32> = first.into_iter().collect();
    let mut distance = 0 as i32;

    for node in second {
        if !friends_hashset.contains(node) {
            distance += 1;
        }
    }

    distance
}
