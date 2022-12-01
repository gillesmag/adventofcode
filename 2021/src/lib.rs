use std::env;
use std::fs;

pub fn read_file(directory: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("src")
        .join(directory)
        .join(format!("day{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

pub struct Grid {
    pub items: Vec<Vec<u32>>,
    include_diagonals: bool,
}

impl Grid {
    pub fn new(items: Vec<Vec<u32>>) -> Grid {
        Grid {
            items,
            include_diagonals: false,
        }
    }

    pub fn with_diagonals(items: Vec<Vec<u32>>) -> Grid {
        Grid {
            items,
            include_diagonals: true,
        }
    }

    // Returns (columns, rows)
    pub fn bounds(&self) -> (usize, usize) {
        (self.items[0].len(), self.items.len())
    }

    pub fn at(&self, x: usize, y: usize) -> u32 {
        self.items[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u32) {
        self.items[y][x] = value;
    }

    pub fn adjacent_positions(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut positions = vec![];

        // Left
        if x >= 1 {
            // Top Left (diagonal)
            if y >= 1 && self.include_diagonals {
                positions.push((x - 1, y - 1));
            }

            // Middle Left
            positions.push((x - 1, y));

            // Bottom Left (diagonal)
            if y + 1 < self.bounds().1 && self.include_diagonals {
                positions.push((x - 1, y + 1));
            }
        }

        // Right
        if x + 1 < self.bounds().0 {
            // Top Right (digonal)
            if y >= 1 && self.include_diagonals {
                positions.push((x + 1, y - 1));
            }

            // Middle Right
            positions.push((x + 1, y));

            // Bottom Right (diagonal)
            if y + 1 < self.bounds().1 && self.include_diagonals {
                positions.push((x + 1, y + 1));
            }
        }

        if y >= 1 {
            positions.push((x, y - 1));
        }

        if y + 1 < self.bounds().1 {
            positions.push((x, y + 1));
        }

        positions
    }

    pub fn print(&self) {
        let (rows, columns) = self.bounds();

        for y in 0..rows {
            for x in 0..columns {
                //if x > 0 {
                //    print!(" ");
                //}
                let val = self.at(x, y).to_string();
                if val == "0" {
                    print!("{}", val);
                } else {
                    print!("{}", val);
                }
            }
            println!("");
        }
    }
}
