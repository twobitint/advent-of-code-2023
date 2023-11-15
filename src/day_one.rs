use crate::traits::Solves;
use scan_fmt::scan_fmt;

pub struct Solver {}

impl Solver {
    fn line_value(line: &str) -> i16 {
        scan_fmt!(line, "{d}", i16).unwrap()
    }
}

impl Solves for Solver {
    const DAY: u8 = 1;

    fn part_one(&self, input: &str) -> String {
        let mut prev: i16 = Self::line_value(input.lines().next().unwrap());
        let mut count: i16 = 0;
        for line in input.lines() {
            let depth = Self::line_value(line);
            if depth > prev {
                count += 1;
            }
            prev = depth;
        }
        count.to_string()
    }
}