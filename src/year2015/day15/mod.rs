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

fn part1() -> i32 {
    let ingredients = parse();

    let _ingredients_lookup: HashMap<String, Ingredient> = parse()
        .into_iter()
        .map(|ingredient| (ingredient.name.clone(), ingredient))
        .collect();

    let _ingredient_names = (&ingredients)
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
    let _iterations = (0..(number_of_ingredients * 100)).collect_vec();

    let mut _counter = 0;
    let mut best_sum = 0;

    for recipe in (0..400)
        .permutations(ingredients.len())
        .unique()
        .map(|recipe| {
            recipe
                .into_iter()
                .map(|ingredient| ingredient % 100)
                .collect_vec()
        })
        .filter(|recipe| {
            return recipe.into_iter().map(|a| *a).reduce(|a, b| a + b).unwrap() == 100;
        })
        .into_iter()
    {
        let my_recipe = (&recipe)
            .into_iter()
            .enumerate()
            .map(|(ingredient_index, spoons)| {
                (&ingredients)
                    .get(ingredient_index)
                    .unwrap()
                    .multiply(&spoons)
            })
            .collect_vec()
            .into_iter()
            .reduce(|a, b| a.add(&b))
            .unwrap();
        let my_recipe_sum = my_recipe.sum();
        println!("{:?}", recipe);
        println!("{:?}", my_recipe);
        println!("{:?}", my_recipe_sum);
        best_sum = std::cmp::max(best_sum, my_recipe_sum);

        println!("Currently best: {:?}", best_sum);
    }

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
