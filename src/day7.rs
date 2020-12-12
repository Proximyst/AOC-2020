use rayon::prelude::*;
use std::collections::HashMap;

pub type Bags = HashMap<String, Vec<(u32, String)>>;

mod parse {
    use super::Bags;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_until, take_while},
        character::complete::{char, line_ending, space0},
        combinator::{map, opt},
        error::VerboseError,
        multi::{separated_list0, separated_list1},
        sequence::{delimited, separated_pair},
    };

    type IR<'a, I, O, E = VerboseError<&'a str>> = nom::IResult<I, O, E>;

    fn bag(i: &str) -> IR<&str, &str> {
        let (i, desc) = take_until(" bag")(i)?;
        let (i, _) = tag(" bag")(i)?;
        let (i, _) = opt(char('s'))(i)?;

        Ok((i, desc))
    }

    fn rule(i: &str) -> IR<&str, (String, Vec<(u32, String)>)> {
        let (i, input) = bag(i)?;
        let (i, _) = delimited(space0, tag("contain"), space0)(i)?;
        let (i, contained) = alt((
            map(tag("no other bags"), |_| Vec::new()),
            separated_list1(
                delimited(space0, char(','), space0),
                map(
                    separated_pair(take_while(|c: char| c.is_ascii_digit()), space0, bag),
                    |(num, bag)| (num.parse().unwrap(), bag.to_owned()),
                ),
            ),
        ))(i)?;
        let (i, _) = char('.')(i)?;

        Ok((i, (input.to_owned(), contained)))
    }

    fn rules(i: &str) -> IR<&str, Vec<(String, Vec<(u32, String)>)>> {
        separated_list0(line_ending, rule)(i)
    }

    pub fn rules_map(i: &str) -> IR<&str, Bags> {
        map(rules, |r| {
            let mut map = Bags::with_capacity(r.len());
            for (name, contained) in r {
                map.insert(name, contained);
            }
            map
        })(i)
    }

    #[test]
    fn test_parsing() {
        assert_eq!(
            rules("light red bags contain 1 bright white bag, 2 muted yellow bags.").unwrap(),
            (
                "",
                vec![(
                    "light red".into(),
                    vec![(1, "bright white".into()), (2, "muted yellow".into())],
                )],
            ),
        );
        assert_eq!(
            rules(
                "vibrant plum bags contain 5 faded blue bags, 1 dotted black bag.
faded blue bags contain no other bags."
            )
            .unwrap(),
            (
                "",
                vec![
                    (
                        "vibrant plum".into(),
                        vec![(5, "faded blue".into()), (1, "dotted black".into())],
                    ),
                    ("faded blue".into(), vec![]),
                ],
            ),
        );
    }
}

#[aoc_generator(day7)]
pub fn gen(input: &str) -> Bags {
    let (rest, map) = parse::rules_map(input).unwrap();
    assert!(rest.is_empty());
    map
}

#[aoc(day7, part1)]
pub fn part1_impl1(input: &Bags) -> usize {
    const MAGIC: &str = "shiny gold";

    input
        .par_iter() // This'll definitely be needed...
        .filter(|&(key, _)| key != MAGIC)
        .filter(|&(key, _)| contains_bag_recursive(key, None, MAGIC, input))
        .count()
}

#[aoc(day7, part2)]
pub fn part2_impl1(input: &Bags) -> u32 {
    const MAGIC: &str = "shiny gold";

    count_inner_bags(MAGIC, input)
}

fn contains_bag_recursive(start: &str, current: Option<&str>, needle: &str, map: &Bags) -> bool {
    let current = match current {
        None => start,
        Some(s) if start != s => s,
        Some(_) => return false, // We've already gone through this bag.
    };

    let bags = match map.get(current) {
        Some(b) if !b.is_empty() => b,
        Some(_) | None => return false, // This contains no bags
    };

    if bags.iter().any(|(_, inner)| {
        inner == needle || contains_bag_recursive(start, Some(inner), needle, map)
    }) {
        return true;
    }
    false
}

fn count_inner_bags(bag: &str, map: &Bags) -> u32 {
    let inner = match map.get(bag) {
        Some(b) if !b.is_empty() => b,
        Some(_) | None => return 0,
    };

    inner
        .iter()
        .map(|(count, inner)| *count * (count_inner_bags(inner, map) + 1))
        .sum()
}

#[test]
fn part1_test() {
    assert_eq!(
        part1_impl1(&gen(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.",
        )),
        4,
    );
}

#[test]
fn part2_test() {
    assert_eq!(
        part2_impl1(&gen(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.",
        )),
        32,
    );
    assert_eq!(
        part2_impl1(&gen("shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.",)),
        126,
    );
}
