use std::{fs, num::ParseIntError};

const PATH: &str = "src/year2015/day07/data.raw";

#[derive(Debug)]
enum Gate {
    NONE,
    NOT,
    AND,
    OR,
    XOR,
    LSHIFT,
    RSHIFT,
}

trait Evaluable {
  fn evaluable(&self) -> bool;
  fn evalate(&self) -> i32;
}

impl Evaluable for Wire {
    fn evaluable(&self) -> bool {
        self.left_reference.is_none() && self.right_reference.is_none()
    }

    fn evalate(&self) -> i32 {
        todo!()
        
    }

    // fn evalate(&self) -> i32 {
    //   self.right_reference;
    //     if self.evaluable() {
    //       match self.op {
    //         Gate::NONE => return -1,
    //         Gate::NOT => return -1,
    //         Gate::AND => todo!(),
    //         Gate::OR => todo!(),
    //         Gate::XOR => todo!(),
    //         Gate::LSHIFT => todo!(),
    //         Gate::RSHIFT => todo!(),
    //     }
    //     return -1;
    // }
    
}


#[derive(Debug)]
struct Wire {
    op: Gate,

    left_value: Option<i32>,
    left_reference: Option<String>,

    right_value: Option<i32>,
    right_reference: Option<String>,

    target: String,
}

fn read_file() -> String {
    println!("Reading a file: {PATH}");
    return fs::read_to_string(PATH).expect("Should be able to read the file");
}
fn segment_to_value(segment: &str) -> (Option<i32>, Option<Gate>, Option<String>) {
    match segment {
        "NOT" => return (None, Some(Gate::NOT), None),
        "AND" => return (None, Some(Gate::AND), None),
        "OR" => return (None, Some(Gate::OR), None),
        "XOR" => return (None, Some(Gate::XOR), None),
        "LSHIFT" => return (None, Some(Gate::LSHIFT), None),
        "RSHIFT" => return (None, Some(Gate::RSHIFT), None),
        _ => {
            // Parse as number
            let number: Result<i32, ParseIntError> = segment.parse::<i32>();
            if number.is_ok() {
                return (number.ok(), None, None);
            }
            return (None, None, Some(segment.to_string()));
        }
    };
}

fn parse() -> Vec<Wire> {
    let wires: Vec<Wire> = Vec::new();

    // Process input
    for line in read_file().split("\n").into_iter() {
        let target = line.split(" -> ").last().unwrap();
        // Remove target and arrow
        let left_side = line.replace(" -> ", "").replace(target, "");
        let left_side_segments: Vec<&str> = left_side.split(" ").into_iter().collect();

        // Create wire struct
        let mut wire = Wire {
            op: Gate::NONE,
            left_value: None,
            left_reference: None,
            right_value: None,
            right_reference: None,
            target: target.to_string(),
        };

        // Parse segments
        for segment in left_side_segments {
            let (value, gate, source) = segment_to_value(segment);
            // Operator
            if value.is_some() {
                if wire.left_value.is_none() {
                    wire.left_value = value.to_owned();
                } else {
                    wire.right_value = value.to_owned();
                }
            } else if gate.is_some() {
                wire.op = gate.unwrap();
            } else if source.is_some() {
                if wire.left_reference.is_none() {
                    wire.left_reference = source;
                } else {
                    wire.right_reference = source;
                }
            }
        }
    }
    return wires;
}

fn part1() -> (i32) {
    let wires = parse();
    
    for wire in wires {

    }

    return -1;
}

fn part2() -> (i32) {
    return -1;
}

pub fn puzzle() {
    let result = part1();
    println!("Part1: {:?}", result);
    let result = part2();
    println!("Part2: {:?}", result);
}
