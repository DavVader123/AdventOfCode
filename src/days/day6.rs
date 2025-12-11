use crate::utils::get_input;

pub fn solve(input: String) -> (usize, usize) {
    let part1 = solve_part1(input.clone());
    let part2 = solve_part2(get_input(6));
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
                _ => {}
            }
        }
        sum += answer;
    }
    sum
}

fn solve_part2(input: String) -> usize {
    let input = input.lines().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let operators = input.len() - 1;
    let (mut sum, mut answer): (usize, usize) = (0, 0);
    let mut number: String = String::new();
    let mut operator: char = '.';
    for column in 0..input[0].len() {
        if input[operators][column] != ' ' {
            operator = input[operators][column];
            sum += answer;
            //println!(" {}", answer);
            answer = if operator == '*' { 1 } else { 0 };
        }
        number.clear();
        for line in 0..input.len() - 1 {
            number.push(input[line][column]);
        }
        let number = number.trim();
        if number.is_empty() {
            continue;
        }
        //print!("{} {} ", number, operator);
        match operator {
            '+' => answer += number.parse::<usize>().unwrap(),
            '*' => answer *= number.parse::<usize>().unwrap(),
            _ => {}
        }
    }
    sum + answer
}
