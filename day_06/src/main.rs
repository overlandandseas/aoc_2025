fn part_one(input: &str) -> usize {
    let mut lines = input.lines().rev();

    let operations: Vec<&str> = lines.next().unwrap().split_whitespace().collect();

    let mut answers = vec![0; operations.len()];

    for line in lines {
        for (idx, num_as_str) in line.trim().split_whitespace().enumerate() {
            let num: usize = num_as_str.parse().unwrap();

            match operations[idx] {
                "*" => {
                    answers[idx] = if answers[idx] == 0 {
                        num
                    } else {
                        answers[idx] * num
                    };
                }
                "+" => answers[idx] += num,
                _ => panic!("Unsupported operation"),
            }
        }
    }

    answers.iter().sum()
}

fn part_two(input: &str) -> usize {
    let mut lines = input.lines().rev();

    let mut operations = lines.next().unwrap().split_whitespace();

    let mut flipped: Vec<Vec<char>> = vec![vec![]; input.find("\n").unwrap()];

    for line in lines.rev() {
        line.chars().enumerate().for_each(|(idx, c)| {
            flipped[idx].push(c);
        });
    }


    let mut answer = 0;

    let mut temp_answers = vec![];
    for num_arr in flipped {
        if let Ok(num) = num_arr
            .iter()
            .clone()
            .collect::<String>()
            .trim()
            .parse::<usize>()
        {
            temp_answers.push(num);
        } else {
            match operations.next() {
                Some("+") => answer += temp_answers.iter().sum::<usize>(),
                Some("*") => answer += temp_answers.iter().product::<usize>(),
                _ => panic!("Operation not supported")
            }
            temp_answers = vec![];
        }
    }
    match operations.next() {
        Some("+") => answer += temp_answers.iter().sum::<usize>(),
        Some("*") => answer += temp_answers.iter().product::<usize>(),
        _ => panic!("Operation not supported")
    }

    answer
}

fn main() {
    let input = include_str!("../input");

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT), 4277556);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT), 3263827);
    }
}
