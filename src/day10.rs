use itertools::Itertools as _;
use std::collections::{BTreeMap, HashMap};

#[aoc_generator(day10)]
pub fn gen(input: &str) -> Vec<u64> {
    input.lines().flat_map(str::parse).sorted().collect()
}

#[aoc(day10, part1)]
pub fn part1_impl1(input: &[u64]) -> u64 {
    let mut map = BTreeMap::new();
    for &i in input {
        map.insert(i, ());
    }

    let mut current_jolt = 0;
    let mut differences = [0; 3];
    let mut range = map.range(..);

    loop {
        let (&next, _) = match range.next() {
            Some(n) => n,
            None => break,
        };
        differences[next as usize - current_jolt as usize - 1] += 1;
        current_jolt = next;
        range = map.range(current_jolt + 1..);
    }

    differences[0] * (differences[2] + 1)
}

#[aoc(day10, part2)]
pub fn part2_impl1(input: &Vec<u64>) -> u64 {
    let mut input = input.clone();
    input.insert(0, 0);
    input.push(input.last().copied().unwrap() + 3);
    _part2_impl1(&mut HashMap::new(), &input)
}

fn _part2_impl1(cache: &mut HashMap<u64, u64>, input: &[u64]) -> u64 {
    if input.len() == 1 {
        return 1;
    }

    let first = input[0];
    if let Some(cached) = cache.get(&first).copied() {
        return cached;
    }

    let i = input
        .iter()
        .skip(1)
        .enumerate()
        .take_while(|&(_, &val)| val - first <= 3)
        .map(|(idx, &_)| _part2_impl1(cache, &input[idx + 1..]))
        .sum();

    cache.insert(first, i);
    i
}

#[test]
fn part1_test() {
    assert_eq!(
        part1_impl1(&gen("16
10
15
5
1
11
7
19
6
12
4")),
        35,
    );
    assert_eq!(
        part1_impl1(&gen("28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3")),
        220,
    );
}

#[test]
fn part2_test() {
    assert_eq!(
        part2_impl1(&gen("16
10
15
5
1
11
7
19
6
12
4")),
        8,
    );
    assert_eq!(
        part2_impl1(&gen("28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3")),
        19208,
    );
    // https://www.reddit.com/r/adventofcode/comments/kapbjb/
    assert_eq!(
        part2_impl1(&gen("10
6
4
7
1
5")),
        4,
    );
    assert_eq!(
        part2_impl1(&gen("4
11
7
8
1
6
5")),
        7,
    );
    assert_eq!(
        part2_impl1(&gen("3
1
6
2")),
        4,
    );
    assert_eq!(
        part2_impl1(&gen("17
6
10
5
13
7
1
4
12
11
14")),
        28,
    );
}
