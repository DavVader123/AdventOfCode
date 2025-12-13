use std::fmt::Debug;

#[derive(Copy, Clone)]
struct Position {
    line: usize,
    column: usize,
}

struct Grid {
    grid: Vec<Vec<String>>,
    current_line: usize,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            writeln!(f, "{}", line.join(" "))?;
        }
        Ok(())
    }
}
pub fn solve(input: String) -> (usize, usize) {
    let mut grid = Grid {
        grid: input.lines().map(|line| line.chars().map(|c| String::from(c)).collect()).collect(),
        current_line: 0,
    };
    while grid.do_step() {
        //println!("{:?}", grid);
    }
    let part1 = grid.count_splits();
    let part2 = grid.count_timelines();
    (part1, part2)
}

impl Position {
    fn step_down(&mut self) {
        self.line += 1;
    }

    fn right_neighbor(&self) -> Position {
        Position {
            line: self.line,
            column: self.column + 1,
        }
    }

    fn left_neighbor(&self) -> Position {
        Position {
            line: self.line,
            column: self.column - 1,
        }
    }
}

impl Grid {
    fn is_valid(&self, pos: Position) -> bool {
        pos.line < self.grid.len() && pos.column < self.grid[0].len()
    }

    fn get(&self, pos: Position) -> String {
        self.grid[pos.line][pos.column].clone()
    }

    fn set_pos(&mut self, pos: Position, c: String) {
        self.grid[pos.line][pos.column] = c;
    }

    fn add_timelines(&mut self, pos: Position, count: usize) {
        let count = count + self.get(pos).parse::<usize>().unwrap_or(0);
        self.set_pos(pos, count.to_string());
    }

    fn do_step(&mut self) -> bool {
        if self.current_line == self.grid.len() - 1 {
            return false;
        }
        let beams: Vec<Position> = self.grid[self.current_line]
            .iter()
            .enumerate()
            .filter(|point| *point.1 == "S" || (*point.1).parse::<usize>().is_ok())
            .map(|a| Position {
                line: self.current_line,
                column: a.0,
            })
            .collect();

        for mut pos in beams {
            let count = self.get(pos).parse::<usize>().unwrap_or(1);
            pos.step_down();
            if self.get(pos) == "^" {
                if self.is_valid(pos.left_neighbor()) {
                    self.add_timelines(pos.left_neighbor(), count)
                }
                if self.is_valid(pos.right_neighbor()) {
                    self.add_timelines(pos.right_neighbor(), count)
                }
                self.set_pos(pos, String::from("M"));
            } else {
                self.add_timelines(pos, count);
            }
        }
        self.current_line += 1;
        true
    }

    fn count_splits(&self) -> usize {
        self.grid.iter().flatten().filter(|point| **point == "M").count()
    }

    fn count_timelines(&self) -> usize {
        self.grid[self.grid.len() - 1]
            .iter()
            .map(|point| point.parse::<usize>().unwrap_or(0))
            .fold(0, |sum, x| sum + x)
    }
}
