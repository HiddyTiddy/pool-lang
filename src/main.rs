use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::{Add, AddAssign, Index, MulAssign};
struct Grid {
    grid: Vec<char>,
    width: usize,
    height: usize,
    x0: usize,
    y0: usize,
}

fn read_file(filename: &str) -> Result<Grid, io::Error> {
    let file = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut src: String = String::new();
    let mut height: usize = 0;
    let mut max_width: usize = 0;
    let mut width: usize = 0;
    let mut x0: Option<usize> = None;
    let mut y0: Option<usize> = None;
    reader
        .lines()
        .enumerate()
        .into_iter()
        .for_each(|(i, line)| {
            height += 1;
            let l = line.unwrap();
            for ch in l.chars() {
                width += 1;
                if !matches!(x0, Some(_)) && ch == '.' {
                    x0 = Some(i);
                    y0 = Some(height - 1);
                }
            }
            if width > max_width {
                max_width = width;
            }
            width = 0;
            src = format!("{}{}\n", src, l);
        });
    max_width += 1;

    let mut grid: Vec<char> = vec![' '; height * max_width];

    let mut i = 0;
    let mut j = 0;
    for ch in src.chars() {
        grid[j + max_width * i] = ch;
        j += 1;
        if ch == '\n' {
            j = 0;
            i += 1;
        }
    }

    if let Some(x) = x0 {
        Ok(Grid {
            width: max_width,
            height,
            grid,
            x0: x,
            y0: y0.expect("no starting point found"),
        })
    } else {
        Ok(Grid {
            width: max_width,
            height,
            grid,
            x0: 0,
            y0: 0,
        })
    }
}

#[derive(Clone, Copy)]
struct PVec {
    x: i64,
    y: i64,
}

