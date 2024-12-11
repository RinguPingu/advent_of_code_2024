#[derive(Debug)]
struct Rule {
    page: i32,
    requires: i32,
}

impl Rule {
    fn new(page: i32, requires: i32) -> Self {
        Rule { page, requires }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input/example.txt").unwrap();

    let i = input.lines();

    let rules: Vec<Rule> = i
        .take_while(|l| !l.is_empty())
        .map(|s| {
            s.split('|')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|numbers| Rule::new(*numbers.last().unwrap(), *numbers.first().unwrap()))
        .collect();
    
    println!("{:?}", rules);

    for line in i {
        println!("{}", line);
    }
}
