use crate::common::puzzle_data;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
    spoonfulls: i32,
}

impl Ingredient {
    fn add(&self, ingredient: &Ingredient) -> Ingredient {
        let mut new_name = self.name.clone();
        new_name.push_str(" / ");
        new_name.push_str(ingredient.name.clone().as_str());
        return Ingredient {
            name: new_name,
            capacity: self.capacity + ingredient.capacity,
            durability: self.durability + ingredient.durability,
            flavor: self.flavor + ingredient.flavor,
            texture: self.texture + ingredient.texture,
            calories: self.calories + ingredient.calories,
            spoonfulls: self.spoonfulls + ingredient.spoonfulls,
        };
    }
    fn multiply(&self, multiplier: &i32) -> Ingredient {
        let mut new_name = self.name.clone();
        new_name.push_str(" x ");
        new_name.push_str(multiplier.clone().to_string().as_str());
        return Ingredient {
            name: new_name,
            capacity: self.capacity * multiplier,
            durability: self.durability * multiplier,
            flavor: self.flavor * multiplier,
            texture: self.texture * multiplier,
            calories: self.calories * multiplier,
            spoonfulls: self.spoonfulls * multiplier,
        };
    }
    fn sum(&self) -> i32 {
        return std::cmp::max(self.capacity, 0)
            * std::cmp::max(self.durability, 0)
            * std::cmp::max(self.flavor, 0)
            * std::cmp::max(self.texture, 0);
    }
}

fn parse() -> Vec<Ingredient> {
    return puzzle_data(std::file!())
        .split("\n")
        .into_iter()
        .map(|string| string.replace("capacity ", ""))
        .map(|string| string.replace("durability ", ""))
        .map(|string| string.replace("flavor ", ""))
        .map(|string| string.replace("texture ", ""))
        .map(|string| string.replace("calories ", ""))
        .map(|string| string.replace(":", ","))
        .map(|string| string.replace(", ", ","))
        .map(|string| {
            let pieces = string
                .split(",")
                .map(|sub| sub.to_string())
                .into_iter()
                .collect_vec();

            let name = pieces.first().unwrap();
            let values = (&pieces)
                .into_iter()
                .enumerate()
                .filter(|enumerate| enumerate.0 != 0)
                .map(|enumerate| enumerate.1)
                .map(|value| value.parse::<i32>().unwrap())
                .into_iter()
                .collect_vec();

            return Ingredient {
                name: name.clone(),
                capacity: values.get(0).unwrap().clone(),
                durability: values.get(1).unwrap().clone(),
                flavor: values.get(2).unwrap().clone(),
                texture: values.get(3).unwrap().clone(),
                calories: values.get(4).unwrap().clone(),
                spoonfulls: 1,
            };
        })
        .collect_vec();
}

fn example() {
    let butterscrotch = Ingredient {
        name: "Butterscotch".to_string(),
        capacity: -1,
        durability: -2,
        flavor: 6,
        texture: 3,
        calories: 8,
        spoonfulls: 1,
    };
    let cinnamon = Ingredient {
        name: "Cinnamon".to_string(),
        capacity: 2,
        durability: 3,
        flavor: -2,
        texture: -1,
        calories: 3,
        spoonfulls: 1,
    };

    let mut buffer = butterscrotch.clone();

    for _index in 0..43 {
        buffer = buffer.add(&butterscrotch);
    }
    for _index in 0..56 {
        buffer = buffer.add(&cinnamon);
    }
    println!("44 Butterscotch, 56 cinnamon: {:?}", buffer.sum());
}

fn spoonfull_combinations(
    max_spoonfulls: i32,
    previous_combinations: Vec<i32>,
    insert_target: &mut Vec<Vec<i32>>,
    depth: usize,
    running_sum: i32,
) {
    if depth == 0 {
        if running_sum == max_spoonfulls {
            insert_target.push(previous_combinations);
        }
        return;
    }

    for spoons in 0..max_spoonfulls {
        if running_sum + spoons <= max_spoonfulls {
            let mut combination = previous_combinations.clone();
            combination.push(spoons);

            spoonfull_combinations(
                max_spoonfulls,
                combination,
                insert_target,
                depth - 1,
                running_sum + spoons,
            );
        }
    }
}

fn part1() -> i32 {
    let ingredients = parse();

    let mut best = 0;

    let mut target: Vec<Vec<i32>> = vec![];
    spoonfull_combinations(100, vec![], &mut target, (&ingredients).len() + 0, 0);

    for entry in target.into_iter() {
        // Build recipe
        let mut mix = Ingredient {
            spoonfulls: 0,
            name: "".to_string(),
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        };

        // For each ingredient
        for ingredient_index in 0..ingredients.len() {
            mix = mix.add(
                &ingredients
                    .get(ingredient_index)
                    .unwrap()
                    .multiply(entry.get(ingredient_index).unwrap()),
            );
        }

        if mix.sum() > best {
            best = mix.sum()
        }
    }

    // Test
    return best;
}

fn part2() -> i32 {
    let ingredients = parse();

    let mut best = 0;

    let mut target: Vec<Vec<i32>> = vec![];
    spoonfull_combinations(100, vec![], &mut target, (&ingredients).len() + 0, 0);

    for entry in target.into_iter() {
        // Build recipe
        let mut mix = Ingredient {
            spoonfulls: 0,
            name: "".to_string(),
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        };

        // For each ingredient
        for ingredient_index in 0..ingredients.len() {
            mix = mix.add(
                &ingredients
                    .get(ingredient_index)
                    .unwrap()
                    .multiply(entry.get(ingredient_index).unwrap()),
            );
        }

        if mix.calories == 500 && mix.sum() > best {
            best = mix.sum()
        }
    }

    return best;
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    example();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
