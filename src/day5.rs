#[aoc_generator(day5)]
pub fn gen(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|s| {
            s.chars().fold(0, |acc, c| match c {
                'F' => acc * 2,
                'B' => acc * 2 + 1,
                'L' => acc * 2,
                'R' => acc * 2 + 1,
                c @ _ => panic!("Unexpected char: {}", c),
            })
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1_impl1(input: &[u32]) -> u32 {
    input.iter().max().copied().unwrap()
}

#[aoc(day5, part2)]
pub fn part2_impl1(input: &[u32]) -> u32 {
    let mut v = input.to_vec();
    v.sort_unstable();

    v.iter()
        .zip((&v[1..]).iter())
        .find(|&(front, back)| back - front == 2)
        .unwrap()
        .0
        + 1
}

#[test]
fn parse_test() {
    assert_eq!(gen("BFFFBBFRRR"), vec![567]);
}
