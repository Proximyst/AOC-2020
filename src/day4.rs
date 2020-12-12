use std::collections::HashMap;

#[derive(Debug)]
pub struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn valid(&self) -> bool {
        match self.byr {
            Some(yr) if yr >= 1920 && yr <= 2002 => (),
            _ => return false,
        }

        match self.iyr {
            Some(yr) if yr >= 2010 && yr <= 2020 => (),
            _ => return false,
        }

        match self.eyr {
            Some(yr) if yr >= 2020 && yr <= 2030 => (),
            _ => return false,
        }

        match &self.pid {
            Some(s) if s.len() == 9 && s.chars().all(|c| c.is_ascii_digit()) => (),
            _ => return false,
        }

        match &self.ecl {
            Some(s) if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s.as_str()) => {
                ()
            }
            _ => return false,
        }

        match &self.hcl {
            Some(s)
                if s.len() == 7
                    && s.starts_with('#')
                    && (&s[1..]).chars().all(|c| c.is_ascii_hexdigit()) =>
            {
                ()
            }
            _ => return false,
        }

        let hgt = match &self.hgt {
            Some(s) if s.ends_with("in") || s.ends_with("cm") => s,
            _ => return false,
        };
        let (hgt, unit) = (
            (&hgt[..hgt.len() - 2]).parse::<i32>(),
            &hgt[hgt.len() - 2..],
        );
        match hgt {
            Ok(hgt) if unit == "in" && hgt >= 59 && hgt <= 76 => (),
            Ok(hgt) if unit == "cm" && hgt >= 150 && hgt <= 193 => (),
            _ => return false,
        }

        true
    }
}

#[aoc_generator(day4)]
pub fn gen(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|id| {
            let properties = id
                .split_whitespace()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let (first, second) = s.split_at(s.find(':').unwrap());
                    (first, &second[1..])
                })
                .collect::<HashMap<&str, &str>>();

            Passport {
                byr: properties.get("byr").map(|s| s.parse().unwrap()),
                iyr: properties.get("iyr").map(|s| s.parse().unwrap()),
                eyr: properties.get("eyr").map(|s| s.parse().unwrap()),
                hgt: properties.get("hgt").map(|&s| s.to_owned()),
                hcl: properties.get("hcl").map(|&s| s.to_owned()),
                ecl: properties.get("ecl").map(|&s| s.to_owned()),
                pid: properties.get("pid").map(|&s| s.to_owned()),
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1_impl1(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(
            |Passport {
                 byr,
                 iyr,
                 eyr,
                 hgt,
                 ecl,
                 hcl,
                 pid,
             }| {
                byr.is_some()
                    && iyr.is_some()
                    && eyr.is_some()
                    && hgt.is_some()
                    && ecl.is_some()
                    && hcl.is_some()
                    && pid.is_some()
            },
        )
        .count()
}

#[aoc(day4, part2)]
pub fn part2_impl1(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.valid()).count()
}
