pub fn solve(input: String) -> (usize, usize) {
    let part1 = solve_part1(input.clone());
    let part2 = solve_part2(input.clone());
    (part1, part2)
}

struct Dial {
    position: i16,
    zeros: usize,
}

impl Dial {
    fn new() -> Dial {
        Dial { position: 50, zeros: 0 }
    }

    fn do_rotation(&mut self, rotation: String) {
        let (dir, dist) = rotation.split_at(1);
        let dist = dist.parse::<i16>().expect("invalid parsing");
        match dir {
            "R" => {
                self.position += dist;
                while self.position > 99 {
                    self.position -= 100;
                }
            }
            "L" => {
                self.position -= dist;
                while self.position < 0 {
                    self.position += 100;
                }
            }
            &_ => {}
        }
        if self.position == 0 {
            self.zeros += 1;
        }
    }

    fn do_rotation_0x434c49434b(&mut self, rotation: String) {
        let (dir, dist) = rotation.split_at(1);
        let dist = dist.parse::<i16>().expect("invalid parsing");
        for _ in 0..dist {
            self.position += match dir {
                "R" => 1,
                "L" => -1,
                &_ => 0,
            };
            if self.position == -1 {
                self.position = 99;
            }
            if self.position == 100 {
                self.position = 0;
            }
            if self.position == 0 {
                self.zeros += 1;
            }
        }
    }
}

fn solve_part1(input: String) -> usize {
    let mut dial = Dial::new();
    for line in input.lines() {
        dial.do_rotation(line.to_string());
    }
    dial.zeros
}

fn solve_part2(input: String) -> usize {
    let mut dial = Dial::new();
    for line in input.lines() {
        dial.do_rotation_0x434c49434b(line.to_string());
    }
    dial.zeros
}