impl Add for PVec {
    type Output = PVec;
    fn add(self, rhs: Self) -> Self::Output {
        PVec {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for PVec {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl MulAssign<i64> for PVec {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Index<i64> for Grid {
    type Output = char;

    /* this is safe... */
    fn index(&self, index: i64) -> &Self::Output {
        &self.grid[index as usize]
    }
}

fn interpret(grid: Grid) -> i64 {
    let mut ptr: PVec = PVec {
        x: grid.x0 as i64,
        y: grid.y0 as i64,
    };
    let mut vel: PVec = PVec { x: 1, y: 0 };
    let mut in_str: bool = false;
    let mut escaped: bool = false;
    let mut stack: Vec<u64> = vec![];

    const HEAP_SIZE: usize = 1024;
    let mut heap: [u64; HEAP_SIZE] = [0; HEAP_SIZE];

    loop {
        ptr += vel;
        if ptr.x >= grid.width as i64 || ptr.x < 0 || ptr.y >= grid.height as i64 || ptr.y < 0 {
            panic!("attempted to go out of grid with x={} y={}", ptr.x, ptr.y);
        }
        let ch = grid[ptr.x + ptr.y * grid.width as i64];
        //println!("{} {:?} {:?}", ch, stack, &heap[..10]);
        // println!("{} {:?}", ch, stack);
        if in_str {
            if escaped {
                match ch {
                    '\\' => stack.push('\\' as u64),
                    'n' => stack.push('\n' as u64),
                    't' => stack.push('\t' as u64),
                    'r' => stack.push('\r' as u64),
                    '"' => stack.push('"' as u64),
                    _ => stack.push(ch as u64),
                }
                escaped = false;
                continue;
            } else if ch == '"' {
                in_str = false;
            } else if ch == '\\' {
                escaped = true;
            } else {
                stack.push(ch as u64);
            }
        } else {
            match ch {
                'v' => {
                    if vel.y == 0 {
                        vel = PVec { x: 0, y: 1 }
                    }
                }
                '<' => {
                    if vel.x == 0 {
                        vel = PVec { x: -1, y: 0 }
                    }
                }
                '^' => {
                    if vel.y == 0 {
                        vel = PVec { x: 0, y: -1 }
                    }
                }
                '>' => {
                    if vel.x == 0 {
                        vel = PVec { x: 1, y: 0 }
                    }
                }
                '+' => {
                    let a = stack.pop().expect("empty stack");
                    let b = stack.pop().expect("empty stack");
                    stack.push(b + a);
                }
                '-' => {
                    let a = stack.pop().expect("empty stack");
                    let b = stack.pop().expect("empty stack");
                    stack.push(b - a);
                }
                '*' => {
                    let a = stack.pop().expect("empty stack");
                    let b = stack.pop().expect("empty stack");
                    stack.push(b * a);
                }
                '/' => {
                    let a = stack.pop().expect("empty stack");
                    let b = stack.pop().expect("empty stack");
                    stack.push(b / a);
                }
                '%' => {
                    let a = stack.pop().expect("empty stack");
                    let b = stack.pop().expect("empty stack");
                    stack.push(b % a);
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a' | 'b' | 'c'
                | 'd' | 'e' | 'f' => {
                    stack.push(ch.to_digit(16).expect("invalid literal") as u64);
                }
                ',' => {
                    let a = stack.pop().expect("empty stack");
                    print!("{}", (a & 0xff) as u8 as char);
                    print!("{}", (a >> 8 & 0xff) as u8 as char);
                    print!("{}", (a >> 16 & 0xff) as u8 as char);
                    print!("{}", (a >> 24 & 0xff) as u8 as char);
                    print!("{}", (a >> 32 & 0xff) as u8 as char);
                    print!("{}", (a >> 40 & 0xff) as u8 as char);
                    print!("{}", (a >> 48 & 0xff) as u8 as char);
                    print!("{}", (a >> 56 & 0xff) as u8 as char);
                }
                '"' => {
                    in_str = true;
                }
                ';' => {
                    let exit_code = stack.pop().expect("empty stack");
                    return exit_code as i64;
                }
                '!' => {
                    let a = stack.pop().expect("empty stack");
                    stack.push(if a == 0 { 1 } else { 0 });
                }
                '`' => {
                    let a = stack.pop().expect("empty stack");
                    let b = stack.pop().expect("empty stack");
                    stack.push(if b > a { 1 } else { 0 });
                }
                ':' => {
                    stack.push(*stack.last().expect("empty stack"));
                }
                '$' => {
                    stack.pop();
                }
                '&' => {
                    let a = stack.pop().expect("empty stack");
                    let b = stack.pop().expect("empty stack");
                    stack.push(a);
                    stack.push(b);
                }
                '|' => {
                    if vel.x != 0 {
                        let condition = stack.pop().expect("empty stack");
                        if condition != 0 {
                            vel *= -1;
                        }
                    }
                }
                '_' => {
                    if vel.y != 0 {
                        let condition = stack.pop().expect("empty stack");
                        if condition != 0 {
                            vel *= -1;
                        }
                    }
                }
                's' => {
                    let address = stack.pop().expect("empty stack");
                    let value = stack.pop().expect("empty stack");
                    if address < HEAP_SIZE.try_into().unwrap() {
                        heap[address as usize] = value;
                    } else {
                        panic!("out of bounds");
                    }
                }
                'r' => {
                    let address = stack.pop().expect("empty stack");
                    if address < HEAP_SIZE.try_into().unwrap() {
                        stack.push(heap[address as usize]);
                    } else {
                        panic!("out of bounds");
                    }
                }
                '√' | 'n' => {
                    let a = stack.pop().unwrap() as f64;
                    stack.push(a.sqrt() as u64);
                }
                'o' => {
                    let a = stack.pop().expect("empty stack");
                    let b = stack.pop().expect("empty stack");
                    stack.push(b);
                    stack.push(a);
                    stack.push(b);
                }
                _ => (),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("missing arguments");
        return;
    }
    let filename = &args[1];
    let grid = read_file(filename).expect("file not found");
    interpret(grid);
}
