pub fn solve(input: String) -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;
    let ranges = input.split(',');
    for range in ranges {
        let (id1, id2): (usize, usize) = range
            .split_once('-')
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .unwrap();
        for id in id1..=id2 {
            let id_str = id.to_string();
            if is_invalid_part1(id_str.clone()) {
                part1 += id;
            }
            if is_invalid_part2(id_str.clone()) {
                part2 += id;
            }
        }
    }
    (part1, part2)
}

fn is_invalid_part1(id: String) -> bool {
    if id.len() % 2 != 0 {
        return false;
    }
    let (left, right) = id.split_at(id.len() / 2);
    left == right
}

fn is_invalid_part2(id: String) -> bool {
    if is_invalid_part1(id.clone()) {
        return true;
    }
    let mut seq: String = id[0..1].to_string();
    let mut i = 1;
    while i < id.len() && i + seq.len() <= id.len() {
        if id[i..i + seq.len()] == seq {
            i += seq.len();
            if i >= id.len() {
                return true;
            }
        } else {
            i += 1;
            seq = id[0..i].to_string();
            if seq.len() > id.len() / 2 {
                return false;
            }
        }
    }
    false
}
