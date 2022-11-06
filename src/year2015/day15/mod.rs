use crate::common::puzzle_data;
use itertools::Itertools;
use std::collections::HashMap;

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
        return Ingredient {
            name: self.name.clone(),
            capacity: self.capacity + ingredient.capacity,
            durability: self.durability + ingredient.durability,
            flavor: self.flavor + ingredient.flavor,
            texture: self.texture + ingredient.texture,
            calories: self.calories + ingredient.calories,
            spoonfulls: self.spoonfulls + ingredient.spoonfulls,
        };
    }
    fn multiply(&self, multiplier: i32) -> Ingredient {
        return Ingredient {
            name: self.name.clone(),
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

    for index in 0..43 {
        buffer = buffer.add(&butterscrotch);
    }
    for index in 0..56 {
        buffer = buffer.add(&cinnamon);
    }
    println!("44 Butterscotch, 56 cinnamon: {:?}", buffer.sum());
}

fn part1() -> i32 {
    let ingredients = parse();

    let ingredients_lookup: HashMap<String, Ingredient> = parse()
        .into_iter()
        .map(|ingredient| (ingredient.name.clone(), ingredient))
        .collect();

    let ingredient_names = (&ingredients)
        .into_iter()
        .map(|ingredient| ingredient.name.clone())
        .collect_vec();

    // go through permutations of ingredients
    // let mut best_combination = Ingredient {
    //     name: "buffer".to_string(),
    //     capacity: 0,
    //     durability: 0,
    //     flavor: 0,
    //     texture: 0,
    //     calories: 0,
    //     spoonfulls: 1,
    // };

    // Generate permutations - multiply ingredients by 100, then cycle combinations, use modulus to cheat against permutations and not being able to have duplicates in it
    let number_of_ingredients = (&ingredients).len().clone();
    let iterations = (0..(number_of_ingredients * 100)).collect_vec();

    let mut counter = 0;

    // let mut max_ingredients: HashMap<String, Ingredient> = HashMap::new();

    // for ingredient in ingredients {
    //     let mut max_amount_of_spoons = ingredient.clone();
    //     let mut ingredient_pile = ingredient.clone();
    //     for spoonfulls in 1..100 {
    //         ingredient_pile = ingredient_pile.add(&ingredient);
    //         println!(
    //             "{:?}, {:?} = {:?}",
    //             spoonfulls,
    //             ingredient_pile,
    //             ingredient_pile.sum()
    //         );
    //         if max_amount_of_spoons.sum() < ingredient_pile.sum() {
    //             max_amount_of_spoons = ingredient_pile.clone();
    //         }
    //     }
    //     max_ingredients
    //         .entry(max_amount_of_spoons.name.clone())
    //         .or_insert(max_amount_of_spoons.clone());
    // }

    // println!("{:?}", max_ingredients);

    // Permutations too slow - need something faster here
    // let mut best_score = 0;

    // for combo in iterations.into_iter().permutations(100).into_iter() {
    //     // println!("{:?}", combo);
    //     let new_sum = combo
    //         .into_iter()
    //         .map(|index| (&ingredients).get(index % number_of_ingredients).unwrap())
    //         .reduce(|a, b| a.clone().add(b).clone())
    //         .unwrap()
    //         .sum();

    //     best_score = std::cmp::max(best_score, new_sum);
    //     counter = counter + 1;
    //     if counter % 1000 == 0 {
    //         println!("Cycles counted: {:?}", counter);
    //     }
    // }

    // Test
    return 0;
}

fn part2() -> i32 {
    return -1;
}

pub fn puzzle() {
    let part1 = part1();
    let part2 = part2();
    example();
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2);
}
