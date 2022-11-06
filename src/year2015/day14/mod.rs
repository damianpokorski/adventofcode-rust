use std::fs;

use itertools::Itertools;

const PATH: &str = "src/year2015/day14/data.raw";

fn read_file() -> String {
    println!("Reading a file: {PATH}");
    return fs::read_to_string(PATH).expect("Should be able to read the file");
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    flight_duration: u32,
    rest_duration: u32,
}

impl Reindeer {
    fn fly_for_duration(&self, duration: i32) -> u32 {
        let mut fuel = self.flight_duration.clone();
        let mut rest_required = 0;
        let mut time_spent = 0;
        let mut distance_travelled = 0;

        while duration > time_spent {
            time_spent = time_spent + 1;

            if fuel > 0 {
                distance_travelled = distance_travelled + self.speed;
                fuel = fuel - 1;
                if fuel == 0 {
                    rest_required = self.rest_duration.clone();
                }
            } else if rest_required > 0 {
                rest_required = rest_required - 1;
                // Restore fuel after rest
                if rest_required == 0 {
                    fuel = self.flight_duration.clone();
                }
            }
        }

        return distance_travelled;
    }
}
fn get_reindeer() -> Vec<Reindeer> {
    return read_file()
        .split("\n")
        .map(|string| string.replace(" can fly", ""))
        .map(|string| string.replace(" km/s for", ""))
        .map(|string| string.replace("seconds, but then must rest for ", ""))
        .map(|string| string.replace(" seconds.", ""))
        .map(|string| {
            (&string)
                .split(" ")
                .map(|chunk| chunk.to_string())
                .into_iter()
                .collect_vec()
                .clone()
        })
        .map(|pieces| Reindeer {
            name: pieces.get(0).unwrap().to_string(),
            speed: pieces.get(1).unwrap().parse::<u32>().unwrap(),
            flight_duration: pieces.get(2).unwrap().parse::<u32>().unwrap(),
            rest_duration: pieces.get(3).unwrap().parse::<u32>().unwrap(),
        })
        .collect_vec();
}

fn part1() -> u32 {
    return get_reindeer()
        .into_iter()
        .map(|reindeer| (&reindeer).fly_for_duration(2503))
        .reduce(|a, b| std::cmp::max(a, b))
        .unwrap();
}

fn part2() -> i32 {
    return -1;
}

pub fn puzzle() {
    let comet = Reindeer {
        name: "Comet".to_string(),
        speed: 14,
        flight_duration: 10,
        rest_duration: 127,
    };
    let dancer = Reindeer {
        name: "Dancer".to_string(),
        speed: 16,
        flight_duration: 11,
        rest_duration: 162,
    };
    println!("Comets test flight {:?}", comet.fly_for_duration(1000));
    println!("Dancers test flight {:?}", dancer.fly_for_duration(1000));

    let result = part1();
    println!("Part1: {:?}", result);
    let result = part2();
    println!("Part2: {:?}", result);
}
