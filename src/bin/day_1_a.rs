const DAY_1_INPUT_A: &str = include_str!("../../input/day_1_a.txt");

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = day_1_trebuchet()?;
    println!("Day 1 part 1 result: {}", result);
    Ok(())
}
