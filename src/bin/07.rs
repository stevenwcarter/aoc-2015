advent_of_code::solution!(7);

use hashbrown::HashMap;

// 123 -> x
// 456 -> y
// x AND y -> d
// x OR y -> e
// x LSHIFT 2 -> f
// y RSHIFT 2 -> g
// NOT x -> h
// NOT y -> i

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, space0},
    combinator::{map, map_res},
    multi::many1,
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
};

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric())(input)
}

fn parse_u16(input: &str) -> IResult<&str, u16> {
    map_res(digit1, |s: &str| s.parse::<u16>())(input)
}

fn parse_signal_line(input: &'_ str) -> IResult<&'_ str, SignalType<'_>> {
    map(
        separated_pair(parse_identifier, tag(" -> "), parse_identifier),
        |(value, output)| SignalType::Input(value, output),
    )(input)
}

fn parse_binary_op<'a>(
    input: &'a str,
    op_tag: &'static str,
    constructor: fn(&'a str, &'a str, &'a str) -> SignalType<'a>,
) -> IResult<&'a str, SignalType<'a>> {
    map(
        tuple((
            parse_identifier,
            preceded(space0, tag(op_tag)),
            preceded(space0, parse_identifier),
            preceded(tag(" -> "), parse_identifier),
        )),
        move |(left, _, right, output)| constructor(left, right, output),
    )(input)
}

fn parse_shift_op<'a>(
    input: &'a str,
    op_tag: &'static str,
    constructor: fn(&'a str, u16, &'a str) -> SignalType<'a>,
) -> IResult<&'a str, SignalType<'a>> {
    map(
        tuple((
            parse_identifier,
            preceded(space0, tag(op_tag)),
            preceded(space0, parse_u16),
            preceded(tag(" -> "), parse_identifier),
        )),
        move |(left, _, shift, output)| constructor(left, shift, output),
    )(input)
}

fn parse_not(input: &'_ str) -> IResult<&'_ str, SignalType<'_>> {
    map(
        tuple((
            tag("NOT "),
            parse_identifier,
            preceded(tag(" -> "), parse_identifier),
        )),
        |(_, input, output)| SignalType::Not(input, output),
    )(input)
}

fn parse_signal(input: &'_ str) -> IResult<&'_ str, SignalType<'_>> {
    nom::branch::alt((
        parse_signal_line,
        |i| parse_binary_op(i, "AND", SignalType::And),
        |i| parse_binary_op(i, "OR", SignalType::Or),
        |i| parse_shift_op(i, "LSHIFT", SignalType::Lshift),
        |i| parse_shift_op(i, "RSHIFT", SignalType::Rshift),
        parse_not,
    ))(input)
}

fn parse_input(input: &'_ str) -> IResult<&'_ str, Vec<SignalType<'_>>> {
    many1(nom::sequence::terminated(
        parse_signal,
        nom::character::complete::line_ending.or(nom::combinator::eof),
    ))(input)
}

#[derive(Debug, Clone, Copy)]
pub enum SignalType<'a> {
    Input(&'a str, &'a str),
    Or(&'a str, &'a str, &'a str),
    And(&'a str, &'a str, &'a str),
    Lshift(&'a str, u16, &'a str),
    Rshift(&'a str, u16, &'a str),
    Not(&'a str, &'a str),
}

fn evaluate<'a>(
    signals: &'a [SignalType<'a>],
    target: &'a str,
    cache: &mut HashMap<&'a str, u16>,
) -> u16 {
    if let Some(&value) = cache.get(target) {
        return value;
    }

    let value = match signals.iter().find(|signal| match signal {
        SignalType::Input(_, output)
        | SignalType::Or(_, _, output)
        | SignalType::And(_, _, output)
        | SignalType::Lshift(_, _, output)
        | SignalType::Rshift(_, _, output)
        | SignalType::Not(_, output) => output == &target,
    }) {
        Some(SignalType::Input(value, _)) => match value.parse::<u16>() {
            Ok(v) => v,
            _ => evaluate(signals, value, cache),
        },
        Some(SignalType::Or(left, right, _)) => {
            let left = match left.parse::<u16>() {
                Ok(v) => v,
                _ => evaluate(signals, left, cache),
            };
            let right = match right.parse::<u16>() {
                Ok(v) => v,
                _ => evaluate(signals, right, cache),
            };
            left | right
        }
        Some(SignalType::And(left, right, _)) => {
            let left = match left.parse::<u16>() {
                Ok(v) => v,
                _ => evaluate(signals, left, cache),
            };
            let right = match right.parse::<u16>() {
                Ok(v) => v,
                _ => evaluate(signals, right, cache),
            };
            left & right
        }
        Some(SignalType::Lshift(left, shift, _)) => {
            let left = match left.parse::<u16>() {
                Ok(v) => v,
                _ => evaluate(signals, left, cache),
            };
            left << shift
        }
        Some(SignalType::Rshift(left, shift, _)) => {
            let left = match left.parse::<u16>() {
                Ok(v) => v,
                _ => evaluate(signals, left, cache),
            };
            left >> shift
        }
        Some(SignalType::Not(input, _)) => !evaluate(signals, input, cache),
        _ => panic!("No matching signal for target: {}", target),
    };

    cache.insert(target, value);
    value
}

fn replace_output<'a>(target: &'a str, value: u16, cache: &mut HashMap<&'a str, u16>) {
    cache.clear();
    cache.insert(target, value);
}

pub fn part_one(input: &str) -> Option<u16> {
    let signals = parse_input(input).unwrap().1;

    // println!("{:#?}", signals);

    let search = if signals.len() < 10 { "d" } else { "a" };

    let mut cache = HashMap::new();
    let result = evaluate(&signals, search, &mut cache);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u16> {
    let signals = parse_input(input).unwrap().1;

    let search = if signals.len() < 10 { "x" } else { "a" };
    let replace = if signals.len() < 10 { "y" } else { "b" };

    let mut cache = HashMap::new();
    let result = evaluate(&signals, search, &mut cache);
    replace_output(replace, result, &mut cache);
    let result = evaluate(&signals, search, &mut cache);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_d() {
        let data = &advent_of_code::template::read_file("examples", DAY);
        let signals = parse_input(data).unwrap().1;
        assert_eq!(evaluate(&signals, "d", &mut HashMap::new()), 72);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(72));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
