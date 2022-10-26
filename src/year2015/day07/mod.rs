use std::{fs, num::ParseIntError};

const PATH: &str = "src/year2015/day07/data.raw";

#[derive(Debug, Clone, Copy)]
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
    fn evalate(&self) -> u16;
}

#[derive(Debug, Clone)]
struct Wire {
    op: Gate,

    left_value: Option<u16>,
    left_reference: Option<String>,

    right_value: Option<u16>,
    right_reference: Option<String>,

    target: String,
}

impl Evaluable for Wire {
    fn evaluable(&self) -> bool {
        self.left_reference.is_none() && self.right_reference.is_none()
    }

    fn evalate(&self) -> u16 {
        if self.evaluable() {
            match self.op {
                Gate::NONE => return self.left_value.unwrap(),
                Gate::NOT => return !self.left_value.unwrap(),
                Gate::AND => return self.left_value.unwrap() & self.right_value.unwrap(),
                Gate::OR => return self.left_value.unwrap() | self.right_value.unwrap(),
                Gate::XOR => return self.left_value.unwrap() ^ self.right_value.unwrap(),
                Gate::LSHIFT => return self.left_value.unwrap() << self.right_value.unwrap(),
                Gate::RSHIFT => return self.left_value.unwrap() >> self.right_value.unwrap(),
            }
        }
        return 0;
    }
}

fn read_file() -> String {
    println!("Reading a file: {PATH}");
    return fs::read_to_string(PATH).expect("Should be able to read the file");
}
fn segment_to_value(segment: &str) -> (Option<u16>, Option<Gate>, Option<String>) {
    match segment {
        "NOT" => return (None, Some(Gate::NOT), None),
        "AND" => return (None, Some(Gate::AND), None),
        "OR" => return (None, Some(Gate::OR), None),
        "XOR" => return (None, Some(Gate::XOR), None),
        "LSHIFT" => return (None, Some(Gate::LSHIFT), None),
        "RSHIFT" => return (None, Some(Gate::RSHIFT), None),
        _ => {
            // Parse as number
            let number: Result<u16, ParseIntError> = segment.parse::<u16>();
            if number.is_ok() {
                return (number.ok(), None, None);
            }
            return (None, None, Some(segment.to_string()));
        }
    };
}

fn parse() -> Vec<Wire> {
    let mut wires: Vec<Wire> = Vec::new();

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
                if wire.left_value.is_none() && wire.left_reference.is_none() {
                    wire.left_value = value.to_owned();
                } else {
                    wire.right_value = value.to_owned();
                }
            } else if gate.is_some() {
                wire.op = gate.unwrap();
            } else if source.is_some() {
                if wire.left_value.is_none() && wire.left_reference.is_none() {
                    wire.left_reference = source;
                } else {
                    wire.right_reference = source;
                }
            }
        }
        println!("{:?}", line);
        println!("{:?}", wire);
        wires.push(wire);
    }

    println!("---------");
    return wires;
}

fn process(wires: Vec<Wire>) -> Vec<Wire> {
    // Go through all wires
    let wires_length = wires.len();
    let mut wires = wires;

    loop {
        // Clone the vector for refence look up
        let other_wires = wires.clone();

        let mut finished = true;
        for wire_index in 0..wires_length {
            let mut current_wire = &mut wires[wire_index];

            println!("{:?}", current_wire.target);
            println!("{:?}", current_wire.evaluable());
            println!("{:?}", current_wire.evalate());
            println!("===");

            // If wire is not evaluatable - check if the source wires are
            if current_wire.evaluable() == false {
                finished = false;
                // Iteratate through evaluatable wires
                for other_wire_index in 0..wires_length {
                    // Extract other wire reference
                    let other_wire = &other_wires[other_wire_index];

                    if other_wire.evaluable() {
                        // If the left value is none
                        if current_wire.left_reference != None
                            && current_wire.left_reference.as_ref().unwrap() == &other_wire.target
                        {
                            let result = other_wire.evalate();
                            current_wire.left_value = Some(result.clone());
                            current_wire.left_reference = None;
                            println!("Left {:?} Match on ", current_wire)
                        }
                        // If the right value is none
                        if current_wire.right_reference != None
                            && current_wire.right_reference.as_ref().unwrap() == &other_wire.target
                        {
                            let result = other_wire.evalate();
                            current_wire.right_value = Some(result.clone());
                            current_wire.right_reference = None;
                            println!("right {:?} Match on ", current_wire)
                        }
                    }
                }
            }
        }
        if finished {
            println!("We are now completely done!");

            for wire_index in 0..wires_length {
                println!(
                    "{:?} {:?}",
                    wires[wire_index].target,
                    wires[wire_index].evalate()
                );
            }
            break;
        } else {
            println!("There's stuff to do");
        }
    }

    return wires;
}

fn part1() -> u16 {
    let wires = process(parse());

    let target = "a".to_string();
    for wire in wires {
        if wire.target == target {
            println!("Got answer!");
            println!("{:?}", wire.evalate());
            return wire.evalate();
        }
    }

    println!("Couldnt find answer");
    return 0;
}

fn part2() -> u16 {
    // part 1

    let wires = process(parse());
    let mut part1 = 0;
    let a = "a".to_string();
    for wire in wires {
        if wire.target == a {
            println!("Got answer!");
            println!("{:?}", wire.evalate());
            part1 = wire.evalate();
        }
    }

    // Update target b to value of a
    let b = "b".to_string();
    let wires: Vec<Wire> = parse()
        .into_iter()
        .map(|wire| {
            if wire.target == b {
                return Wire {
                    op: Gate::NONE,
                    left_value: Some(part1),
                    left_reference: None,
                    right_value: None,
                    right_reference: None,
                    target: b.clone(),
                };
            } else {
                return wire;
            }
        })
        .collect();

    return process(wires)
        .into_iter()
        .find(|wire| wire.target == a)
        .unwrap()
        .evalate();
}

pub fn puzzle() {
    let result = part1();
    println!("Part1: {:?}", result);
    let result = part2();
    println!("Part2: {:?}", result);
}
