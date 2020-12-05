use aoc_runner_derive::*;
use std::cmp::Ordering;

#[aoc_generator(day1)]
pub fn day1_gen(input: &str) -> Vec<u32> {
    input.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn d1p1(vec: &Vec<u32>) -> u32 {
    for n1 in vec {
        for n2 in vec {
            if n1 + n2 == 2020 { return n1 * n2 }
        }
    }
    0
}

#[aoc(day1, part2)]
pub fn d1p2(vec: &Vec<u32>) -> u32 {
    for n1 in vec {
        for n2 in vec {
            for n3 in vec {
                if n1 + n2 + n3 == 2020 { return n1 * n2 * n3 }
            }
        }
    }
    0
}

// day 2
pub struct Password {
    pub string: String,
    pub min: usize,
    pub max: usize,
    pub ch: char,
}

#[aoc_generator(day2)]
pub fn day2_gen(input: &str) -> Vec<Password> {
    let mut vec = Vec::new();

    for line in input.split('\n') {
        let mut min: usize = 0;
        let mut max: usize = 0;
        let mut ch: char = '\0';
        let mut string = String::new();
        let mut n: u8 = 0;

        for word in line.split_whitespace() {
            match n {
                0 => {
                    let map: Vec<usize> = word.split('-').map(|n| n.parse::<usize>().unwrap()).collect();
                    min = map[0];
                    max = map[1];
                }
                1 => {
                    ch = word[0..1].parse().unwrap();
                }
                2 => {
                    string = word.to_string();
                }
                _ => { panic!("your code sucks")}
            }
            n += 1;
        }

        vec.push(Password { min, max, ch, string });
    }

    vec
}

#[aoc(day2, part1)]
pub fn d2p1(passwords: &Vec<Password>) -> usize {
    let mut n: usize = 0;
    for pass in passwords {
        let mut letter_count: usize = 0;
        let ch = pass.ch as u8;
        pass.string.as_bytes().iter().for_each(|c| { if *c == ch { letter_count += 1; } });
        if letter_count <= pass.max && letter_count >= pass.min { n += 1; }
    }
    n
}

#[aoc(day2, part2)]
pub fn d2p2(passwords: &Vec<Password>) -> usize {
    let mut n: usize = 0;

    for pass in passwords {
        let is_at_first  = pass.string.as_bytes()[pass.min - 1] == pass.ch as u8;
        let is_at_second = pass.string.as_bytes()[pass.max - 1] == pass.ch as u8;
        if is_at_first ^ is_at_second { n += 1 }
    }
    n
}

// day 3
struct Map {
    map: Vec<u8>,
    width: usize,
    lines: usize,
}

impl Map {
    pub fn tree_at(&self, x: usize, y: usize) -> bool {
        self.map[self.width * y + (x % self.width)] == b'#'
    }
}

#[aoc_generator(day3)]
fn day3_gen(input: &str) -> Map {
    let width = input.lines().next().unwrap().len();
    let lines = input.lines().count();

    let mut string = input.to_string();
    string.retain(|f| !f.is_whitespace());
    let map = string.into_bytes();

    Map { map, width, lines }
}

#[aoc(day3, part1)]
fn d3p1(map: &Map) -> usize {
    let mut n = 0usize;
    let mut x = 0usize;

    for line in 0..map.lines {
        if map.tree_at(x, line) { n += 1 }
        x += 3;
    }
    n
}

#[aoc(day3, part2)]
fn d3p2(map: &Map) -> usize {
    let results = &mut [0, 0, 0, 0, 0];
    let mut current = 0usize;

    for slope in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut x = 0usize;
        for y in (0..map.lines).step_by(slope.1) {
            if map.tree_at(x, y) {
                results[current] += 1;
            }
            x += slope.0;
        }
        current += 1;
    }
    let mut n = 1usize;
    results.iter().for_each(|k| n *= k);
    n
}

