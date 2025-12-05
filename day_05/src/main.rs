use std::ops::RangeInclusive;

fn part_one(input: &str) -> usize {
    let (fresh_ids_ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges: Vec<RangeInclusive<usize>> = fresh_ids_ranges
        .lines()
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    ingredients
        .lines()
        .map(|id| id.parse().unwrap())
        .filter(|num| ranges.iter().find(|range| range.contains(num)).is_some())
        .count()
}

fn part_two(input: &str) -> usize {
    let fresh_ids_ranges = input.split_once("\n\n").unwrap().0;

    let mut ranges: Vec<RangeInclusive<usize>> = vec![];

    'outer: for range_str in fresh_ids_ranges.lines() {
        let (start_str, end_str) = range_str.split_once("-").unwrap();

        let mut start: usize = start_str.parse().unwrap();
        let mut end: usize = end_str.parse().unwrap();

        for range in ranges.clone().iter() {
            if range.contains(&start) && range.contains(&end) {
                continue 'outer;
            }
            if range.contains(&start) {
                start = range.end() + 1;
            }

            if range.contains(&end) {
                end = range.start() - 1;
            }
        }

        ranges.push(RangeInclusive::new(start, end));
    }

    ranges
        .iter()
        .filter(|range| {
            ranges
                .iter()
                .find(|r| {
                    (r.start() != range.start() && r.end() != range.end())
                        && (r.contains(range.start()) || r.contains(range.end()))
                })
                .is_none()
        })
        .map(|range| range.end() + 1 - range.start())
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

    static INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT), 3);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT), 14);
    }
}
