use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(line: String) -> Range {
        let (first, second) = line.split_once("-").unwrap();
        Range {
            start: first.parse::<usize>().unwrap(),
            end: second.parse::<usize>().unwrap(),
        }
    }

    fn contains(&self, id: &usize) -> bool {
        self.start <= *id && *id <= self.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    fn merge(&mut self, other: &Range) {
        if !self.overlaps(other) {
            panic!("Can't merge two ranges that don't overlap!");
        }
        self.start = std::cmp::min(self.start, other.start);
        self.end = std::cmp::max(self.end, other.end);
    }

    fn total_ids(&self) -> usize {
        self.end - self.start + 1
    }
}

pub fn solve(input: String) -> (usize, usize) {
    let (mut ranges, mut ids): (Vec<Range>, Vec<usize>) = (Vec::new(), Vec::new());
    for line in input.lines() {
        if line.contains("-") {
            ranges.push(Range::new(line.parse().unwrap()));
        } else if !line.is_empty() {
            ids.push(line.parse().unwrap());
        }
    }
    let part1 = solve_part1(&ranges, &ids);
    let part2 = solve_part2(ranges);
    (part1, part2)
}

fn solve_part1(fresh_ranges: &Vec<Range>, ids: &Vec<usize>) -> usize {
    let mut fresh_sum = 0;
    for id in ids {
        for range in fresh_ranges {
            if range.contains(id) {
                fresh_sum += 1;
                break;
            }
        }
    }
    fresh_sum
}

//noinspection D
fn solve_part2(mut fresh_ranges: Vec<Range>) -> usize {
    loop {
        let mut temp: Vec<Range> = fresh_ranges.clone();
        let mut cont = false;
        let mut to_remove: HashSet<usize> = HashSet::new();
        for (i, range) in fresh_ranges.iter_mut().enumerate() {
            for (j, other) in temp.iter().enumerate() {
                if i != j && range.overlaps(other) {
                    range.merge(other);
                    to_remove.insert(j);
                    to_remove.insert(i);
                    cont = true;
                }
            }
            if !to_remove.is_empty() {
                temp.push(range.clone());
                break;
            }
        }
        fresh_ranges = Vec::new();
        for i in 0..temp.len() {
            if !to_remove.contains(&i) {
                fresh_ranges.push(temp[i].clone());
            }
        }
        //println!("{:?}", fresh_ranges);
        if !cont {
            break;
        }
    }
    fresh_ranges.iter().fold(0, |sum, range| sum + range.total_ids())
}
