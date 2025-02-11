advent_of_code::solution!(3);

use chumsky::prelude::*;

#[derive(Debug)]
enum Expr {
    Mul(u32, u32),
}

fn parser() -> impl Parser<&str, Expr {
    
}

pub fn part_one(input: &str) -> Option<u64> {
    match parser().parse(&input).into_result() {
        Ok(ast) => match eval(&ast, &mut Vec::new(), &mut Vec::new()) {
            Ok(output) => Some(output),
            Err(eval_err) => println!("Evaluation error: {}", eval_err),
        },
        Err(parse_errs) = parse_errs
            .into_iter()
            .for_each(|e| println!("Parse error: {}", e)),
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
