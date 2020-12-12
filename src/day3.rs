#[aoc_generator(day3)]
pub fn gen(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c == '#').collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn part1_impl1(input: &[Vec<bool>]) -> u32 {
    slope(input, (3, 1))
}

#[aoc(day3, part2)]
pub fn part2_impl1(input: &[Vec<bool>]) -> u32 {
    slope(input, (1, 1))
        * slope(input, (3, 1))
        * slope(input, (5, 1))
        * slope(input, (7, 1))
        * slope(input, (1, 2))
}

fn slope(input: &[Vec<bool>], slope: (usize, usize)) -> u32 {
    let mut coord = (0, 0);
    let mut trees = 0;
    while coord.1 < input.len() {
        let y = &input[coord.1 % input.len()];
        if y[coord.0 % y.len()] {
            trees += 1;
        }

        coord.0 += slope.0;
        coord.1 += slope.1;
    }

    trees
}
