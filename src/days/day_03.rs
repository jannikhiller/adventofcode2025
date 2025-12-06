use crate::util::parse_grid_rows;

pub fn part_1(input: &str) -> String {
    let grid = parse_grid_rows::<i32>(input, "");
    let mut sum = 0;

    for bank in &grid {
        let b1 = bank[..bank.len() - 1].iter().max().unwrap();
        let i_b1 = bank.iter().position(|v| v == b1).unwrap();

        let b2 = bank[i_b1 + 1..].iter().max().unwrap();
        sum += b1 * 10 + b2;
    }

    sum.to_string()
}

pub fn find_first_largest<T: Ord + Copy>(bank: &[T], start: usize, end: usize) -> (usize, T) {
    let b1 = bank[start..end].iter().max().unwrap();
    let i_b1 = start + bank[start..end].iter().position(|v| v == b1).unwrap();
    (i_b1, *b1)
}

pub fn part_2(input: &str) -> String {
    let grid = parse_grid_rows::<i64>(input, "");
    let mut sum = 0;

    for bank in &grid {
        let mut digits = vec![];
        let mut start = 0;

        for i in 0..12 {
            let end = bank.len() - 11 + i;

            let (start_next, v) = find_first_largest(bank, start, end);

            digits.push(v);
            start = start_next + 1;
        }
        let mut joltage = 0;
        for d in digits {
            joltage *= 10;
            joltage += d;
        }
        sum += joltage;
    }

    sum.to_string()
}
