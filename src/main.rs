// Inputs

const DAY_1_INPUT_A: &str = include_str!("../input/day_1_a.txt");
const DAY_1_INPUT_B: &str = include_str!("../input/day_1_b.txt");
const DAY_2_INPUT_A: &str = include_str!("../input/day_2_a.txt");

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
        }
        let first = first.expect("There are no digits");
        let last = last.expect("There are no digits");
        10 * first + last
    });
    result.sum()
}


/* Day 2: Cube Conundrum

You play several games and record the information from each game (your puzzle
input). Each game is listed with its ID number (like the 11 in Game 11: ...)
followed by a semicolon-separated list of subsets of cubes that were
revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

In game 1, three sets of cubes are revealed from the bag (and then put back
again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red
cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag
contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had
been loaded with that configuration. However, game 3 would have been impossible
because at one point the Elf showed you 20 red cubes at once; similarly, game 4
would also have been impossible because the Elf showed you 15 blue cubes at
once. If you add up the IDs of the games that would have been possible, you get
8.

Determine which games would have been possible if the bag had been loaded with
only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs
of those games?
*/

// Parser combinators

#[derive(Clone, Copy)]
struct Parser<'a> {
    body: &'a str,
    pos: usize,
}

impl Parser<'_> {
    fn as_str(&self) -> &str {
        &self.body[self.pos..]
    }
}

impl<'a> Parser<'a> {
    fn at(self, pos: usize) -> Parser<'a> {
        Parser {
            body: self.body,
            pos,
        }
    }
    fn with<T>(self, item: T) -> ParserResult<'a, T> {
        Some((self, item))
    }
}

type ParserResult<'a, T> = Option<(Parser<'a>, T)>;

fn exact<'a>(parser: Parser<'a>, s: &str) -> ParserResult<'a, ()> {
    let body = parser.body;
    let pos = parser.pos;
    let end = pos + s.len();
    if end > body.len() {
        return None;
    }
    if &body[pos..end] != s {
        return None;
    }
    parser.at(end).with(())
}

fn skip_whitespace(parser: Parser) -> ParserResult<()> {
    let body = parser.body;
    let mut end = parser.pos;
    while let Some(c) = body[end..].chars().next() {
        if !c.is_whitespace() {
            break;
        }
        end += 1;
    }
    parser.at(end).with(())
}

fn digit(parser: Parser) -> ParserResult<char> {
    let c = parser.as_str();
    if let Some(c) = c.chars().next() {
        if c.is_ascii_digit() {
            return parser.at(parser.pos + 1).with(c);
        }
    }
    None
}

fn repeat_to_str<T>(parser: Parser, f: fn(Parser) -> ParserResult<T>) -> ParserResult<&str> {
    let start = parser;
    let mut cur_parser = parser;
    while let Some((parser, _)) = f(cur_parser) {
        cur_parser = parser;
    }
    let end = cur_parser.pos;
    cur_parser.with(&start.body[start.pos..end])
}

fn parse_u32(parser: Parser) -> ParserResult<u32> {
    let (parser, digits) = repeat_to_str(parser, digit)?;
    let n = digits.parse::<u32>().expect("digits are not a u32");
    parser.with(n)
}

fn parse_list<'a, T>(
    parser: Parser<'a>,
    sep: &'static str,
    item_parser: fn(Parser<'a>) -> ParserResult<T>,
) -> ParserResult<'a, Vec<T>> {
    let mut nums: Vec<T> = Vec::new();
    let mut cur_parser = parser;
    loop {
        let (parser, num) = item_parser(cur_parser)?;
        nums.push(num);
        cur_parser = parser;
        let (parser, _) = skip_whitespace(cur_parser)?;
        if let Some((parser, _)) = exact(parser, sep) {
            let (parser, _) = skip_whitespace(parser)?;
            cur_parser = parser;
        } else {
            break;
        }
    }
    cur_parser.with(nums)
}

fn consume<T>(parser_result: ParserResult<T>) -> T {
    let (parser, result) = parser_result.expect("Parser failed");
    if parser.as_str().is_empty() {
        result
    } else {
        let tail = parser.as_str();
        panic!("Parser did not consume all input, remaining:\n{:?}", tail)
    }
}

// Parsers for Day 2

#[derive(Debug, Clone, Copy)]
enum Rgb {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy)]
struct ColorQty {
    color: Rgb,
    qty: u32,
}

struct Game {
    id: u32,
    sets: Vec<Vec<ColorQty>>,
}

fn parse_games(parser: Parser) -> ParserResult<Vec<Game>> {
    let mut games = Vec::new();
    let mut cur_parser = parser;
    while let Some((parser, game)) = parse_game(cur_parser) {
        games.push(game);
        let (parser, _) = skip_whitespace(parser)?;
        cur_parser = parser;
    }
    cur_parser.with(games)
}

fn parse_game(parser: Parser) -> ParserResult<Game> {
    let (parser, _) = exact(parser, "Game ")?;
    let (parser, id) = parse_u32(parser)?;
    let (parser, _) = exact(parser, ":")?;
    let (parser, sets) = parse_game_sets(parser)?;
    parser.with(Game { id, sets })
}

fn parse_game_sets(parser: Parser) -> ParserResult<Vec<Vec<ColorQty>>> {
    let (parser, _) = skip_whitespace(parser)?;
    let (parser, sets) = parse_list(parser, ";", parse_game_set)?;
    parser.with(sets)
}

fn parse_game_set(parser: Parser) -> ParserResult<Vec<ColorQty>> {
    let (parser, _) = skip_whitespace(parser)?;
    let (parser, game_set) = parse_list(parser, ",", parse_color_qty)?;
    parser.with(game_set)
}

fn parse_color_qty(parser: Parser) -> ParserResult<ColorQty> {
    let (parser, _) = skip_whitespace(parser)?;
    let (parser, qty) = parse_u32(parser)?;
    let (parser, _) = skip_whitespace(parser)?;
    let (parser, color) = parse_rgb(parser)?;
    parser.with(ColorQty { color, qty })
}

fn parse_rgb(parser: Parser) -> ParserResult<Rgb> {
    if let Some((parser, _)) = exact(parser, "red") {
        parser.with(Rgb::Red)
    } else if let Some((parser, _)) = exact(parser, "green") {
        parser.with(Rgb::Green)
    } else if let Some((parser, _)) = exact(parser, "blue") {
        parser.with(Rgb::Blue)
    } else {
        None
    }
}

struct MaxColors {
    r: u32,
    g: u32,
    b: u32,
}
impl MaxColors {
    fn get(&self, color: Rgb) -> u32 {
        match color {
            Rgb::Red => self.r,
            Rgb::Green => self.g,
            Rgb::Blue => self.b,
        }
    }
}

fn day_2_cube_conundrum() -> u32 {
    let body = DAY_2_INPUT_A;
    let max_colors = MaxColors {
        r: 12,
        g: 13,
        b: 14,
    };

    let games = consume(parse_games(Parser { body, pos: 0 }));

    let process_game = |game: &Game| {
        for set in &game.sets {
            for color_qty in set {
                if color_qty.qty > max_colors.get(color_qty.color) {
                    return None;
                }
            }
        }
        Some(game.id)
    };

    games.iter().filter_map(process_game).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 1 part 1: {}", day_1_trebuchet().unwrap());
    println!("Day 1 part 2: {}", day_1_trebuchet_part_2());
    println!("Day 2 part 1: {}", day_2_cube_conundrum());

    Ok(())
}
