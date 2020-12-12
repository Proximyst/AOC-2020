const PREAMBLE: usize = 25;

#[aoc_generator(day9)]
pub fn gen(input: &str) -> Vec<u64> {
    input.lines().flat_map(str::parse).collect()
}

#[aoc(day9, part1)]
pub fn part1_impl1(input: &[u64]) -> u64 {
    debug_assert!(input.len() > PREAMBLE);

    'outer: for (idx, &i) in (&input[PREAMBLE..]).iter().enumerate() {
        let preamble = &input[idx..idx + PREAMBLE];
        for (pidx, &p) in preamble.iter().enumerate() {
            for (ppidx, &pp) in preamble.iter().enumerate() {
                if pidx != ppidx && i == p + pp {
                    continue 'outer;
                }
            }
        }

        return i;
    }

    unreachable!()
}

#[aoc(day9, part2)]
pub fn part2_impl1(input: &[u64]) -> u64 {
    let part1 = part1_impl1(input);

    let mut idx = 0;
    while idx < input.len() {
        let mut sum = 0;
        let slice = &input[idx..(idx + PREAMBLE).min(input.len())];
        for (iidx, &i) in slice.iter().enumerate() {
            sum += i;
            if sum == part1 {
                let slice = &slice[..=iidx];
                return slice.iter().min().unwrap() + slice.iter().max().unwrap();
            } else if sum > part1 {
                break;
            }
        }

        idx += 1;
    }

    unreachable!()
}
