use crate::days::day8_structs::{BoxPair, Circuit, JunctionBox};
use std::collections::{BinaryHeap, HashSet};

pub fn solve(input: String) -> (usize, usize) {
    let boxes: Vec<JunctionBox> = input
        .lines()
        .map(|line| {
            let split: Vec<usize> = line.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
            JunctionBox::new(split[0], split[1], split[2])
        })
        .collect();
    let part1 = solve_part1(boxes.clone(), 10);
    let part2 = solve_part2(boxes.clone());
    (part1, part2)
}

fn solve_part1(boxes: Vec<JunctionBox>, connections: usize) -> usize {
    let mut closest_pairs: BinaryHeap<BoxPair> = BinaryHeap::with_capacity(connections + 1);
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            closest_pairs.push(BoxPair::new(boxes[i], boxes[j]));
            if closest_pairs.len() > connections {
                closest_pairs.pop();
            }
        }
    }
    let mut circuits: Vec<Circuit> = Vec::new();
    while let Some(pair) = closest_pairs.pop() {
        circuits.insert(0, Circuit::from(pair));
    }
    println!("{:?}\n\n", circuits);
    merge_circuits(&mut circuits);
    circuits.sort();
    println!("{} Circuits: {:?}\n", circuits.len(), circuits);
    circuits.iter().take(3).fold(1, |acc, circuit| acc * circuit.size())
}

fn solve_part2(boxes: Vec<JunctionBox>) -> usize {
    let mut pairs: BinaryHeap<BoxPair> = BinaryHeap::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            pairs.push(BoxPair::new(boxes[i], boxes[j]));
        }
    }
    let mut circuits: Vec<Circuit> = boxes.into_iter().map(|box_| Circuit::from(box_)).collect();
    let mut last_pair: Option<BoxPair> = None;
    while circuits.len() > 1
        && let Some(pair) = pairs.pop()
    {
        last_pair = Some(pair);
        circuits.push(Circuit::from(pair));
        merge_circuits(&mut circuits);
    }
    println!("{:?}", last_pair);
    0
}

fn add_connection(circuits: &mut Vec<Circuit>, connection: BoxPair) {
    let circuit = Circuit::from(connection);
    //circuits.iter().filter(|c| c.overlaps(&circuit))
}

//noinspection D
fn merge_circuits(circuits: &mut Vec<Circuit>) {
    loop {
        let mut temp: Vec<Circuit> = circuits.clone();
        let mut cont = false;
        let mut to_remove: HashSet<usize> = HashSet::new();
        for (i, circuit) in circuits.iter_mut().enumerate() {
            for (j, other) in temp.iter().enumerate() {
                if i != j && circuit.overlaps(other) {
                    circuit.merge(other);
                    to_remove.insert(j);
                    to_remove.insert(i);
                    cont = true;
                }
            }
            if !to_remove.is_empty() {
                temp.push(circuit.clone());
                break;
            }
        }
        circuits.clear();
        for i in 0..temp.len() {
            if !to_remove.contains(&i) {
                circuits.push(temp[i].clone());
            }
        }
        if !cont {
            break;
        }
    }
}
