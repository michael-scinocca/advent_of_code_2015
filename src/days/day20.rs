pub fn part1() {
    let mut house = 1;
    let mut max_presents = 0;

    loop {
        let mut presents = 0;

        for elf in 1..=house {
            if house % elf == 0 {
                presents += elf * 10;
            }
        }

        if presents > max_presents {
            max_presents = presents;

            println!("House {} got {:08} presents.", house, presents);
        }

        if presents >= 34000000 {
            break;
        }

        house += 1;
    }

    println!("{}", house);
}

pub fn part2() {
    let mut house = 1;
    let mut max_presents = 0;

    loop {
        let mut presents = 0;

        for elf in 1..=house {
            if house / elf <= 50 && house % elf == 0 {
                presents += elf * 11;
            }
        }

        if presents > max_presents {
            max_presents = presents;

            println!("House {} got {:08} presents.", house, presents);
        }

        if presents >= 34000000 {
            break;
        }

        house += 1;
    }

    println!("{}", house);
}
