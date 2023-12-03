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

fn entry<T: Clone>(matrix: &mut Matrix<T>, x: i32, y: i32) -> Option<&mut T> {
    if x < 0 || y < 0 {
        return None;
    }
    let x = x as usize;
    let y = y as usize;
    if x >= matrix.width || y >= matrix.height {
        return None;
    }
    Some(&mut matrix.data[x + y * matrix.width])
}

fn day_3_gear_ratios(input: &str) -> u32 {
    let lines = input.lines();
    let lines: Vec<&[u8]> = lines.map(|line| line.as_bytes()).collect();

    let width = lines[0].len();
    let height = lines.len();
    let mut flag_matrix = Matrix::<bool>::new(width, height, false);

    for (y, line) in lines.iter().enumerate() {
        for (x, &byte) in line.iter().enumerate() {
            if !byte.is_ascii_digit() && byte != b'.' {
                // Mark adjacent cells
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if let Some(entry) = entry(&mut flag_matrix, x as i32 + dx, y as i32 + dy) {
                            *entry = true;
                        }
                    }
                }
            }
        }
    }

    let mut is_symbol_adjacent = |x: i32, y: i32| {
        if let Some(true) = entry(&mut flag_matrix, x, y) {
            return true;
        }
        false
    };

    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        let mut current_number: Option<u32> = None;
        let mut is_part = false;
        for (x, &byte) in line.iter().enumerate() {
            if byte.is_ascii_digit() {
                let d = (byte - b'0') as u32;
                current_number = current_number.map(|n| n * 10 + d).or(Some(d));
                if !is_part && is_symbol_adjacent(x as i32, y as i32) {
                    is_part = true;
                }
            } else {
                if let (Some(number), true) = (current_number, is_part) {
                    sum += number;
                }
                current_number = None;
                is_part = false;
            }
        }
        if let (Some(number), true) = (current_number, is_part) {
            sum += number;
        }
    }

    sum
}

fn main() {
    let input = include_str!("../../input/day_3_a.txt");
    let result = day_3_gear_ratios(input);
    println!("Day 3 part 1 result: {}", result);
}
