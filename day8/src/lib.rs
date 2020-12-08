use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(input: &str) -> Result<Operation, Self::Err> {
        match input {
            "acc" => Ok(Operation::Acc),
            "jmp" => Ok(Operation::Jmp),
            "nop" => Ok(Operation::Nop),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

#[derive(Debug)]
struct Interpreter {
    instructions: Vec<Instruction>,
    pointer: usize,
    accumulator: i32,
}

impl Interpreter {
    fn new(input: &Vec<&str>) -> Self {
        let instructions = input
            .iter()
            .map(|line| {
                let parts = line.split(' ').collect::<Vec<&str>>();
                Instruction {
                    operation: Operation::from_str(parts.get(0).unwrap()).unwrap(),
                    argument: parts.get(1).unwrap().parse::<i32>().unwrap(),
                }
            })
            .collect::<Vec<Instruction>>();

        Interpreter {
            instructions,
            pointer: 0,
            accumulator: 0,
        }
    }

    fn reset(&mut self) {
        self.accumulator = 0;
        self.pointer = 0;
    }
}

impl Iterator for Interpreter {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions.get(self.pointer)?;

        match instruction.operation {
            Operation::Acc => {
                self.accumulator += instruction.argument;
                self.pointer += 1;
            }
            Operation::Jmp => {
                self.pointer = (self.pointer as i32 + instruction.argument) as usize;
            }
            Operation::Nop => {
                self.pointer += 1;
            }
        }

        Some(())
    }
}

pub fn part1(input: &Vec<&str>) -> i32 {
    let mut interpreter = Interpreter::new(input);
    let mut visited: HashSet<usize> = HashSet::new();

    while let Some(_) = interpreter.next() {
        if visited.contains(&interpreter.pointer) {
            break;
        }
        visited.insert(interpreter.pointer.clone());
    }

    interpreter.accumulator
}

pub fn part2(input: &Vec<&str>) -> i32 {
    fn swap_jmp_and_nop(operation: &Operation) -> Operation {
        if *operation == Operation::Jmp {
            Operation::Nop
        } else {
            Operation::Jmp
        }
    }

    let mut interpreter = Interpreter::new(input);
    let mut replace_index = None;

    'outer: loop {
        interpreter.reset();

        // Revert previous replace
        if replace_index.is_some() {
            let instruction: &mut Instruction = interpreter
                .instructions
                .get_mut(replace_index.unwrap())
                .unwrap();

            instruction.operation = swap_jmp_and_nop(&instruction.operation);
        }

        // Replace next
        for (index, instruction) in interpreter.instructions.iter_mut().enumerate() {
            if (replace_index.is_some() && index <= replace_index.unwrap())
                || instruction.operation == Operation::Acc
            {
                continue;
            }

            replace_index = Some(index);
            instruction.operation = swap_jmp_and_nop(&instruction.operation);

            break;
        }

        // Run interpreter
        let mut visited: HashSet<usize> = HashSet::new();
        while let Some(_) = interpreter.next() {
            if visited.contains(&interpreter.pointer) {
                continue 'outer;
            }
            visited.insert(interpreter.pointer.clone());
        }

        return interpreter.accumulator;
    }
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 8);
    }
}
