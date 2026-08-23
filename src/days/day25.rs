enum RowColMode {
    Row,
    Col,
}

pub fn part1() {
    let mut rowcol_mode = RowColMode::Row;
    let mut starting_row = 1;
    let mut rowcol = (1, 1);

    let mut code = 20151125;

    for _ in 0..100000000 {
        match rowcol_mode {
            RowColMode::Row => {
                rowcol.0 = starting_row + 1;
                rowcol.1 = 1;
                starting_row = rowcol.0;
                rowcol_mode = RowColMode::Col;
            }
            RowColMode::Col => {
                rowcol.0 -= 1;
                rowcol.1 += 1;

                if rowcol.1 == starting_row {
                    rowcol_mode = RowColMode::Row;
                }
            }
        }

        code = next_code(code);

        if rowcol.0 == 3010 && rowcol.1 == 3019 {
            println!("({},{}) {code}", rowcol.0, rowcol.1);

            break;
        }
    }
}

fn next_code(code: u32) -> u32 {
    ((code as u64 * 252533) % 33554393) as u32
}
