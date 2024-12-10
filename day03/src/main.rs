use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("./input/input.txt").expect("Invalid Input File!");

    let instruction_pattern = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mut instructions = instruction_pattern
        .find_iter(&input)
        .map(|m| {
            m.as_str()
                .replace("mul(", "")
                .replace(")", "")
                .split(",")
                .map(|split| split.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    println!("{:?}", instructions);

    let result: i32 = instructions.iter().map(|i| i.first().unwrap() * i.last().unwrap()).sum();

    println!("Result: {}", result);
}
