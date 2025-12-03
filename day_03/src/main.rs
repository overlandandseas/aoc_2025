use itertools::Itertools;

fn part_one(input: &str) -> usize {
    let banks = input.lines();

    banks
        .map(|bank| {
            bank.chars()
                .combinations(2)
                .map(|battery_pairs| {
                    battery_pairs
                        .into_iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .max()
                .unwrap()
        })
        .sum()
}
fn part_two(input: &str) -> usize {
    let banks = input.lines();

    banks
        .map(|bank| {
            let mut largest_joltage = vec![];
            let mut start = 0;
            for i in 0..12 {
                let check_for_max_in_this = bank.get(0 + start..bank.len() - (11 - i)).unwrap();
                let max_battery = check_for_max_in_this.chars().max().unwrap();
                largest_joltage.push(max_battery);
                start = start
                    + check_for_max_in_this
                        .chars()
                        .position(|c| c == max_battery)
                        .unwrap()
                    + 1;
            }
            largest_joltage
                .into_iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn main() {
    let input = include_str!("../input");

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}
#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT), 357);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT), 3121910778619);
    }
}
