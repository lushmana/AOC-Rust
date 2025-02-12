advent_of_code::solution!(4);
use glam::IVec2;
use std::collections::HashMap;

const DIRECTIONS: [[IVec2; 3]; 8] = [
    [IVec2::new(0, 1),   IVec2::new(0, 2),   IVec2::new(0, 3)],
    [IVec2::new(0, -1),  IVec2::new(0, -2),  IVec2::new(0, -3), ],
    [IVec2::new(1, 1),   IVec2::new(2, 2),   IVec2::new(3, 3)],
    [IVec2::new(1, -1),  IVec2::new(2, -2),  IVec2::new(3, -3), ],
    [IVec2::new(-1, 1),  IVec2::new(-2, 2),  IVec2::new(-3, 3), ],
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3), ],
    [IVec2::new(1, 0),   IVec2::new(2, 0),   IVec2::new(3, 0)],
    [IVec2::new(-1, 0),  IVec2::new(-2, 0),  IVec2::new(-3, 0),  ],
];

pub fn part_one(input: &str) -> Option<usize> {
    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, value)| {
                (IVec2::new(x as i32, y as i32), value)
            },
        )
        }).collect::<HashMap<IVec2, char>>();
    
    println!("Positions: {:?}", positions);

    let mas = ['M', 'A', 'S'];

    let result: usize = positions
        .iter()
        .filter(|(_position, value)| **value == 'X')
        .map(|(position, _value)| {
            let count = DIRECTIONS
                .iter()
                .map(|mas_positions| {
                    mas_positions
                        .iter()
                        .map(|offset| {
                            positions
                                .get(&(position + offset))
                        })
                        .enumerate()
                        .all(|(index, value)| {
                            mas.get(index) == value
                        })
                })
                .filter(|b| *b)
                .count();
//=            info!(?position, ?value, count);
            count
        })
        .sum();
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
