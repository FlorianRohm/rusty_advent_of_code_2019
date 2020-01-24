#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

fn main() {
    let filename = "./day3/resources/input";
    let string = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let code: Vec<&str> = string.lines().collect();
    let distance_manhattan = instructions_string_to_distance_manhattan(code[0], code[1]);

    println!("smallest manhattan distance was {}", distance_manhattan);

    let distance_steps = instructions_string_to_distance_steps(code[0], code[1]);

    println!("smallest step distance was {}", distance_steps);
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
enum Errors {
    ParseError(String),
}

#[derive(Debug, PartialEq)]
struct LineInstruction {
    direction: Direction,
    length: usize,
}

impl TryFrom<&str> for LineInstruction {
    type Error = Errors;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        use Errors::ParseError;
        let (dir_str, len_str) = input.split_at(1);

        let direction = match dir_str {
            "R" => Direction::Right,
            "D" => Direction::Down,
            "U" => Direction::Up,
            "L" => Direction::Left,
            _ => return Err(ParseError("no valid direction".to_string())),
        };

        let length = if let Ok(length) = usize::from_str(len_str) {
            length
        } else {
            return Err(ParseError("no valid number".to_string()));
        };

        Ok(LineInstruction { direction, length })
    }
}

#[derive(Default, Eq, Clone, Debug)]
struct LineStep {
    x: i32,
    y: i32,
    step_nr: usize,
}

