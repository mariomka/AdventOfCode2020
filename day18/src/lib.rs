pub mod pest_grammar;

fn calc(lhs: usize, operator: &str, rhs: usize) -> usize {
    let res = match operator {
        "+" => lhs + rhs,
        "*" => lhs * rhs,
        _ => unreachable!(),
    };

    res
}

fn eval(expression: &str, start: usize) -> (usize, usize) {
    let pre_expression = expression
        .to_string()
        .clone()
        .replace("(", "( ")
        .replace(")", " )");

    let mut last_number = None;
    let mut last_operator = None;
    let mut offset = 0;

    for (index, part) in pre_expression.split(" ").enumerate().skip(start) {
        if offset > 0 {
            offset -= 1;
            continue;
        }

        match part {
            " " => {}
            "+" => {
                last_operator = Some(part);
            }
            "*" => {
                last_operator = Some(part);
            }
            "(" => {
                let (new_offset, number) = eval(expression, index + 1);
                offset = new_offset;

                if last_number.is_some() {
                    let result = calc(last_number.unwrap(), last_operator.unwrap(), number);

                    last_operator = None;
                    last_number = Some(result);
                } else {
                    last_number = Some(number);
                }
            }
            ")" => {
                return (index - start + 1, last_number.unwrap());
            }
            _ => {
                let number = part.to_string().parse().unwrap();
                if last_number.is_some() && last_operator.is_some() {
                    let result = calc(last_number.unwrap(), last_operator.unwrap(), number);

                    last_operator = None;
                    last_number = Some(result);
                } else {
                    last_number = Some(number);
                }
            }
        }
    }

    (0, last_number.unwrap())
}

fn eval2(expression: &str, start: usize, is_deep: bool, deep: usize) -> (usize, usize) {
    let pre_expression = expression
        .to_string()
        .clone()
        .replace("(", "( ")
        .replace(")", " )");

    let mut last_number = None;
    let mut last_operator = None;
    let mut offset = 0;
    let mut last_index = 0;

    for (index, part) in pre_expression.split(" ").enumerate().skip(start) {
        if offset > 0 {
            offset -= 1;
            last_index = index;
            continue;
        }

        match part {
            " " => {}
            "+" => {
                last_operator = Some(part);
            }
            "*" => {
                if is_deep {
                    return (index - start, last_number.unwrap());
                }

                let (new_offset, number) = eval2(expression, index + 1, true, deep + 1);

                offset = new_offset;

                let result = calc(last_number.unwrap(), "*", number);

                last_operator = None;
                last_number = Some(result);
            }
            "(" => {
                let (new_offset, number) = eval2(expression, index + 1, false, deep + 1);

                offset = new_offset;

                if last_number.is_some() && last_operator.is_some() {
                    let result = calc(last_number.unwrap(), last_operator.unwrap(), number);

                    last_operator = None;
                    last_number = Some(result);
                } else {
                    last_number = Some(number);
                }
            }
            ")" => {
                if is_deep {
                    return (index - start, last_number.unwrap());
                }

                return (index - start + 1, last_number.unwrap());
            }
            _ => {
                let number = part.to_string().parse().unwrap();
                if last_number.is_some() && last_operator.is_some() {
                    let result = calc(last_number.unwrap(), last_operator.unwrap(), number);

                    last_operator = None;
                    last_number = Some(result);
                } else {
                    if last_number.is_some() {
                        // it ends??
                        continue;
                    }

                    last_number = Some(number);
                }
            }
        }

        last_index = index;
    }

    if is_deep {
        return (last_index - start, last_number.unwrap());
    }

    (last_index - start + 1, last_number.unwrap())
}

pub fn part1(input: &Vec<&str>) -> usize {
    input
        .iter()
        .fold(0, |sum, expression| sum + eval(expression, 0).1)
}

pub fn part2(input: &Vec<&str>) -> usize {
    input
        .iter()
        .fold(0, |sum, expression| sum + eval2(expression, 0, false, 0).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_a() {
        assert_eq!(eval("1 + 2 * 3 + 4 * 5 + 6", 0).1, 71)
    }

    #[test]
    fn test_eval_b() {
        assert_eq!(eval("1 + (2 * 3) + (4 * (5 + 6))", 0).1, 51)
    }

    #[test]
    fn test_eval_c() {
        assert_eq!(eval("2 * 3 + (4 * 5)", 0).1, 26)
    }

    #[test]
    fn test_eval_d() {
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)", 0).1, 437)
    }

    #[test]
    fn test_eval_e() {
        assert_eq!(
            eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 0).1,
            12240
        )
    }

    #[test]
    fn test_eval_f() {
        assert_eq!(
            eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 0).1,
            13632
        )
    }

    #[test]
    fn test_eval2_a() {
        assert_eq!(eval2("1 + (2 * 3) + (4 * (5 + 6))", 0, false, 0).1, 51)
    }

    #[test]
    fn test_eval2_b() {
        assert_eq!(eval2("2 * 3 + (4 * 5)", 0, false, 0).1, 46)
    }

    #[test]
    fn test_eval2_c() {
        assert_eq!(eval2("5 + (8 * 3 + 9 + 3 * 4 * 3)", 0, false, 0).1, 1445)
    }

    #[test]
    fn test_eval2_d() {
        assert_eq!(
            eval2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 0, false, 0).1,
            669060
        )
    }

    #[test]
    fn test_eval2_e() {
        assert_eq!(
            eval2(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
                0,
                false,
                0
            )
            .1,
            23340
        )
    }
}
