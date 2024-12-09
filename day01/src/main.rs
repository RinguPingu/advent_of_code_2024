fn main() {
    let input = std::fs::read_to_string("./input/input.txt").expect("Invalid File!");

    let number_pairs = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut left_list: Vec<i32> = number_pairs
        .iter()
        .map(|pair| pair.first().unwrap().parse().unwrap())
        .collect();
    
    let mut right_list: Vec<i32> = number_pairs
        .iter()
        .map(|pair| pair.last().unwrap().parse().unwrap())
        .collect();

    left_list.sort();
    right_list.sort();

    println!("{:?}", left_list);
    println!("{:?}", right_list);

    let distances: Vec<u32> = left_list.iter().zip(&right_list).map(|numbers| numbers.0.abs_diff(*numbers.1)).collect();

    let summed_distances: u32 = distances.iter().sum();

    println!("Summed Distances:\t{}", summed_distances);

    let similarity_scores : Vec<i32> = left_list.iter().map(|num| {
        num * right_list.iter().filter(|n| *n == num).count() as i32
    }).collect();

    println!("{:?}", similarity_scores);

    let summed_similarity_scores : i32 = similarity_scores.iter().sum();

    println!("Summed Similarity Scores:\t{}", summed_similarity_scores);

}
