use std::ops::RangeInclusive;

fn part_one(input: &str) -> usize {
    let ranges = input.split(",");
    ranges
        .map(|range| {
            let (start_str, end_str) = range.split_once("-").unwrap();

            let range_inc: RangeInclusive<usize> =
                RangeInclusive::new(start_str.parse().unwrap(), end_str.parse().unwrap());

            range_inc.fold(0, |acc, n| {
                let n_str = n.to_string();
                let (left, right) = n_str.split_at(n_str.len() / 2);
                if left == right { acc + n } else { acc }
            })
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let ranges = input.split(",");
    ranges
        .map(|range| {
            let (start_str, end_str) = range.split_once("-").unwrap();

            let range_inc: RangeInclusive<usize> =
                RangeInclusive::new(start_str.parse().unwrap(), end_str.parse().unwrap());

            range_inc.fold(0, |acc, n| {
                let n_str = n.to_string();
                for i in 1..=n_str.len() / 2 {
                    let (for_this, check_this) = n_str.split_at(i);
                    if for_this.repeat(check_this.len() / for_this.len()) == check_this {
                        return acc + n;
                    }
                }

                acc
            })
        })
        .sum()
}

fn main() {
    let input = include_str!("../input").trim();

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT), 1227775554);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT), 4174379265);
    }
}
