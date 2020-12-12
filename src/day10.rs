use itertools::Itertools as _;
use std::collections::BTreeMap;

#[aoc_generator(day10)]
pub fn gen(input: &str) -> BTreeMap<u32, ()> {
    let mut map = BTreeMap::new();

    for i in input.lines().flat_map(str::parse).sorted() {
        map.insert(i, ());
    }

    map
}

#[aoc(day10, part1)]
pub fn part1_impl1(input: &BTreeMap<u32, ()>) -> u32 {
    let mut current_jolt = 0;
    let mut differences = [0; 3];
    let mut range = input.range(..);

    loop {
        let (&next, _) = match range.next() {
            Some(n) => n,
            None => break,
        };
        differences[next as usize - current_jolt as usize - 1] += 1;
        current_jolt = next;
        range = input.range(current_jolt + 1..);
    }

    differences[0] * (differences[2] + 1)
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
