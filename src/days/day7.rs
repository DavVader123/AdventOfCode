#[derive(Copy, Clone)]
struct Position {
    line: usize,
    column: usize,
}

struct Grid {
    grid: Vec<Vec<String>>,
    current_line: usize,
}
pub fn solve(input: String) -> (usize, usize) {
    let mut grid = Grid {
        grid: input.lines().map(|line| line.chars().map(|c| String::from(c)).collect()).collect(),
        current_line: 0,
    };
    while grid.do_step() {}
    let part1 = grid.count_splits();
    let part2 = grid.count_timelines();
    (part1, part2)
}

impl Position {
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

    fn set_beam(&mut self, pos: Position) {
        self.set_pos(pos, "|");
    }

    fn set_pos(&mut self, pos: Position, c: &str) {
        self.grid[pos.line][pos.column] = String::from(c);
    }

    fn do_step(&mut self) -> bool {
        self.current_line += 1;
        if self.current_line == self.grid.len() {
            return false;
        }
        let beams: Vec<Position> = self.grid[self.current_line - 1]
            .iter()
            .enumerate()
            .filter(|point| *point.1 == "S" || *point.1 == "|" || (*point.1).parse::<usize>().is_ok())
            .map(|a| Position {
                line: self.current_line,
                column: a.0,
            })
            .collect();

        for pos in beams {
            if self.get(pos) == "." {
                self.set_beam(pos);
            } else if self.get(pos) == "^" {
                if self.is_valid(pos.left_neighbor()) {
                    self.set_beam(pos.left_neighbor())
                }
                if self.is_valid(pos.right_neighbor()) {
                    self.set_beam(pos.right_neighbor())
                }
                self.set_pos(pos, "M");
            }
        }
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
