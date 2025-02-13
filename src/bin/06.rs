use std::{collections::HashSet, ops::Add};

advent_of_code::solution!(6);

struct Lab {
    grid: Vec<Vec<u8>>,
    guard: Guard,
}
impl Lab {
    fn next(&mut self) -> Option<()> {
        let next = self.guard.pos + self.guard.dir.offset();

        match self.get(next) {
            Some(b'#' | b'O') => {
                self.guard.dir = self.guard.dir.turn();
                Some(())
            }
            Some(_) => {
                self.guard.pos = next;
                Some(())
            }
            None => None,
        }
    }

    fn walk(&mut self) -> HashSet<Pos> {
        let mut visited = HashSet::new();

        loop {
            visited.insert(self.guard.pos);

            if self.next().is_none() {
                break;
            }
        }
        visited
    }

    fn is_looping(&mut self, origin: Guard, obstacle: Pos) -> bool {
        let mut visited = HashSet::new();

        self.guard = origin;
        self.set(obstacle, b'O');

        let looping = loop {
            if !visited.insert((self.guard.pos, self.guard.dir)) {
                break true;
            }

            if self.next().is_none() {
                break false;
            }
        };
        self.set(obstacle, b'.');
        looping
    }

    fn get(&self, Pos(x, y): Pos) -> Option<u8> {
        self.grid.get(x as usize)?.get(y as usize).copied()
    }

    fn set(&mut self, Pos(x, y): Pos, val: u8) {
        if let Some(cell) = self
            .grid
            .get_mut(x as usize)
            .and_then(|row| row.get_mut(y as usize))
        {
            *cell = val;
        }
    }
}
impl From<&str> for Lab {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
        let pos = grid
            .iter()
            .enumerate()
            .find_map(|(x, row)| {
                row.iter()
                    .position(|&c| c == b'^')
                    .map(|y| Pos(x as i32, y as i32))
            })
            .unwrap_or_default();
        Self {
            grid,
            guard: Guard {
                pos,
                dir: Direction::Up,
            },
        }
    }
}

#[derive(Copy, Clone)]
struct Guard {
    pos: Pos,
    dir: Direction,
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

impl Add<Off> for Pos {
    type Output = Self;

    fn add(self, Off(dx, dy): Off) -> Self::Output {
        Pos(self.0 + dx, self.1 + dy)
    }
}
struct Off(i32, i32);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}
impl Direction {
    fn offset(self) -> Off {
        match self {
            Direction::Up => Off(-1, 0),
            Direction::Down => Off(1, 0),
            Direction::Right => Off(0, 1),
            Direction::Left => Off(0, -1),
        }
    }

    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let count = Lab::from(input).walk().len() as u64;
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lab = Lab::from(input);
    let origin = lab.guard;

    Some(
        lab.walk()
            .iter()
            .filter(|&&obstacle| lab.is_looping(origin, obstacle))
            .count() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
