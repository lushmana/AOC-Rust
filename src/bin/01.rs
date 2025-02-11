advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lh = Vec::new();
    let mut rh = Vec::new();
    for line in input.lines() {
        let mut nums = line.split_whitespace(); 
        
        let lhn:i64 = nums.next().unwrap().parse::<i64>().unwrap();
        let rhn:i64 = nums.next().unwrap().parse::<i64>().unwrap();
        lh.push(lhn);
        rh.push(rhn);
    }
    lh.sort();
    rh.sort();
    let result: i64 = std::iter::zip(lh, rh)
        .map(|(l, r)| (l-r).abs())
        .sum();
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lh = Vec::new();
    let mut rh = Vec::new();
    for line in input.lines() {
        let mut nums = line.split_whitespace(); 
        
        let lhn = nums.next().unwrap().parse::<usize>().unwrap();
        let rhn = nums.next().unwrap().parse::<usize>().unwrap();
        lh.push(lhn);
        rh.push(rhn);
    }
    let result: usize = lh.iter()
        .map(|number| number * rh.iter().filter(|f|&number ==f).count() )
        .sum();
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
