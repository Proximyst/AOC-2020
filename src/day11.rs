#[derive(Clone, Copy)]
pub enum Tile {
    Occupied,
    Free,
    Floor,
}

#[aoc_generator(day11)]
pub fn gen(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    'L' => Tile::Free,
                    '.' => Tile::Floor,
                    '#' => Tile::Occupied,
                    c @ _ => panic!("Unknown tile: {}", c),
                })
                .collect()
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn part1_impl1(input: &Vec<Vec<Tile>>) -> usize {
    let mut current = input.clone();
    let mut next = Vec::with_capacity(input.len());
    let mut occupied = 0;
    let mut previous = 1;

    while occupied != previous {
        previous = occupied;

        next.clear();
        for y in 0..current.len() {
            let row = current[y];
            for x in 0..row.len() {
                let mut column = row.get_mut(x);
            }
        }
        for row in &current {
            let mut new_row = row.clone();
        }
    }

    occupied
}
