use std::fs::read_to_string;

const GRID_SIZE: usize = 100;

pub fn part1() {
    let mut grid: [[bool; GRID_SIZE]; GRID_SIZE] = [[false; GRID_SIZE]; GRID_SIZE];

    for (index_y, line) in read_to_string("data/day18.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        for (index_x, char) in line.chars().enumerate() {
            grid[index_y][index_x] = matches!(char, '#');
        }
    }

    print_grid(&grid);

    println!();

    for _ in 0..100 {
        run_grid(&mut grid, false);
    }

    let count = grid.iter().flatten().filter(|x| **x).count();

    println!("{}", count);
}

pub fn run_grid(grid: &mut [[bool; GRID_SIZE]; GRID_SIZE], lock_corners: bool) {
    let mut working_grid = *grid;

    let bounds = GRID_SIZE as i32;

    for x in 0..bounds {
        for y in 0..bounds {
            let x_index = x as usize;
            let y_index = y as usize;

            if lock_corners && (x == bounds - 1 || x == 0) && (y == bounds - 1 || y == 0) {
                working_grid[x_index][y_index] = true;

                continue;
            }

            let mut neighbours = 0;
            neighbours += get_clamped_value(grid, x, y - 1, lock_corners);
            neighbours += get_clamped_value(grid, x + 1, y - 1, lock_corners);
            neighbours += get_clamped_value(grid, x + 1, y, lock_corners);
            neighbours += get_clamped_value(grid, x + 1, y + 1, lock_corners);
            neighbours += get_clamped_value(grid, x, y + 1, lock_corners);
            neighbours += get_clamped_value(grid, x - 1, y + 1, lock_corners);
            neighbours += get_clamped_value(grid, x - 1, y, lock_corners);
            neighbours += get_clamped_value(grid, x - 1, y - 1, lock_corners);

            working_grid[x_index][y_index] = match grid[x_index][y_index] {
                true => matches!(neighbours, 2 | 3),
                false => matches!(neighbours, 3),
            };
        }
    }

    *grid = working_grid;
}

fn print_grid(grid: &[[bool; GRID_SIZE]; GRID_SIZE]) {
    for y in grid.iter().take(GRID_SIZE) {
        for x in y {
            let char = match *x {
                true => '#',
                false => '.',
            };

            print!("{}", char);
        }

        println!();
    }
}

pub fn get_clamped_value(
    grid: &mut [[bool; GRID_SIZE]; GRID_SIZE],
    x: i32,
    y: i32,
    lock_corners: bool,
) -> i32 {
    let bounds = GRID_SIZE as i32;

    if lock_corners && (x == bounds - 1 || x == 0) && (y == bounds - 1 || y == 0) {
        return 1;
    }

    if x < 0 || y < 0 {
        return 0;
    }

    let x_index = x as usize;
    let y_index = y as usize;

    if x_index > GRID_SIZE - 1 || y_index > GRID_SIZE - 1 {
        return 0;
    }

    match grid[x_index][y_index] {
        true => 1,
        false => 0,
    }
}

pub fn part2() {
    let mut grid: [[bool; GRID_SIZE]; GRID_SIZE] = [[false; GRID_SIZE]; GRID_SIZE];

    for (index_y, line) in read_to_string("data/day18.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        for (index_x, char) in line.chars().enumerate() {
            grid[index_y][index_x] = matches!(char, '#');
        }
    }

    grid[0][0] = true;
    grid[0][GRID_SIZE - 1] = true;
    grid[GRID_SIZE - 1][0] = true;
    grid[GRID_SIZE - 1][GRID_SIZE - 1] = true;

    print_grid(&grid);

    println!();

    for _ in 0..100 {
        run_grid(&mut grid, true);
    }

    let count = grid.iter().flatten().filter(|x| **x).count();

    println!("{}", count);
}
