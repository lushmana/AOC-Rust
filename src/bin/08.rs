advent_of_code::solution!(8);

use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign, Sub},
};

struct Map {
    grid: Vec<Vec<u8>>,
    antennas: HashMap<u8, Vec<Point>>,
}

impl Map {
    fn get(&self, Point(x, y): Point) -> Option<u8> {
        self.grid.get(x as usize)?.get(y as usize).copied()
    }

    fn get_pairs(&self) -> Vec<(Point, Point)> {
        self.antennas
            .values()
            .flat_map(|antenna| {
                antenna
                    .iter()
                    .flat_map(|&p1| antenna.iter().map(move |&p2| (p1, p2)))
                    .filter(|(p1, p2)| p1 != p2)
            })
            .collect()
    }

    fn signal(&self) -> HashSet<Point> {
        let mut antinodes = HashSet::new();

        for (p1, p2) in self.get_pairs() {
            [p1 + (p1 - p2), p2 + (p2 - p1)]
                .into_iter()
                .filter(move |&antinode| self.get(antinode).is_some())
                .for_each(|antinode| {
                    antinodes.insert(antinode);
                })
        }
        antinodes
    }

    fn harmonics(&self) -> HashSet<Point> {
        let mut antinodes = HashSet::new();

        for (p1, p2) in self.get_pairs() {
            [(p1, p1 - p2), (p2, p2 - p1)]
                .into_iter()
                .for_each(|(mut point, offset)| {
                    while self.get(point).is_some() {
                        antinodes.insert(point);
                        point += offset;
                    }
                })
        }
        antinodes
    }
}

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1)
    }
}

impl From<&str> for Map {
    fn from(puzzle: &str) -> Self {
        let grid: Vec<Vec<u8>> = puzzle.lines().map(|row| row.bytes().collect()).collect();

        let antennas = grid
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter().enumerate().filter_map(move |(y, &cell)| {
                    (cell != b'.').then_some((cell, Point(x as i32, y as i32)))
                })
            })
            .fold(
                HashMap::new(),
                |mut antennas: HashMap<u8, Vec<Point>>, (sym, pos)| {
                    antennas.entry(sym).or_default().push(pos);
                    antennas
                },
            );

        Self { grid, antennas }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(Map::from(input).signal().len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(Map::from(input).harmonics().len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
