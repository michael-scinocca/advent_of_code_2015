use std::fs::read_to_string;

struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavour: i32,
    texture: i32,
    _calories: i32,
}

#[derive(Clone)]
struct Recipe<'a> {
    ingredients: Vec<(u32, &'a Ingredient)>,
}

impl<'a> Recipe<'a> {
    fn new() -> Recipe<'a> {
        Recipe {
            ingredients: Vec::new(),
        }
    }

    fn add_ingredient(&mut self, quantity: u32, ingredient: &'a Ingredient) {
        let existing = self
            .ingredients
            .iter_mut()
            .find(|i| i.1.name == ingredient.name);

        if let Some(existing) = existing {
            existing.0 = quantity;
        } else {
            self.ingredients.push((quantity, ingredient));
        }
    }

    fn get_score(&self) -> i32 {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavour = 0;
        let mut texture = 0;

        for ingredient in self.ingredients.iter() {
            capacity += ingredient.1.capacity * ingredient.0 as i32;
            durability += ingredient.1.durability * ingredient.0 as i32;
            flavour += ingredient.1.flavour * ingredient.0 as i32;
            texture += ingredient.1.texture * ingredient.0 as i32;
        }

        if capacity <= 0 || durability <= 0 || flavour <= 0 || texture <= 0 {
            return 0;
        }

        capacity * durability * flavour * texture
    }
}

fn get_ingredients() -> Vec<Ingredient> {
    let mut ingredients = Vec::new();

    for line in read_to_string("data/day15.txt").unwrap().lines() {
        let mut words = line.split_whitespace();

        let name = words.next().unwrap().trim_end_matches(":").to_string();
        words.next();

        let capacity = words.next().unwrap().trim_end_matches(",").parse::<i32>().unwrap();
        words.next();

        let durability = words.next().unwrap().trim_end_matches(",").parse::<i32>().unwrap();
        words.next();

        let flavour = words.next().unwrap().trim_end_matches(",").parse::<i32>().unwrap();
        words.next();

        let texture = words.next().unwrap().trim_end_matches(",").parse::<i32>().unwrap();
        words.next();

        let calories = words.next().unwrap().trim_end_matches(",").parse::<i32>().unwrap();
        words.next();

        ingredients.push(Ingredient {
            name,
            capacity,
            durability,
            flavour,
            texture,
            _calories: calories,
        })
    }

    ingredients
}

fn find_optimum<'a>(max_quantity: u32, ingredients: &'a [Ingredient]) -> (i32, Recipe<'a>) {
    let mut max_score = 0;
    let mut max_recipe = Recipe::new();

    let mut recipe = Recipe::new();

    find_optimum_working_recipe(
        max_quantity,
        ingredients,
        &mut recipe,
        &mut max_score,
        &mut max_recipe,
    );

    (max_score, max_recipe)
}

fn find_optimum_working_recipe<'a>(
    max_quantity: u32,
    ingredients: &'a [Ingredient],
    working_recipe: &mut Recipe<'a>,
    max_score: &mut i32,
    max_recipe: &mut Recipe<'a>,
) {
    let is_last = ingredients.len() == 1;

    let min_quantity = { if is_last { max_quantity } else { 0 } };

    for i in min_quantity..=max_quantity {
        let ingredient = &ingredients[0];

        working_recipe.add_ingredient(i, ingredient);

        if ingredients.len() > 1 {
            find_optimum_working_recipe(
                max_quantity - i,
                &ingredients[1..],
                working_recipe,
                max_score,
                max_recipe,
            );
        } else {
            let score = working_recipe.get_score();

            if score > *max_score {
                *max_score = score;
                *max_recipe = working_recipe.clone();
            }
        }
    }
}

pub fn part1() {
    let ingredients = get_ingredients();

    let (score, recipe) = find_optimum(100, &ingredients);

    println!("{}", score);

    for ingredient in recipe.ingredients {
        println!("{} {}", ingredient.0, ingredient.1.name);
    }
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_ingredients() -> Vec<Ingredient> {
        vec![
            Ingredient {
                name: "butterscotch".to_string(),
                capacity: -1,
                durability: -2,
                flavour: 6,
                texture: 3,
                _calories: 8,
            },
            Ingredient {
                name: "cinnamon".to_string(),
                capacity: 2,
                durability: 3,
                flavour: -2,
                texture: -1,
                _calories: 3,
            },
        ]
    }

    #[test]
    fn test_ingredients_score() {
        let ingredients = get_test_ingredients();

        let mut recipe = Recipe::new();

        recipe.add_ingredient(
            44,
            ingredients
                .iter()
                .find(|s| s.name == "butterscotch")
                .unwrap(),
        );
        recipe.add_ingredient(
            56,
            ingredients.iter().find(|s| s.name == "cinnamon").unwrap(),
        );

        assert_eq!(recipe.get_score(), 62842880);
    }

    #[test]
    fn test_optimal_ingredients() {
        let ingredients = get_test_ingredients();

        let score = find_optimum(100, &ingredients);

        assert_eq!(score.0, 62842880);
    }
}
