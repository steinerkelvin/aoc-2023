/* Day 1: Trebuchet

The newly-improved calibration document consists of lines of text; each line
originally contained a specific calibration value that the Elves now need to
recover. On each line, the calibration value can be found by combining the first
digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
In this example, the calibration values of these four lines are 12, 38, 15, and
77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the
calibration values?
*/

const DAY_1_INPUT_A: &str = include_str!("../input/day_1_a.txt");
const DAY_1_INPUT_B: &str = include_str!("../input/day_1_b.txt");

fn day_1_trebuchet() -> Result<u32, Box<dyn std::error::Error>> {
    let body = DAY_1_INPUT_A;
    let result = body.lines().map(|line| {
        let mut chars = line.chars();
        let d1 = loop {
            let c = chars.next().expect("There are no digits");
            if let Some(d1) = c.to_digit(10) {
                break d1;
            }
        };
        let mut chars = line.chars();
        loop {
            let c = chars.next_back().expect("There are no digits");
            if let Some(d2) = c.to_digit(10) {
                return 10 * d1 + d2;
            }
        }
    });
    let result = result.sum::<u32>();
    Ok(result)
}

/* Day 1: Trebuchet, part 2

Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?

*/

const NUMBERS: &[(&str, u32)] = &[
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn prefix_to_digit(prefix: &str) -> Option<u32> {
    NUMBERS.iter().find_map(|(word, digit)| {
        if prefix.starts_with(word) {
            Some(*digit)
        } else {
            None
        }
    })
}

fn day_1_trebuchet_part_2() -> u32 {
    let body = DAY_1_INPUT_B;

    let result = body.lines().map(|line| {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        let mut chars = line.chars();
        loop {
            let cur_str = chars.as_str();
            if let Some(n) = prefix_to_digit(cur_str) {
                first = first.or(Some(n));
                last = Some(n);
            }
            if chars.next().is_none() {
                break;
            }
        };
        let first = first.expect("There are no digits");
        let last = last.expect("There are no digits");
        10 * first + last
    });
    result.sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 1 part 1: {}", day_1_trebuchet().unwrap());
    println!("Day 1 part 2: {}", day_1_trebuchet_part_2());
    Ok(())
}
