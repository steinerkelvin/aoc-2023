/*
--- Part Two ---
The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!

As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?

Again consider the example games from earlier:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
Game 4 required at least 14 red, 3 green, and 15 blue cubes.
Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.

For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?
*/

const DAY_2_INPUT_B: &str = include_str!("../../input/day_2_a.txt");

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

#[allow(dead_code)]
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

struct Colors {
    r: u32,
    g: u32,
    b: u32,
}
impl Colors {
    fn get(&self, color: Rgb) -> u32 {
        match color {
            Rgb::Red => self.r,
            Rgb::Green => self.g,
            Rgb::Blue => self.b,
        }
    }
    fn set(&mut self, color: Rgb, qty: u32) {
        match color {
            Rgb::Red => self.r = qty,
            Rgb::Green => self.g = qty,
            Rgb::Blue => self.b = qty,
        }
    }
}

fn day_2_cube_conundrum_part_2() -> u32 {
    let body = DAY_2_INPUT_B;

    let games = consume(parse_games(Parser { body, pos: 0 }));

    let process_game = |game: &Game| {
        let mut colors = Colors { r: 0, g: 0, b: 0 };
        for set in &game.sets {
            for color_qty in set {
                if color_qty.qty > colors.get(color_qty.color) {
                    colors.set(color_qty.color, color_qty.qty);
                }
            }
        }
        colors.r * colors.g * colors.b
    };

    games.iter().map(process_game).sum()
}

fn main() {
    let result = day_2_cube_conundrum_part_2();
    println!("Day 2 part 2 result: {}", result);
}
