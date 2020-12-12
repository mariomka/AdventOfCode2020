#[macro_use]
extern crate lazy_static;

use std::str::FromStr;

use regex::Regex;

#[derive(PartialEq, Clone, Debug)]
enum CardinalPoint {
    North,
    South,
    East,
    West,
}

impl CardinalPoint {
    fn ordered_keys() -> [CardinalPoint; 4] {
        [Self::North, Self::East, Self::South, Self::West]
    }

    fn rotate(&self, steps: usize) -> Self {
        let current_index = Self::ordered_keys()
            .iter()
            .position(|cardinal_point| cardinal_point == self)
            .unwrap();

        let next_index = (current_index + steps) % 4;

        Self::ordered_keys()[next_index].clone()
    }
}

#[derive(Debug)]
enum ActionType {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
    RotateLeft,
    RotateRight,
    MoveForward,
}

impl FromStr for ActionType {
    type Err = ();

    fn from_str(input: &str) -> Result<ActionType, Self::Err> {
        match input {
            "N" => Ok(ActionType::MoveNorth),
            "S" => Ok(ActionType::MoveSouth),
            "E" => Ok(ActionType::MoveEast),
            "W" => Ok(ActionType::MoveWest),
            "L" => Ok(ActionType::RotateLeft),
            "R" => Ok(ActionType::RotateRight),
            "F" => Ok(ActionType::MoveForward),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Action {
    action_type: ActionType,
    value: i32,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(input: &str) -> Result<Action, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"^(?P<type>[NSEWLRF])(?P<value>\d{1,3})$").unwrap();
        }

        let captures = REGEX.captures(input).unwrap();

        Ok(Action {
            action_type: captures.name("type").unwrap().as_str().parse().unwrap(),
            value: captures.name("value").unwrap().as_str().parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct ShipPosition {
    x: i32,
    y: i32,
    face: CardinalPoint,
}

impl ShipPosition {
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn rotate(&mut self, steps: i8) {
        let steps = if steps > 0 { steps } else { 4 + steps } as usize;
        self.face = self.face.rotate(steps);
    }
}

#[derive(Debug)]
struct WaypointPosition {
    x: i32,
    y: i32,
}

impl WaypointPosition {
    fn rotate(&mut self, steps: i32) {
        for _ in 0..steps.abs() {
            let new_x = if steps > 0 { 1 } else { -1 } * self.y;
            let new_y = if steps > 0 { -1 } else { 1 } * self.x;
            self.x = new_x;
            self.y = new_y;
        }
    }
}

pub fn part1(input: &Vec<&str>) -> i32 {
    let mut ship_position = ShipPosition {
        x: 0,
        y: 0,
        face: CardinalPoint::East,
    };

    input.iter().for_each(|line| {
        let action: Action = line.parse().unwrap();

        match action.action_type {
            ActionType::MoveNorth => {
                ship_position.y += action.value;
            }
            ActionType::MoveSouth => {
                ship_position.y -= action.value;
            }
            ActionType::MoveEast => {
                ship_position.x += action.value;
            }
            ActionType::MoveWest => {
                ship_position.x -= action.value;
            }
            ActionType::RotateLeft => match action.value {
                90 => ship_position.rotate(-1),
                180 => ship_position.rotate(-2),
                270 => ship_position.rotate(-3),
                _ => unreachable!(),
            },
            ActionType::RotateRight => match action.value {
                90 => ship_position.rotate(1),
                180 => ship_position.rotate(2),
                270 => ship_position.rotate(3),
                _ => unreachable!(),
            },
            ActionType::MoveForward => match ship_position.face {
                CardinalPoint::North => {
                    ship_position.y += action.value;
                }
                CardinalPoint::South => {
                    ship_position.y -= action.value;
                }
                CardinalPoint::East => {
                    ship_position.x += action.value;
                }
                CardinalPoint::West => {
                    ship_position.x -= action.value;
                }
            },
        }
    });

    ship_position.distance()
}

pub fn part2(input: &Vec<&str>) -> i32 {
    let mut ship_position = ShipPosition {
        x: 0,
        y: 0,
        face: CardinalPoint::East,
    };
    let mut waypoint_position = WaypointPosition { x: 10, y: 1 };

    input.iter().for_each(|line| {
        let action: Action = line.parse().unwrap();

        match action.action_type {
            ActionType::MoveNorth => {
                waypoint_position.y += action.value;
            }
            ActionType::MoveSouth => {
                waypoint_position.y -= action.value;
            }
            ActionType::MoveEast => {
                waypoint_position.x += action.value;
            }
            ActionType::MoveWest => {
                waypoint_position.x -= action.value;
            }
            ActionType::RotateLeft => match action.value {
                90 => waypoint_position.rotate(-1),
                180 => waypoint_position.rotate(-2),
                270 => waypoint_position.rotate(-3),
                _ => unreachable!(),
            },
            ActionType::RotateRight => match action.value {
                90 => waypoint_position.rotate(1),
                180 => waypoint_position.rotate(2),
                270 => waypoint_position.rotate(3),
                _ => unreachable!(),
            },
            ActionType::MoveForward => {
                ship_position.x += waypoint_position.x * action.value;
                ship_position.y += waypoint_position.y * action.value;
            }
        }
    });

    ship_position.distance()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
F10
N3
F7
R90
F11";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 25)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 286)
    }
}
