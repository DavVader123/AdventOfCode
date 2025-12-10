pub fn solve(input: String) -> (usize, usize) {
	let part1 = solve_part1(input.clone());
	let part2 = solve_part2(input.clone()); 
    (part1, part2) 
} 
 
fn solve_part1(input: String) -> usize {
    let mut numbers: Vec<Vec<usize>> = Vec::new();
    for line in input.lines().rev().skip(1) {
        numbers.push(line.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect());
    }
    let operators: Vec<&str> = input.lines().last().unwrap().split_whitespace().collect();
    let mut sum = 0;
    for problem in 0..numbers[0].len() {
        let mut answer = numbers[0][problem];
        for n in 1..numbers.len() {
            match operators[problem] {
                "+" => answer += numbers[n][problem],
                "*" => answer *= numbers[n][problem],
                _ => {},
            }
        }
        sum += answer;
    }
    sum
} 
 
fn solve_part2(input: String) -> usize { 
    let input = input.lines()..map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    0
}
