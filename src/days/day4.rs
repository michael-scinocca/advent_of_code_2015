pub fn part1() {
    let mut number = 0;

    loop {
        let hash = md5::compute(format!("bgvyzdsv{}", number));

        if format!("{:x}", hash).starts_with("00000") {
            println!("{}", number);

            break;
        }

        number += 1;
    }
}

pub fn part2() {
    let mut number = 0;

    loop {
        let hash = md5::compute(format!("bgvyzdsv{}", number));

        if format!("{:x}", hash).starts_with("000000") {
            println!("{}", number);

            break;
        }

        number += 1;
    }
}