// day 4
#[derive(Clone, Debug)]
pub struct Passport {
    valid: u8,
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn is_valid(&self, is_part_one: bool) -> bool {
        // don't care about cid as cid is *always* valid
        if !(self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some()) {
            return false
        }

        if is_part_one { return true }

        match self.byr.as_ref().unwrap().parse::<u32>() {
            Ok(y) => if !(1920..=2002).contains(&y) { return false }
            Err(_) => return false
        }

        match self.iyr.as_ref().unwrap().parse() {
            Ok(y) => if !(2010..=2020).contains(&y) { return false },
            Err(_) => return false,
        }

        match self.eyr.as_ref().unwrap().parse() {
            Ok(y) => if !(2020..=2030).contains(&y) { return false },
            Err(_) => return false,
        }

        let min;
        let max;

        if self.hgt.as_ref().unwrap().ends_with("cm") {
            min = 150;
            max = 193;
        } else if self.hgt.as_ref().unwrap().ends_with("in") {
            min = 59;
            max = 76;
        } else {
            return false
        }

        if !(min..=max).contains(&match self.hgt.as_ref().unwrap().as_str()
            [..self.hgt.as_ref().unwrap().len() - 2].parse::<i32>() {
            Ok(n) => n,
            Err(_) => return false,
        }) { return false }

        if !self.hcl.as_ref().unwrap().starts_with('#')
            || u32::from_str_radix(&self.hcl.as_ref().unwrap()[1..], 16).is_err() {
            return false
        };

        if !&["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl.as_ref().unwrap().as_str()) {
            return false
        }

        self.pid.as_ref().unwrap().len() == 9 && self.pid.as_ref().unwrap().parse::<usize>().is_ok()

    }
}

impl std::default::Default for Passport {
    fn default() -> Self {
        Self { valid: 0, byr: None, iyr: None, eyr: None, hgt: None, hcl: None, ecl: None, pid: None }
    }
}

#[aoc_generator(day4)]
pub fn day4_gen(input: &str) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut passport: Passport = Default::default();
    for line in input.lines() {
        if line == "" {
            passports.push(passport);
            passport = Default::default();
        }
        for word in line.split_whitespace() {
            let mut key_value = word.split(':');
            let key = key_value.next().unwrap();
            let value = key_value.next().unwrap().to_string();
            match key {
                "byr" => passport.byr = Some(value),
                "iyr" => passport.iyr = Some(value),
                "eyr" => passport.eyr = Some(value),
                "hgt" => passport.hgt = Some(value),
                "hcl" => passport.hcl = Some(value),
                "ecl" => passport.ecl = Some(value),
                "pid" => passport.pid = Some(value),
                _ => {}
            }
        }
    }
    passports
}

#[aoc(day4, part1)]
fn d4p1(passports: &Vec<Passport>) -> usize {
    let mut n = 0usize;
    passports.iter().for_each(|i| if i.is_valid(true) { n += 1 });
    n
}

#[aoc(day4, part2)]
fn d4p2(passports: &Vec<Passport>) -> usize {
    let mut n = 0usize;
    passports.iter().for_each(|i| if i.is_valid(false) { n += 1 });
    n
}

// day 5
#[derive(Copy, Clone)]
struct BoardingPass {
    row: u8,
    column: u8,
    seat_id: u32,
}

impl From<&str> for BoardingPass {
    fn from(input: &str) -> Self {
        let row_vec = (0..=127u8).collect::<Vec<u8>>();
        let column_vec = (0..=7).collect::<Vec<u8>>();

        let mut row = row_vec.as_slice();
        let mut column = column_vec.as_slice();

        for ch in input.as_bytes() {
            match *ch {
                b'F' => row = &row[..row.len() / 2],
                b'B' => row = &row[row.len() / 2..],
                b'L' => column = &column[..column.len() / 2],
                b'R' => column = &column[column.len() / 2..],
                _ => panic!("your code sucks"),
            }
        }

        Self { row: row[0], column: column[column.len() - 1], seat_id: row[0] as u32 * 8 + column[column.len() - 1] as u32 }
    }
}

#[aoc_generator(day5)]
fn day5_gen(input: &str) -> Vec<BoardingPass> {
    input.lines().map(|line| BoardingPass::from(line)).collect()
}

#[aoc(day5, part1)]
fn d5p1(boarding_passes: &Vec<BoardingPass>) -> u32 {
    let mut max = 0u32;
    for pass in boarding_passes {
        if pass.seat_id > max { max = pass.seat_id }
    }

    max
}

#[aoc(day5, part2)]
fn d5p2(boarding_passes: &Vec<BoardingPass>) -> u32 {
    let mut locs = boarding_passes.iter().map(|n| (n.row, n.column)).collect::<Vec<_>>();
    locs.sort();

    for i in 0..locs.len() - 1 {
        let here = locs[i];
        let next = locs[i + 1];

        if next.0 - here.0 == 1 {
            if next.1 == 2 {
                return next.0 as u32 * 8 + next.1 as u32;
            }
        }

        if next.1 - here.1 == 2 {
            return next.0 as u32 * 8 + here.1 as u32 + 1;
        }

    }

    0
}

aoc_lib!{ year = 2020 }