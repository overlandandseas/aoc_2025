use std::collections::{HashMap, HashSet};

fn part_one(input: &str) -> usize {
    let mut lines = input.lines();
    let mut tachyon_beams: HashSet<usize> = HashSet::new();
    let mut number_of_splits = 0;

    tachyon_beams.insert(lines.next().unwrap().find("S").unwrap());

    for line in lines.map(str::chars) {
        for (idx, manifold) in line.enumerate() {
            if manifold == '^' && tachyon_beams.contains(&idx) {
                tachyon_beams.remove(&idx);
                tachyon_beams.insert(idx + 1);
                tachyon_beams.insert(idx - 1);
                number_of_splits += 1;
            }
        }
    }

    number_of_splits
}

fn part_two(input: &str) -> usize {
    let mut lines = input.lines();
    let mut tachyon_beams: HashMap<usize, usize> = HashMap::new();
    let mut universe_num = 1;

    let initial_position = lines.next().unwrap().find("S").unwrap();

    tachyon_beams.insert(initial_position, 1);

    for line in lines.map(str::chars) {
        for (idx, manifold) in line.enumerate() {
            if manifold == '^' && tachyon_beams.contains_key(&idx) {
                let current_beams: usize = *tachyon_beams.get(&idx).unwrap();

                tachyon_beams
                    .entry(idx)
                    .and_modify(|timelines| *timelines -= current_beams)
                    .or_insert(0);
                tachyon_beams
                    .entry(idx + 1)
                    .and_modify(|timelines| *timelines += current_beams)
                    .or_insert(current_beams);
                tachyon_beams
                    .entry(idx - 1)
                    .and_modify(|timelines| *timelines += current_beams)
                    .or_insert(current_beams);
                universe_num += current_beams;
            }
        }
    }

    universe_num
}
fn main() {
    let input = include_str!("../input");
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
    // 3072 too low
}
#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT), 21);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT), 40);
    }
}
