use grid::Grid;
fn main() {
    let input = std::fs::read_to_string("./input/input.txt").unwrap();

    let mut xmas_counter: u32 = 0;

    let grid = Grid::from_vec(input.chars().filter(|c| c.is_alphanumeric()).collect(), input.lines().count());

    println!("{:#?}", grid);

    for row in 0..grid.rows()
    {
        for col in 0..grid.cols()
        {
            if *grid.get(row, col).unwrap() == 'X'
            {
                // Check right
                if let Some(second) = grid.get(row, col + 1)
                {
                    if *second == 'M'
                    {
                        if let Some(third) = grid.get(row, col + 2)
                        {
                            if *third == 'A'
                            {
                                if let Some(fourth) = grid.get(row, col + 3)
                                {
                                    if *fourth == 'S'
                                    {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                // Check right-down
                if let Some(second) = grid.get(row + 1, col + 1)
                {
                    if *second == 'M'
                    {
                        if let Some(third) = grid.get(row + 2, col + 2)
                        {
                            if *third == 'A'
                            {
                                if let Some(fourth) = grid.get(row + 3, col + 3)
                                {
                                    if *fourth == 'S'
                                    {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                 // Check right-up
                 if let Some(second) = grid.get(row as i32 - 1, col + 1)
                 {
                     if *second == 'M'
                     {
                         if let Some(third) = grid.get(row as i32 - 2, col + 2)
                         {
                             if *third == 'A'
                             {
                                 if let Some(fourth) = grid.get(row as i32 - 3, col + 3)
                                 {
                                     if *fourth == 'S'
                                     {
                                         xmas_counter += 1;
                                     }
                                 }
                             }
                         }
                     }
                 }
                // Check left
                if let Some(second) = grid.get(row, col as i32 - 1)
                {
                    if *second == 'M'
                    {
                        if let Some(third) = grid.get(row, col as i32 - 2)
                        {
                            if *third == 'A'
                            {
                                if let Some(fourth) = grid.get(row, col as i32 - 3)
                                {
                                    if *fourth == 'S'
                                    {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                // Check left-down
                if let Some(second) = grid.get(row + 1, col as i32 - 1)
                {
                    if *second == 'M'
                    {
                        if let Some(third) = grid.get(row + 2, col as i32 - 2)
                        {
                            if *third == 'A'
                            {
                                if let Some(fourth) = grid.get(row + 3, col as i32 - 3)
                                {
                                    if *fourth == 'S'
                                    {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                // Check left-up
                if let Some(second) = grid.get(row as i32 - 1, col as i32 - 1)
                {
                    if *second == 'M'
                    {
                        if let Some(third) = grid.get(row as i32 - 2, col as i32 - 2)
                        {
                            if *third == 'A'
                            {
                                if let Some(fourth) = grid.get(row as i32 - 3, col as i32 - 3)
                                {
                                    if *fourth == 'S'
                                    {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                // Check down
                if let Some(second) = grid.get(row + 1, col)
                {
                    if *second == 'M'
                    {
                        if let Some(third) = grid.get(row + 2, col)
                        {
                            if *third == 'A'
                            {
                                if let Some(fourth) = grid.get(row + 3, col)
                                {
                                    if *fourth == 'S'
                                    {
                                        xmas_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                // Check up
                if let Some(second) = grid.get(row as i32 - 1, col)
                {
                    if *second == 'M'
                    {
                        if let Some(third) = grid.get(row as i32 - 2, col)
                        {
                            if *third == 'A'
                            {
                                if let Some(fourth) = grid.get(row as i32 - 3, col)
                                {
                                    if *fourth == 'S'
                                    {
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
}
