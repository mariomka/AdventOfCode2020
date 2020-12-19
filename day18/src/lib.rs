use std::collections::VecDeque;

pub mod pest_grammar;

type Tokens<'a> = VecDeque<&'a str>;

pub mod part1 {
    use super::Tokens;

    pub fn solve(input: &Vec<&str>) -> usize {
        input.iter().fold(0, |sum, line| {
            let line = line.replace("(", "( ").replace(")", " )");

            sum + expression(&mut line.split(" ").collect())
        })
    }

    // Grammar
    // expression = term (('+' | '*') term)*
    // term = 0-9+ | ( expression )

    fn expression(tokens: &mut Tokens) -> usize {
        let mut lhs = term(tokens);

        loop {
            let token = tokens.pop_front().unwrap_or("");

            match token {
                "*" => {
                    lhs = lhs * term(tokens);
                }
                "+" => {
                    lhs = lhs + term(tokens);
                }
                _ => return lhs,
            }
        }
    }

    fn term(tokens: &mut Tokens) -> usize {
        let token = tokens.pop_front().unwrap();

        if token == "(" {
            expression(tokens)
        } else {
            token.parse::<usize>().unwrap()
        }
    }
}

pub mod part2 {
    use super::Tokens;

    pub fn solve(input: &Vec<&str>) -> usize {
        input.iter().fold(0, |sum, line| {
            let line = line.replace("(", "( ").replace(")", " )");

            sum + expression(&mut line.split(" ").collect())
        })
    }

    // Grammar
    // expression = factor | factor '*' expression
    // factor = term | term '+' expression
    // term = 0-9+ | ( expression )

    fn expression(tokens: &mut Tokens) -> usize {
        let lhs = factor(tokens);

        if tokens.len() > 0 && tokens[0] == "*" {
            tokens.pop_front();
            lhs * expression(tokens)
        } else {
            lhs
        }
    }

    fn factor(tokens: &mut Tokens) -> usize {
        let lhs = term(tokens);

        if tokens.len() > 0 && tokens[0] == "+" {
            tokens.pop_front();
            lhs + factor(tokens)
        } else {
            lhs
        }
    }

    fn term(tokens: &mut Tokens) -> usize {
        let token = tokens.pop_front().unwrap();

        if token == "(" {
            let result = expression(tokens);
            tokens.pop_front(); // It should be a ')'

            result
        } else {
            token.parse::<usize>().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_a() {
        assert_eq!(part1::solve(&vec!["1 + 2 * 3 + 4 * 5 + 6"]), 71)
    }

    #[test]
    fn test_eval_b() {
        assert_eq!(part1::solve(&vec!["1 + (2 * 3) + (4 * (5 + 6))"]), 51)
    }

    #[test]
    fn test_eval_c() {
        assert_eq!(part1::solve(&vec!["2 * 3 + (4 * 5)"]), 26)
    }

    #[test]
    fn test_eval_d() {
        assert_eq!(part1::solve(&vec!["5 + (8 * 3 + 9 + 3 * 4 * 3)"]), 437)
    }

    #[test]
    fn test_eval_e() {
        assert_eq!(
            part1::solve(&vec!["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"]),
            12240
        )
    }

    #[test]
    fn test_eval_f() {
        assert_eq!(
            part1::solve(&vec!["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"]),
            13632
        )
    }

    #[test]
    fn test_eval2_a() {
        assert_eq!(part2::solve(&vec!["1 + (2 * 3) + (4 * (5 + 6))"]), 51)
    }

    #[test]
    fn test_eval2_b() {
        assert_eq!(part2::solve(&vec!["2 * 3 + (4 * 5)"]), 46)
    }

    #[test]
    fn test_eval2_c() {
        assert_eq!(part2::solve(&vec!["5 + (8 * 3 + 9 + 3 * 4 * 3)"]), 1445)
    }

    #[test]
    fn test_eval2_d() {
        assert_eq!(
            part2::solve(&vec!["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"]),
            669060
        )
    }

    #[test]
    fn test_eval2_e() {
        assert_eq!(
            part2::solve(&vec!["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"]),
            23340
        )
    }
}
