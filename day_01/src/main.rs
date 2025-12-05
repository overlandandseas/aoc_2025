fn part_one(input: &str) -> usize {
    let mut pos: isize = 50;
    let mut zeros = 0;
    let rotations = input.lines();

    for rot in rotations {
        let (direction, amount_str) = rot.split_at(1);

        let amount: isize = amount_str.parse().unwrap();
        match direction {
            "L" => pos = pos - amount,
            "R" => pos = pos + amount,
            _ => panic!("No idea"),
        }

        if pos < 0 {
            pos = 100 + pos % -100;
        }
        pos = pos % 100;

        if pos == 0 {
            zeros = zeros + 1;
        }
    }

    zeros
}

fn part_two(input: &str) -> isize {
    let mut pos: isize = 50;
    let mut zeros = 0;
    let rotations = input.lines();

    for rot in rotations {
        let (direction, amount_str) = rot.split_at(1);

        let amount: isize = amount_str.parse().unwrap();
        match direction {
            "L" => {
                if pos == 0 {
                    zeros = zeros + amount / 100
                } else {
                    zeros = zeros + (100 - pos + amount) / 100;
                }
                pos = pos - amount
            }
            "R" => {
                zeros = zeros + (pos + amount) / 100;
                pos = pos + amount
            }
            _ => panic!("No idea"),
        }

        if pos < 0 {
            pos = 100 + pos % -100;
        }
        pos = pos % 100;
    }

    zeros
}

fn main() {
    let input = include_str!("../input");

    println!("Part One: {}", part_one(input));
    println!("Part Two: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT), 3);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT), 6);
    }
}
