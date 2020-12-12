#[aoc_generator(day6)]
pub fn gen(input: &str) -> Vec<([u32; 26], usize)> {
    input
        .split("\n\n")
        .map(|s| {
            let mut counts = [0u32; 26];
            for c in s.chars().filter(|c| c.is_ascii_lowercase()) {
                counts[c as usize - 'a' as usize] += 1;
            }

            (counts, s.lines().filter(|s| !s.trim().is_empty()).count())
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn part1_impl1(input: &[([u32; 26], usize)]) -> usize {
    input
        .iter()
        .map(|(counts, _)| counts.iter().filter(|&&count| count != 0).count())
        .sum()
}

#[aoc(day6, part2)]
pub fn part2_impl1(input: &[([u32; 26], usize)]) -> usize {
    input
        .iter()
        .map(|(counts, people)| {
            counts
                .iter()
                .filter(|&&count| count == *people as u32)
                .count()
        })
        .sum()
}
