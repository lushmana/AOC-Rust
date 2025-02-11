advent_of_code::solution!(2);

use itertools::Itertools;
enum Direction {
    Increasing,
    Decreasing,
}

type Report = Vec<i32>;

fn check_safe(report: &Report) -> bool {
    let mut direction: Option<Direction> = None;
    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        match diff.signum() {
            -1 => match direction {
                Some(Direction::Increasing) => {
                    return false;
                }
                Some(Direction::Decreasing) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return false;
                    } else {
                        continue;
                    }
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return false;
                    } else {
                        direction = Some(Direction::Decreasing);
                        continue;
                    }
                    
                }
            },
            1 => match direction {
                Some(Direction::Increasing) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return false;
                    } else {
                        continue;
                    }
                }
                Some(Direction::Decreasing) => {
                    return false;
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return false;
                    } else {
                        direction = Some(Direction::Increasing);
                        continue;
                    }
                    
                }
            },
            0 => {
                return false;
            },
            _ => panic!("Should never have a non -1,1,0 number")
        }
    };
    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut rst = 0;
    for line in input.lines() {
        let report = line.split_whitespace().map(|n|n.parse::<i32>().unwrap()).collect::<Report>();
        if check_safe(&report) {
            rst += 1;
        }
    }
    Some(rst)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rst = 0;
    for line in input.lines() {
        let report = line.split_whitespace().map(|n|n.parse::<i32>().unwrap()).collect::<Report>();
        
        if check_safe(&report) {
            rst += 1;
        } else {
            for index in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(index);
                if check_safe(&new_report) {
                    rst += 1;
                    break;
                }
            }
        }
    }
    Some(rst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
