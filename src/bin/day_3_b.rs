/** Day 3: Gear Ratios (Part 2)

The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?

*/

struct Matrix<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    fn new(width: usize, height: usize, fill: T) -> Self {
        Self {
            width,
            height,
            data: vec![fill; width * height],
        }
    }
}

fn entry<T: Clone>(matrix: &Matrix<T>, x: i32, y: i32) -> Option<&T> {
    if x < 0 || y < 0 {
        return None;
    }
    let x = x as usize;
    let y = y as usize;
    if x >= matrix.width || y >= matrix.height {
        return None;
    }
    Some(&matrix.data[x + y * matrix.width])
}

fn day_3_gear_ratios_part_2(input: &str) -> u32 {
    let lines = input.lines();
    let lines: Vec<&[u8]> = lines.map(|line| line.as_bytes()).collect();

    let width = lines[0].len();
    let height = lines.len();

    type Gear = Vec<u32>;
    let mut gear_matrix = Matrix::<Option<Gear>>::new(width, height, None);

    for (y, line) in lines.iter().enumerate() {
        for (x, &byte) in line.iter().enumerate() {
            if byte == b'*' {
                gear_matrix.data[x + y * width] = Some(Gear::new());
            }
        }
    }

    for (y, line) in lines.iter().enumerate() {
        let mut current_number: Option<u32> = None;
        let mut current_gears = std::collections::HashSet::new();
        for (x, &byte) in line.iter().enumerate() {
            if byte.is_ascii_digit() {
                let d = (byte - b'0') as u32;
                current_number = current_number.map(|n| n * 10 + d).or(Some(d));
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let gear_x = x as i32 + dx;
                        let gear_y = y as i32 + dy;
                        if let Some(Some(_)) = entry(&gear_matrix, gear_x, gear_y) {
                            current_gears.insert((gear_x as usize, gear_y as usize));
                        }
                    }
                }
            } else if let Some(number) = current_number {
                for &(x, y) in &current_gears {
                    gear_matrix.data[x + y * width]
                        .as_mut()
                        .unwrap()
                        .push(number);
                }
                current_number = None;
                current_gears.clear();
            }
        }
        if let Some(number) = current_number {
            for &(x, y) in &current_gears {
                gear_matrix.data[x + y * width]
                    .as_mut()
                    .unwrap()
                    .push(number);
            }
        }
    }

    let mut sum = 0;
    for gear in gear_matrix.data.iter().flatten() {
        if gear.len() == 2 {
            sum += gear[0] * gear[1];
        }
    }
    sum
}

fn main() {
    let input = include_str!("../../input/day_3_b.txt");
    let result = day_3_gear_ratios_part_2(input);
    println!("Day 3 part 2 result: {}", result);
}
