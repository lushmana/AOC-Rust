advent_of_code::solution!(3);

use std::iter::from_fn;

pub fn part_one(input: &str) -> Option<u64> {
    let mut iter = input.chars().peekable();
    let mut result :u64 = 0;

    while let Some(ch) = iter.next() {
        match ch {
            'm' => {
                if let Some(_u) = iter.next_if(|s| *s == 'u') {
                    if let Some(_l) = iter.next_if(|s| *s == 'l') {
                        if let Some(_lb) = iter.next_if(|s| *s == '(') {
            
                            let a: u16 = from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit()))
                                .collect::<String>()
                                .parse()
                                .unwrap();
                
                            if let Some(_comma) = iter.next_if(|s| *s == ',') {

                                let b: u16 = from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit()))
                                    .collect::<String>()
                                    .parse()
                                    .unwrap();

                                if let Some(_rb) = iter.next_if(|s| *s == ')') {
                                    result += a as u64 * b as u64;    
                                }       
                            }
                        }
                    }
                }
            },
            _ => continue,
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut iter = input.chars().peekable();
    let mut result :u64 = 0;
    let mut enabled :bool = true;

    while let Some(ch) = iter.next() {
        match ch {
            'd' => {
                if let Some(_o) = iter.next_if(|s| *s == 'o') {
                    if let Some(_n) = iter.next_if(|s| *s == 'n') {
                        if let Some(_q) = iter.next_if(|s| *s == '\'') {
                             if let Some(_t) = iter.next_if(|s| *s == 't') {
                                if let Some(_lb) = iter.next_if(|s| *s == '(') {
                                     if let Some(_rb) = iter.next_if(|s| *s == ')') {
                                        enabled = false;
                                     }
                                }
                             }
                        }
                    }
                    if let Some(_lb) = iter.next_if(|s| *s == '(') {
                         if let Some(_rb) = iter.next_if(|s| *s == ')') {
                            enabled = true;
                         }
                    }
                }  
            },
            'm' => {
                if let Some(_u) = iter.next_if(|s| *s == 'u') {
                    if let Some(_l) = iter.next_if(|s| *s == 'l') {
                        if let Some(_lb) = iter.next_if(|s| *s == '(') {
            
                            let a: u16 = from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit()))
                                .collect::<String>()
                                .parse()
                                .unwrap();
                
                            if let Some(_comma) = iter.next_if(|s| *s == ',') {

                                let b: u16 = from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit()))
                                    .collect::<String>()
                                    .parse()
                                    .unwrap();

                                if let Some(_rb) = iter.next_if(|s| *s == ')') {
                                    if enabled {
                                        result += a as u64 * b as u64;
                                    }
                                }       
                            }
                        }
                    }
                }
            },
            _ => continue,
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
