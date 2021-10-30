use crate::util::PVec;
use std::convert::TryInto;


pub struct Grid {
    pub grid: Vec<char>,
    pub width: usize,
    pub height: usize,
    pub x0: usize,
    pub y0: usize,
}


// pub fn bc it's gonna go to the pub later
pub fn interpret(grid: Grid) -> i64 {
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