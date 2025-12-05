#[derive(Debug)]
struct PaperRollGrid {
    rolls: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl PaperRollGrid {
    fn new(input: String) -> PaperRollGrid {
        let paper_rolls = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<char>>>();
        PaperRollGrid {
            width: paper_rolls[0].len() - 1,
            height: paper_rolls.len() - 1,
            rolls: paper_rolls,
        }
    }

    fn get_pos(&self, pos: (usize, usize)) -> char {
        self.rolls[pos.1][pos.0]
    }

    fn set_pos(&mut self, pos: (usize, usize), value: char) {
        self.rolls[pos.1][pos.0] = value;
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        if x > 0 && y > 0 {
            result.push((x - 1, y - 1));
            result.push((x - 1, y));
            result.push((x, y - 1));
        } else if x > 0 {
            result.push((x - 1, y));
        } else if y > 0 {
            result.push((x, y - 1));
        }

        if x < self.width && y < self.height {
            result.push((x + 1, y + 1));
            result.push((x + 1, y));
            result.push((x, y + 1));
        } else if x < self.width {
            result.push((x + 1, y));
        } else if y < self.height {
            result.push((x, y + 1));
        }

        if x < self.width && y > 0 {
            result.push((x + 1, y - 1));
        }
        if y < self.height && x > 0 {
            result.push((x - 1, y + 1));
        }
        result
    }

    fn adjacent_rolls(&self, x: usize, y: usize) -> usize {
        let mut sum = 0;
        for neighbor in self.get_neighbors(x, y) {
            if self.get_pos(neighbor) == '@' {
                sum += 1;
            }
        }
        sum
    }
}

pub fn solve(input: String) -> (usize, usize) {
    let part1 = solve_part1(PaperRollGrid::new(input.clone()));
    let part2 = solve_part2(PaperRollGrid::new(input.clone()));
    (part1, part2)
}

fn solve_part1(input: PaperRollGrid) -> usize {
    let mut accessible_rolls = 0;
    for x in 0..=input.width {
        for y in 0..=input.height {
            if input.get_pos((x, y)) == '@' && input.adjacent_rolls(x, y) < 4 {
                accessible_rolls += 1;
            }
        }
    }
    accessible_rolls
}

fn solve_part2(mut input: PaperRollGrid) -> usize {
    let mut removable_rolls = 0;
    loop {
        let mut removed = false;
        for x in 0..=input.width {
            for y in 0..=input.height {
                if input.get_pos((x, y)) == '@' && input.adjacent_rolls(x, y) < 4 {
                    removable_rolls += 1;
                    removed = true;
                    input.set_pos((x, y), '.');
                }
            }
        }
        if !removed {
            break;
        }
    }
    removable_rolls
}
