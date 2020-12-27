use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: Vec<&'a str>,
}

fn parse<'a>(input: &'a Vec<&'a str>) -> (Vec<Food>, HashSet<&str>, HashMap<&str, HashSet<&str>>) {
    let mut food_list = Vec::new();
    let mut ingredient_list = HashSet::new();

    for line in input {
        let split: Vec<&str> = line.split(" (contains ").collect();
        let ingredients: HashSet<&str> = split[0].split(" ").collect();
        let allergens: Vec<&str> = split[1][..split[1].len() - 1].split(", ").collect();

        let food = Food {
            ingredients: ingredients.clone(),
            allergens,
        };

        ingredients.into_iter().for_each(|ingredient| {
            ingredient_list.insert(ingredient);
        });

        food_list.push(food);
    }

    let mut allergen_map = HashMap::new();

    for food in food_list.iter() {
        for allergen in food.allergens.iter() {
            match allergen_map.entry(allergen.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(food.ingredients.clone());
                }
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() = entry
                        .get()
                        .intersection(&food.ingredients)
                        .map(|i| i.clone())
                        .collect();
                }
            }
        }
    }

    (food_list, ingredient_list, allergen_map)
}

pub fn part1(input: &Vec<&str>) -> usize {
    let (food_list, mut ingredient_list, allergen_map) = parse(input);

    for (_, ingredients) in allergen_map.iter() {
        for ingredient in ingredients {
            ingredient_list.remove(ingredient);
        }
    }
    let mut count = 0;

    for food in food_list.iter() {
        for ingredient in ingredient_list.iter() {
            if food.ingredients.contains(ingredient) {
                count += 1;
            }
        }
    }

    count
}

fn find<'a>(
    allergens: &'a Vec<(&'a str, HashSet<&str>)>,
    res: HashMap<&'a str, &'a str>,
    deep: usize,
) -> Option<HashMap<&'a str, &'a str>> {
    if res.len() == allergens.len() {
        return Some(res);
    }

    let allergen = &allergens[deep];

    for ingredient in allergen.1.iter() {
        if res.contains_key(ingredient) {
            continue;
        }

        let mut res = res.clone();
        res.insert(ingredient, allergen.0);

        let found = find(allergens, res, deep + 1);
        if found.is_some() {
            return found;
        }
    }

    None
}

pub fn part2(input: &Vec<&str>) -> String {
    let (_, _, allergen_map) = parse(input);

    let allergen_vec: Vec<(&str, HashSet<&str>)> = allergen_map
        .into_iter()
        .map(|(allergen, ingredients)| (allergen, ingredients))
        .collect();

    let mut ingredients = find(&allergen_vec, HashMap::new(), 0)
        .unwrap()
        .iter()
        .map(|(ingredients, allergen)| (ingredients.clone(), allergen.clone()))
        .collect::<Vec<_>>();

    ingredients.sort_by(|(_, allergen_a), (_, allergen_b)| allergen_a.cmp(allergen_b));

    ingredients
        .into_iter()
        .map(|(ingredient, _)| ingredient.clone())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 5)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), "mxmxvkd,sqjhc,fvjkl")
    }
}
