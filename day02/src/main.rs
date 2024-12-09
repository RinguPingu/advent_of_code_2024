fn main() {
    let input = std::fs::read_to_string("./input/example.txt").expect("Invalid Input File!");

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|report| report.iter().map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();

    let mut safe_reports = 0;
    let mut part2_safe_reports = 0;

    for report in reports {
        let mut distances = Vec::new();
        let mut i = report.iter().enumerate().peekable();

        while let Some(level) = i.next() {
            if let Some(next_number) = i.peek() {
                let distance = level.1 - next_number.1;
                distances.push((distance, level.0));
            }
        }

        println!("Distances:\t{:?}", distances);

        let mut unsafe_distances: Vec<&(i32, usize)> = Vec::new();

        for distance in distances
            .iter()
            .filter(|distance| distance.0.abs() > 3 || distance.0.abs() < 1)
        {
            println!("Unsafe distance! Too far: {}", distance.0);
            unsafe_distances.push(distance);
        }

        let positive_distances = distances
            .iter()
            .filter(|distance| distance.0.is_positive())
            .count();
        let negative_distances = distances
            .iter()
            .filter(|distance| distance.0.is_negative())
            .count();

        if positive_distances != distances.len() || negative_distances != distances.len() {
            if positive_distances > negative_distances {
                for distance in distances.iter().filter(|distance| distance.0.is_negative()) {
                    println!("Unsafe distance! Negative: {}", distance.0);
                    unsafe_distances.push(distance);
                }
            } else {
                for distance in distances.iter().filter(|distance| distance.0.is_positive()) {
                    println!("Unsafe distance! Positive: {}", distance.0);
                    unsafe_distances.push(distance);
                }
            }
        }

        unsafe_distances.dedup();
        println!("Unsafe Distances:\t{:?}\n", unsafe_distances);

        if unsafe_distances.len() < 2
        {
            safe_reports += 1;
            part2_safe_reports += 1;
            continue;
        }
        if unsafe_distances.is_empty()
        {
            safe_reports += 1;
        }
    }

    println!("Safe Reports: {}", safe_reports);
    println!("Part 2 Safe Reports: {}", part2_safe_reports);
}
