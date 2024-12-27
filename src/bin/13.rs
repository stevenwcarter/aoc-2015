advent_of_code::solution!(13);

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::map,
    sequence::tuple,
    IResult, Parser,
};

#[derive(Debug, PartialEq)]
struct Pairing<'a> {
    name1: &'a str,
    name2: &'a str,
    happiness: i64,
}

fn parse_name(input: &str) -> IResult<&str, &str> {
    take_until(" would")(input)
}

fn parse_happiness(input: &str) -> IResult<&str, i64> {
    map(
        tuple((
            map(tag("gain "), |_| 1).or(map(tag("lose "), |_| -1)),
            digit1,
        )),
        |(sign, digits): (i64, &str)| {
            let number: i64 = digits.parse().unwrap();
            sign * number
        },
    )(input)
}

fn parse_pairing(input: &str) -> IResult<&str, Pairing> {
    map(
        tuple((
            parse_name,
            tag(" would "),
            parse_happiness,
            tag(" happiness units by sitting next to "),
            take_until("."),
            tag("."),
        )),
        |(name1, _, value, _, name2, _)| Pairing {
            name1,
            name2,
            happiness: value,
        },
    )(input)
}

fn parse_all(input: &str) -> IResult<&str, Vec<Pairing>> {
    nom::multi::separated_list1(tag("\n"), parse_pairing)(input)
}

fn get_score_for_order(names: &[&str], happy_map: &HashMap<(&str, &str), i64>) -> i64 {
    let first_name = names[0];

    names
        .iter()
        .chain(vec![&first_name])
        .tuple_windows()
        .map(|(&n1, &n2)| happy_map.get(&(n1, n2)).unwrap())
        .sum()
}

pub fn part_one(input: &str) -> Option<i64> {
    let pairings = parse_all(input).unwrap().1;
    let mut happy_map: HashMap<(&str, &str), i64> = HashMap::new();
    let mut names: HashSet<&str> = HashSet::new();
    pairings.iter().for_each(|p| {
        *happy_map.entry((p.name1, p.name2)).or_insert(0) += p.happiness;
        *happy_map.entry((p.name2, p.name1)).or_insert(0) += p.happiness;
        names.insert(p.name1);
        names.insert(p.name2);
    });

    let names_len = names.len();
    names
        .into_iter()
        .permutations(names_len)
        .map(|p| get_score_for_order(&p, &happy_map))
        .max()
}

pub fn part_two(input: &str) -> Option<i64> {
    let pairings = parse_all(input).unwrap().1;
    let mut happy_map: HashMap<(&str, &str), i64> = HashMap::new();
    let mut names: HashSet<&str> = HashSet::new();
    pairings.iter().for_each(|p| {
        *happy_map.entry((p.name1, p.name2)).or_insert(0) += p.happiness;
        *happy_map.entry((p.name2, p.name1)).or_insert(0) += p.happiness;
        happy_map.entry((p.name2, "ME")).or_insert(0);
        happy_map.entry((p.name1, "ME")).or_insert(0);
        happy_map.entry(("ME", p.name2)).or_insert(0);
        happy_map.entry(("ME", p.name1)).or_insert(0);
        names.insert(p.name1);
        names.insert(p.name2);
    });
    names.insert("ME");

    let names_len = names.len();
    names
        .into_iter()
        .permutations(names_len)
        .map(|p| get_score_for_order(&p, &happy_map))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(330));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
