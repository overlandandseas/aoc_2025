use std::usize;

fn part_one(input: &str) -> usize {
    let shelves: Vec<Vec<char>> = input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    shelves
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|(y, item)| {
                    if **item != '@' {
                        return false;
                    }
                    count_toilet_paper(&shelves, x as isize, *y as isize) < 4
                })
                .count()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let mut shelves: Vec<Vec<char>> = input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let mut collected = 0;

    loop {
        let count: usize = shelves
            .clone()
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(y, item)| {
                        if **item != '@' {
                            return false;
                        }
                        if count_toilet_paper(&shelves, x as isize, *y as isize) < 4 {
                            shelves[x][*y] = '.';
                            true
                        } else {
                            false
                        }
                    })
                    .count()
            })
            .sum();
        if count == 0 {
            break;
        }
        collected += count;
    }

    collected
}

fn count_toilet_paper(shelves: &Vec<Vec<char>>, row: isize, column: isize) -> usize {
    let check_list: Vec<(isize, isize)> = vec![
        (row - 1, column - 1),
        (row - 1, column),
        (row - 1, column + 1),
        (row, column - 1),
        (row, column + 1),
        (row + 1, column - 1),
        (row + 1, column),
        (row + 1, column + 1),
    ];

    check_list
        .iter()
        .filter(|(x, y)| {
            if *x < 0 || *y < 0 {
                return false;
            }
            shelves
                .get(*x as usize)
                .is_some_and(|row| row.get(*y as usize).is_some_and(|car| *car == '@'))
        })
        .count()
}

fn main() {
    let input = include_str!("../input");

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT), 13);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT), 43);
    }
}
