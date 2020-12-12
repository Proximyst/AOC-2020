use std::collections::HashSet;
use itertools::Itertools as _;

#[aoc_generator(day1)]
pub fn gen(input: &str) -> HashSet<i32> {
    input.lines().flat_map(str::parse).collect()
}

#[aoc(day1, part1)]
pub fn part1_impl1(input: &HashSet<i32>) -> i32 {
    const MAGIC: i32 = 2020;

    for i in input.iter() {
        let remaining = MAGIC - i;
        if input.contains(&remaining) {
            return remaining * i;
        }
    }

    unreachable!()
}

#[aoc(day1, part2)]
pub fn part2_impl1(input: &HashSet<i32>) -> i32 {
    const MAGIC: i32 = 2020;

    for combos in input.iter().combinations(2) {
        let &a = combos[0];
        let &b = combos[1];
        let remaining = MAGIC - a - b;
        if input.contains(&remaining) {
            return a * b * remaining;
        }
    }

    unreachable!()
}
