#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Password {
    requirement: (usize, usize),
    required: char,
    password: String,
}

impl Password {
    fn valid_part1(&self) -> bool {
        let &chars = &self
            .password
            .chars()
            .filter(|&c| c == self.required)
            .count();
        self.requirement.0 <= chars && self.requirement.1 >= chars
    }

    fn valid_part2(&self) -> bool {
        let first = self
            .password
            .chars()
            .nth(self.requirement.0 - 1)
            .unwrap_or('\0');
        let last = self
            .password
            .chars()
            .nth(self.requirement.1 - 1)
            .unwrap_or('\0');

        (first == self.required) ^ (last == self.required)
    }
}

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|s| {
            let (req, rest) = s.split_at(s.find(' ').unwrap());
            let (from, to) = req.split_at(req.find('-').unwrap());
            let (from, to) = (from.parse().unwrap(), (&to[1..]).parse().unwrap());

            let (ch, rest) = rest.trim().split_at(rest.find(':').unwrap());
            let ch = (&ch[..1]).parse().unwrap();

            Password {
                requirement: (from, to),
                required: ch,
                password: (&rest[1..]).into(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1_impl1(input: &[Password]) -> usize {
    input.iter().filter(|p| p.valid_part1()).count()
}

#[aoc(day2, part2)]
pub fn part2_impl1(input: &[Password]) -> usize {
    input.iter().filter(|p| p.valid_part2()).count()
}
