advent_of_code::solution!(16);

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Sue {
    sue_id: u32,
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

fn parse_sue(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}
fn parse_number(input: &str) -> IResult<&str, u8> {
    map_res(digit1, str::parse)(input)
}

fn parse_key_value(input: &str) -> IResult<&str, (&str, u8)> {
    separated_pair(alpha1, tag(": "), parse_number)(input)
}

fn parse_sue_line(input: &str) -> IResult<&str, Sue> {
    let (input, _) = tag("Sue ")(input)?;
    let (input, sue_id) = parse_sue(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, attributes) = separated_list0(tag(", "), parse_key_value)(input)?;

    let mut attr_map: HashMap<&str, u8> = HashMap::new();
    for (key, value) in attributes {
        attr_map.insert(key, value);
    }

    let sue = Sue {
        sue_id,
        children: attr_map.get("children").copied(),
        cats: attr_map.get("cats").copied(),
        samoyeds: attr_map.get("samoyeds").copied(),
        pomeranians: attr_map.get("pomeranians").copied(),
        akitas: attr_map.get("akitas").copied(),
        vizslas: attr_map.get("vizslas").copied(),
        goldfish: attr_map.get("goldfish").copied(),
        trees: attr_map.get("trees").copied(),
        cars: attr_map.get("cars").copied(),
        perfumes: attr_map.get("perfumes").copied(),
    };

    Ok((input, sue))
}

fn parse_all_sues(input: &str) -> IResult<&str, Vec<Sue>> {
    separated_list0(multispace0, parse_sue_line)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let sues = parse_all_sues(input).unwrap().1;
    let aunt = sues
        .iter()
        .filter(|l| l.children.is_none() || l.children == Some(3))
        .filter(|l| l.cats.is_none() || l.cats == Some(7))
        .filter(|l| l.samoyeds.is_none() || l.samoyeds == Some(2))
        .filter(|l| l.pomeranians.is_none() || l.pomeranians == Some(3))
        .filter(|l| l.akitas.is_none() || l.akitas == Some(0))
        .filter(|l| l.vizslas.is_none() || l.vizslas == Some(0))
        .filter(|l| l.goldfish.is_none() || l.goldfish == Some(5))
        .filter(|l| l.trees.is_none() || l.trees == Some(3))
        .filter(|l| l.cars.is_none() || l.cars == Some(2))
        .find(|l| l.perfumes.is_none() || l.perfumes == Some(1))
        .unwrap();

    Some(aunt.sue_id)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sues = parse_all_sues(input).unwrap().1;
    let aunt = sues
        .iter()
        .filter(|l| l.children.is_none() || l.children == Some(3))
        .filter(|l| l.cats.is_none() || l.cats > Some(7))
        .filter(|l| l.samoyeds.is_none() || l.samoyeds == Some(2))
        .filter(|l| l.pomeranians.is_none() || l.pomeranians < Some(3))
        .filter(|l| l.akitas.is_none() || l.akitas == Some(0))
        .filter(|l| l.vizslas.is_none() || l.vizslas == Some(0))
        .filter(|l| l.goldfish.is_none() || l.goldfish < Some(5))
        .filter(|l| l.trees.is_none() || l.trees > Some(3))
        .filter(|l| l.cars.is_none() || l.cars == Some(2))
        .find(|l| l.perfumes.is_none() || l.perfumes == Some(1))
        .unwrap();

    Some(aunt.sue_id)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }
//
//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, None);
//     }
// }