impl Hash for LineStep {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for LineStep {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl LineStep {
    fn manhattan(&self) -> i32 {
        i32::abs(self.x) + i32::abs(self.y)
    }
}

#[derive(Debug, PartialEq)]
struct Coordinates(HashSet<LineStep>);

impl From<Vec<LineInstruction>> for Coordinates {
    fn from(values: Vec<LineInstruction>) -> Self {
        use Direction::*;
        let mut current_point = LineStep::default();
        let mut coordinates = HashSet::new();

        for value in values {
            for _ in 0..value.length {
                match value.direction {
                    Up => current_point.y += 1,
                    Down => current_point.y -= 1,
                    Left => current_point.x -= 1,
                    Right => current_point.x += 1,
                }
                current_point.step_nr += 1;
                coordinates.insert(current_point.clone());
            }
        }

        Coordinates(coordinates)
    }
}

fn distance_manhattan(coordinates1: Coordinates, coordinates2: Coordinates) -> i32 {
    coordinates1
        .0
        .intersection(&coordinates2.0)
        .map(|point| point.manhattan())
        .min()
        .expect("no common points found")
}

fn distance_steps(coordinates1: Coordinates, coordinates2: Coordinates) -> usize {
    coordinates1
        .0
        .iter()
        .filter_map(|coordinate1| {
            coordinates2
                .0
                .get(coordinate1)
                .map(|coordinate_2| coordinate1.step_nr + coordinate_2.step_nr)
        })
        .min()
        .expect("no intersection found")
}

fn instruction_string_to_instructions(instructions: &str) -> Vec<LineInstruction> {
    instructions
        .split(',')
        .map(|line| LineInstruction::try_from(line).expect("parsing failed"))
        .collect()
}

fn instructions_string_to_distance_manhattan(line1: &str, line2: &str) -> i32 {
    distance_manhattan(
        Coordinates::from(instruction_string_to_instructions(line1)),
        Coordinates::from(instruction_string_to_instructions(line2)),
    )
}

fn instructions_string_to_distance_steps(line1: &str, line2: &str) -> usize {
    distance_steps(
        Coordinates::from(instruction_string_to_instructions(line1)),
        Coordinates::from(instruction_string_to_instructions(line2)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_parsing() {
        assert_eq!(
            LineInstruction::try_from("D12"),
            Ok(LineInstruction {
                direction: Direction::Down,
                length: 12,
            })
        );
        assert_eq!(
            LineInstruction::try_from("U1234"),
            Ok(LineInstruction {
                direction: Direction::Up,
                length: 1234,
            })
        );
        assert_eq!(
            LineInstruction::try_from("L5654"),
            Ok(LineInstruction {
                direction: Direction::Left,
                length: 5654,
            })
        );
        assert_eq!(
            LineInstruction::try_from("R1"),
            Ok(LineInstruction {
                direction: Direction::Right,
                length: 1,
            })
        );
    }

    #[test]
    fn test_lines() {
        /* R8,U5,L5,D3
        ...........
        ....+----+.
        ....|....|.
        ....|....|.
        ....|....|.
        .........|.
        .o-------+.
        ...........
        */
        let line_instructions = vec![
            LineInstruction {
                direction: Direction::Right,
                length: 8,
            },
            LineInstruction {
                direction: Direction::Up,
                length: 5,
            },
            LineInstruction {
                direction: Direction::Left,
                length: 5,
            },
            LineInstruction {
                direction: Direction::Down,
                length: 3,
            },
        ];

        let points: Coordinates = Coordinates(HashSet::from_iter(vec![
            LineStep {
                x: 1,
                y: 0,
                step_nr: 1,
            },
            LineStep {
                x: 2,
                y: 0,
                step_nr: 2,
            },
            LineStep {
                x: 3,
                y: 0,
                step_nr: 3,
            },
            LineStep {
                x: 4,
                y: 0,
                step_nr: 4,
            },
            LineStep {
                x: 5,
                y: 0,
                step_nr: 5,
            },
            LineStep {
                x: 6,
                y: 0,
                step_nr: 6,
            },
            LineStep {
                x: 7,
                y: 0,
                step_nr: 7,
            },
            LineStep {
                x: 8,
                y: 0,
                step_nr: 8,
            },
            LineStep {
                x: 8,
                y: 1,
                step_nr: 9,
            },
            LineStep {
                x: 8,
                y: 2,
                step_nr: 10,
            },
            LineStep {
                x: 8,
                y: 3,
                step_nr: 11,
            },
            LineStep {
                x: 8,
                y: 4,
                step_nr: 12,
            },
            LineStep {
                x: 8,
                y: 5,
                step_nr: 13,
            },
            LineStep {
                x: 7,
                y: 5,
                step_nr: 14,
            },
            LineStep {
                x: 6,
                y: 5,
                step_nr: 15,
            },
            LineStep {
                x: 5,
                y: 5,
                step_nr: 16,
            },
            LineStep {
                x: 4,
                y: 5,
                step_nr: 17,
            },
            LineStep {
                x: 3,
                y: 5,
                step_nr: 18,
            },
            LineStep {
                x: 3,
                y: 4,
                step_nr: 19,
            },
            LineStep {
                x: 3,
                y: 3,
                step_nr: 20,
            },
            LineStep {
                x: 3,
                y: 2,
                step_nr: 21,
            },
        ]));
        let coordinates = Coordinates::from(line_instructions);
        let mut steps: Vec<usize> = coordinates
            .0
            .iter()
            .map(|line_step| line_step.step_nr)
            .collect();
        steps.sort();

        assert_eq!(coordinates, points);
        assert!(steps.into_iter().eq(1..=21));
    }

    #[test]
    fn test_online() {
        let str1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let str2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        let distance = 159;

        assert_eq!(
            instructions_string_to_distance_manhattan(str1, str2),
            distance
        );
    }

    #[test]
    fn test_online_2() {
        let str1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let str2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let distance = 135;

        assert_eq!(
            instructions_string_to_distance_manhattan(str1, str2),
            distance
        );
    }

    #[test]
    fn test_online_steps() {
        let str1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let str2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        let distance = 610;

        assert_eq!(instructions_string_to_distance_steps(str1, str2), distance);
    }

    #[test]
    fn test_online_steps_2() {
        let str1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let str2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let distance = 410;

        assert_eq!(instructions_string_to_distance_steps(str1, str2), distance);
    }
}
