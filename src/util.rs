use crate::pool::Grid;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::{Add, AddAssign, Index, MulAssign};

#[derive(Clone, Copy)]
pub struct PVec {
    pub x: i64,
    pub y: i64,
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

impl PartialEq for PVec {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
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

pub fn read_file(filename: &str) -> Result<Grid, io::Error> {
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
