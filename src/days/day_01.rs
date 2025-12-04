fn parse_input(input: &str) -> impl Iterator<Item = i32> {
    input.lines().map(|line| {
        let (dir_str, num_str) = line.split_at(1);
        let num = num_str.parse::<i32>().unwrap();
        match dir_str {
            "L" => -num,
            "R" => num,
            _ => panic!(),
        }
    })
}

pub fn part_1(input: &str) -> String {
    let mut position = 50;

    parse_input(input)
        .filter(|&rotation| {
            position = (position + rotation).rem_euclid(100);
            position == 0
        })
        .count()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    let mut position: i32 = 50;

    parse_input(input)
        .map(|rotation| {
            position = position.rem_euclid(100);
            if position == 0 && rotation < 0 {
                position += 100;
            }
            position += rotation;
            ((position == 0) as i32) + position.abs() / 100 + ((position < 0) as i32)
        })
        .sum::<i32>()
        .to_string()
}
