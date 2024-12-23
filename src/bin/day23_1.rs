use std::collections::{HashMap, BTreeSet};

const FILE_PATH: &str = "day23.txt";

fn main() {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    for line in std::fs::read_to_string(FILE_PATH).expect("Error reading input file").lines() {
        let mut split = line.split('-');
        let left = split.next().unwrap().to_string();
        let right = split.next().unwrap().to_string();

        for (computer1, computer2) in [(left.clone(), right.clone()), (right, left)] {
            if connections.contains_key(&computer1) {
                connections.get_mut(&computer1).unwrap().push(computer2);
            } else {
                connections.insert(computer1, vec![computer2]);
            }
        }
    }

    let mut interconnections: BTreeSet<BTreeSet<String>> = BTreeSet::new();

    for (computer1, computers) in &connections {
        for computer2 in computers {
            if computer1 == computer2 {
                continue;
            }
            for computer3 in computers {
                if computer2 == computer3 || computer1 == computer3
                    || !connections[computer2].contains(&computer3) {
                    continue;
                }

                interconnections.insert(BTreeSet::from([computer1.clone(), computer2.clone(), computer3.clone()]));
            }
        }
    }

    println!("{}", interconnections
            .into_iter()
            .filter(|interconnection| interconnection.into_iter().any(|s| s.chars().next().unwrap() == 't'))
            .count());
}

