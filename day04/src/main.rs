use grid::Grid;
fn main() {
    let input = std::fs::read_to_string("./input/example.txt").unwrap();

    let mut xmas_counter: u32 = 0;
    let mut part2_xmas_counter: u32 = 0;

    let grid = Grid::from_vec(
        input.chars().filter(|c| c.is_alphanumeric()).collect(),
        input.lines().count(),
    );

    println!("{:#?}", grid);

    // Part 1 Solution
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if *grid.get(row, col).unwrap() == 'X' {
                // Check right
                if let Some(second) = grid.get(row, col + 1) {
                    if *second == 'M' {
                        if let Some(third) = grid.get(row, col + 2) {
                            if *third == 'A' {
                                if let Some(fourth) = grid.get(row, col + 3) {
                                    if *fourth == 'S' {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                // Check right-down
                if let Some(second) = grid.get(row + 1, col + 1) {
                    if *second == 'M' {
                        if let Some(third) = grid.get(row + 2, col + 2) {
                            if *third == 'A' {
                                if let Some(fourth) = grid.get(row + 3, col + 3) {
                                    if *fourth == 'S' {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                // Check right-up
                if let Some(second) = grid.get(row as i32 - 1, col + 1) {
                    if *second == 'M' {
                        if let Some(third) = grid.get(row as i32 - 2, col + 2) {
                            if *third == 'A' {
                                if let Some(fourth) = grid.get(row as i32 - 3, col + 3) {
                                    if *fourth == 'S' {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                // Check left
                if let Some(second) = grid.get(row, col as i32 - 1) {
                    if *second == 'M' {
                        if let Some(third) = grid.get(row, col as i32 - 2) {
                            if *third == 'A' {
                                if let Some(fourth) = grid.get(row, col as i32 - 3) {
                                    if *fourth == 'S' {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                // Check left-down
                if let Some(second) = grid.get(row + 1, col as i32 - 1) {
                    if *second == 'M' {
                        if let Some(third) = grid.get(row + 2, col as i32 - 2) {
                            if *third == 'A' {
                                if let Some(fourth) = grid.get(row + 3, col as i32 - 3) {
                                    if *fourth == 'S' {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                // Check left-up
                if let Some(second) = grid.get(row as i32 - 1, col as i32 - 1) {
                    if *second == 'M' {
                        if let Some(third) = grid.get(row as i32 - 2, col as i32 - 2) {
                            if *third == 'A' {
                                if let Some(fourth) = grid.get(row as i32 - 3, col as i32 - 3) {
                                    if *fourth == 'S' {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                // Check down
                if let Some(second) = grid.get(row + 1, col) {
                    if *second == 'M' {
                        if let Some(third) = grid.get(row + 2, col) {
                            if *third == 'A' {
                                if let Some(fourth) = grid.get(row + 3, col) {
                                    if *fourth == 'S' {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                // Check up
                if let Some(second) = grid.get(row as i32 - 1, col) {
                    if *second == 'M' {
                        if let Some(third) = grid.get(row as i32 - 2, col) {
                            if *third == 'A' {
                                if let Some(fourth) = grid.get(row as i32 - 3, col) {
                                    if *fourth == 'S' {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\"XMAS\" Count: {}", xmas_counter);

    // Part 2 Solution
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if *grid.get(row, col).unwrap() == 'A' {
                if (row as i32 - 1).is_negative() || (col as i32).is_negative() || row + 1 == grid.rows() || col + 1 == grid.cols()
                {
                    continue
                }

                if let Some(top_left) = grid.get(row as i32 - 1, col as i32 - 1) {
                    if *top_left == 'M' {
                        if let Some(bottom_right) = grid.get(row + 1, col + 1) {
                            if *bottom_right != 'S' {
                                continue;
                            }
                        }
                    }
                    if *top_left == 'S' {
                        if let Some(bottom_right) = grid.get(row + 1, col + 1) {
                            if *bottom_right != 'M' {
                                continue;
                            }
                        }
                    }
                }

                if let Some(top_right) = grid.get(row as i32 - 1, col + 1) {
                    if *top_right == 'M' {
                        if let Some(bottom_left) = grid.get(row + 1, col as i32 - 1) {
                            if *bottom_left != 'S' {
                                continue;
                            }
                        }
                    }
                    if *top_right == 'S' {
                        if let Some(bottom_left) = grid.get(row + 1, col as i32 - 1) {
                            if *bottom_left != 'M' {
                                continue;
                            }
                        }
                    }
                }
                part2_xmas_counter += 1;
            }
        }
    }

    println!("Part 2 X-MAS Counter: {}", part2_xmas_counter);
}
