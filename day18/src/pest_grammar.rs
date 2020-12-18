use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CalcParser;

fn eval(expression: Pairs<Rule>, prec_climber: &PrecClimber<Rule>) -> usize {
    prec_climber.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.as_str().parse::<usize>().unwrap(),
            Rule::expr => eval(pair.into_inner(), prec_climber),
            _ => unreachable!(),
        },
        |lhs: usize, op: Pair<Rule>, rhs: usize| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::multiply => lhs * rhs,
            _ => unreachable!(),
        },
    )
}

pub fn solve(input: &str, multiply_precedence: bool) -> usize {
    let file = CalcParser::parse(Rule::file, input)
        .unwrap()
        .next()
        .unwrap();

    let prec_climber: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        if multiply_precedence {
            PrecClimber::new(vec![
                Operator::new(multiply, Left),
                Operator::new(add, Left),
            ])
        } else {
            PrecClimber::new(vec![
                Operator::new(multiply, Left) | Operator::new(add, Left),
            ])
        }
    };

    let mut sum = 0;

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::line => {
                sum = sum + eval(line.into_inner(), &prec_climber);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_a() {
        assert_eq!(solve("1 + 2 * 3 + 4 * 5 + 6\n", false), 71)
    }

    #[test]
    fn test_eval_b() {
        assert_eq!(solve("1 + (2 * 3) + (4 * (5 + 6))\n", false), 51)
    }

    #[test]
    fn test_eval_c() {
        assert_eq!(solve("2 * 3 + (4 * 5)\n", false), 26)
    }

    #[test]
    fn test_eval_d() {
        assert_eq!(solve("5 + (8 * 3 + 9 + 3 * 4 * 3)\n", false), 437)
    }

    #[test]
    fn test_eval_e() {
        assert_eq!(
            solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))\n", false),
            12240
        )
    }

    #[test]
    fn test_eval_f() {
        assert_eq!(
            solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2\n", false),
            13632
        )
    }

    #[test]
    fn test_eval2_a() {
        assert_eq!(solve("1 + (2 * 3) + (4 * (5 + 6))\n", true), 51)
    }

    #[test]
    fn test_eval2_b() {
        assert_eq!(solve("2 * 3 + (4 * 5)\n", true), 46)
    }

    #[test]
    fn test_eval2_c() {
        assert_eq!(solve("5 + (8 * 3 + 9 + 3 * 4 * 3)\n", true), 1445)
    }

    #[test]
    fn test_eval2_d() {
        assert_eq!(
            solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))\n", true),
            669060
        )
    }

    #[test]
    fn test_eval2_e() {
        assert_eq!(
            solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2\n", true,),
            23340
        )
    }
}
