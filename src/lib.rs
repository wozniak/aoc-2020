use aoc_runner_derive::*;

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
    let mut on = 0usize;

    for slope in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut x = 0usize;
        for y in (0..map.lines).step_by(slope.1) {
            if map.tree_at(x, y) {
                results[on] += 1;
            }
            x += slope.0;
        }
        on += 1;
    }
    let mut n = 1usize;
    results.iter().for_each(|k| n *= k);
    n
}


aoc_lib!{ year = 2020 }