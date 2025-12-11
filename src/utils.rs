pub fn get_input_trimmed(day: u8) -> String {
    let path: String = format!("./input/input{}.txt", day);
    let contents = std::fs::read_to_string(path.clone()).expect(format!("Something went wrong reading the file {}", path).as_str());
    contents.trim().to_string()
}

pub fn get_input(day: u8) -> String {
    let path: String = format!("./input/input{}.txt", day);
    let contents = std::fs::read_to_string(path.clone()).expect(format!("Something went wrong reading the file {}", path).as_str());
    contents.to_string()
}
