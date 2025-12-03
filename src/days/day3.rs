pub fn solve(input: String) -> (usize, usize) {
	let mut part1 = 0;
	let mut part2 = 0;
    for line in input.lines() {
        part1 += maximum_joltage(line.to_string(), 2);
        part2 += maximum_joltage(line.to_string(), 12);
    }
    (part1, part2)
}

fn find_max_digit(line: &String, first: usize, last: usize) -> usize {
    let mut max = 1;
    let mut max_id = first;
    for i in first..last {
        let digit = line.chars().nth(i).unwrap().to_digit(10).unwrap();
        if digit > max {
            max = digit;
            max_id = i;
            if max == 9 {
                break;
            }
        }
    }
    //println!("Max digit of {} between {} and {} is {} at position {}", line, first, last, max, max_id);
    max_id
}
 
fn maximum_joltage(battery_bank: String, num_batteries: usize) -> usize {
    let mut max: String = String::new();
    let mut current_idx = 0;
    for i in (0..num_batteries).rev() {
        if current_idx < battery_bank.len()-i {
            current_idx = find_max_digit(&battery_bank, current_idx, battery_bank.len()-i);
        }
        max.push_str(&battery_bank[current_idx..=current_idx]);
        current_idx += 1;
    }
    //println!("The maximum joltage with {} batteries is {}", num_batteries, max);
    max.parse::<usize>().unwrap()
} 
