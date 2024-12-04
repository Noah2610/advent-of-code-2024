use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
struct AppError {
    message: String,
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            message: format!("[IO Error] {error}"),
        }
    }
}

type Grid = Vec<Vec<char>>;

struct WordSearch {
    grid: Grid,
}

impl From<String> for WordSearch {
    fn from(s: String) -> Self {
        let mut grid = Vec::new();

        for line in s.lines() {
            let mut row = Vec::new();

            for c in line.chars() {
                row.push(c);
            }

            if row.len() > 0 {
                grid.push(row);
            }
        }

        Self { grid }
    }
}

impl WordSearch {
    pub fn iter(
        &self,
        word_len: usize,
        direction: Direction,
    ) -> WordSearchIter {
        WordSearchIter::new(&self.grid, direction, word_len)
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Horizontal,
    HorizontalReverse,
    Vertical,
    VerticalReverse,
    DiagonalTLBR,
    DiagonalTLBRReverse,
    DiagonalBLTR,
    DiagonalBLTRReverse,
}

struct WordSearchIter<'a> {
    grid: &'a Grid,
    direction: Direction,
    word_len: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    start_x: usize,
    start_y: usize,
    is_done: bool,
}

impl<'a> WordSearchIter<'a> {
    pub fn new(grid: &'a Grid, direction: Direction, word_len: usize) -> Self {
        use Direction::*;

        let w = grid[0].len();
        let h = grid.len();

        let tl = (0, 0);
        let tr = (w - 1, 0);
        let bl = (0, h - 1);
        let br = (w - 1, h - 1);

        let (x, y) = match direction {
            Horizontal => tl,
            HorizontalReverse => tr,
            Vertical => tl,
            VerticalReverse => bl,
            DiagonalTLBR => tr,
            DiagonalTLBRReverse => tr,
            DiagonalBLTR => tl,
            DiagonalBLTRReverse => tl,
        };

        Self {
            grid,
            direction,
            word_len,
            x,
            y,
            w,
            h,
            start_x: x,
            start_y: y,
            is_done: false,
        }
    }

    fn get_next_pos(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        use Direction::*;

        match self.direction {
            Horizontal => {
                if x + 1 == self.w {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            HorizontalReverse => x.checked_sub(1).map(|x| (x, y)),
            Vertical => {
                if y + 1 == self.h {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            VerticalReverse => y.checked_sub(1).map(|y| (x, y)),
            DiagonalTLBR => {
                if x + 1 == self.w || y + 1 == self.h {
                    None
                } else {
                    Some((x + 1, y + 1))
                }
            }
            DiagonalTLBRReverse => {
                x.checked_sub(1).zip(y.checked_sub(1)).map(|(x, y)| (x, y))
            }
            DiagonalBLTR => {
                if x + 1 == self.w {
                    None
                } else {
                    y.checked_sub(1).map(|y| (x + 1, y))
                }
            }
            DiagonalBLTRReverse => {
                if y + 1 == self.h {
                    None
                } else {
                    x.checked_sub(1).map(|x| (x, y + 1))
                }
            }
        }
    }

    fn wrap(&self) -> Option<(usize, usize)> {
        use Direction::*;

        match self.direction {
            Horizontal => {
                if self.start_y + 1 == self.h {
                    None
                } else {
                    Some((self.start_x, self.start_y + 1))
                }
            }
            HorizontalReverse => {
                if self.start_y + 1 == self.h {
                    None
                } else {
                    Some((self.start_x, self.start_y + 1))
                }
            }
            Vertical => {
                if self.start_x + 1 == self.w {
                    None
                } else {
                    Some((self.start_x + 1, self.start_y))
                }
            }
            VerticalReverse => {
                if self.start_x + 1 == self.w {
                    None
                } else {
                    Some((self.start_x + 1, self.start_y))
                }
            }
            DiagonalTLBR => {
                if self.start_x == 0 && self.start_y + 1 == self.h {
                    None
                } else {
                    Some((
                        (self.start_x).checked_sub(1).unwrap_or(0),
                        if self.start_x > 0 {
                            self.start_y
                        } else {
                            self.start_y + 1
                        },
                    ))
                }
            }
            DiagonalTLBRReverse => {
                if self.start_x == 0 && self.start_y + 1 == self.h {
                    None
                } else {
                    Some((
                        if self.start_y + 1 < self.h {
                            self.start_x
                        } else {
                            self.start_x.checked_sub(1).unwrap_or(0)
                        },
                        (self.start_y + 1).min(self.h - 1),
                    ))
                }
            }
            DiagonalBLTR => {
                if self.start_x + 1 == self.w && self.start_y + 1 == self.h {
                    None
                } else {
                    Some((
                        if self.start_y + 1 < self.h {
                            self.start_x
                        } else {
                            self.start_x + 1
                        },
                        (self.start_y + 1).min(self.h - 1),
                    ))
                }
            }
            DiagonalBLTRReverse => {
                if self.start_x + 1 == self.w && self.start_y + 1 == self.h {
                    None
                } else {
                    Some((
                        (self.start_x + 1).min(self.w - 1),
                        if self.start_x + 1 < self.w {
                            self.start_y
                        } else {
                            self.start_y + 1
                        },
                    ))
                }
            }
        }
    }
}

struct WordSearchIterItem {
    pub word: String,
    pub positions: Vec<(usize, usize)>,
}

impl<'a> Iterator for WordSearchIter<'a> {
    type Item = WordSearchIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let mut word = String::new();
        let mut positions = Vec::new();

        let mut x = self.x;
        let mut y = self.y;

        while word.len() < self.word_len {
            word.push(self.grid[y][x]);
            positions.push((x, y));

            if word.len() == self.word_len {
                break;
            }

            let next_pos = self.get_next_pos(x, y);

            if let Some(next_pos) = next_pos {
                x = next_pos.0;
                y = next_pos.1;
            } else {
                word.clear();
                positions.clear();

                let wrapped_pos = self.wrap();
                if let Some(wrapped_pos) = wrapped_pos {
                    x = wrapped_pos.0;
                    y = wrapped_pos.1;
                    self.x = x;
                    self.y = y;
                    self.start_x = x;
                    self.start_y = y;
                } else {
                    self.is_done = true;
                    break;
                }
            }
        }

        if self.is_done {
            return None;
        }

        let next_pos = self.get_next_pos(self.x, self.y);
        if let Some((x, y)) = next_pos {
            self.x = x;
            self.y = y;
        } else {
            let wrapped_pos = self.wrap();
            if let Some((x, y)) = wrapped_pos {
                self.x = x;
                self.y = y;
                self.start_x = x;
                self.start_y = y;
            } else {
                self.is_done = true;
            }
        }

        Some(WordSearchIterItem { word, positions })
    }
}

fn count_words(iter: WordSearchIter, word: &str) -> usize {
    iter.map(|item| if item.word == word { 1 } else { 0 }).sum()
}

fn sum_words(word_search: &WordSearch, word: &str) -> usize {
    [
        Direction::Horizontal,
        Direction::HorizontalReverse,
        Direction::Vertical,
        Direction::VerticalReverse,
        Direction::DiagonalTLBR,
        Direction::DiagonalTLBRReverse,
        Direction::DiagonalBLTR,
        Direction::DiagonalBLTRReverse,
    ]
    .into_iter()
    .map(|dir| {
        let count = count_words(word_search.iter(word.len(), dir), word);
        println!("{:?}: {}", dir, count);
        count
    })
    .sum()
}

fn count_word_crosses(word_search: &WordSearch, word: &str) -> usize {
    let mut center_position_counts = HashMap::<(usize, usize), usize>::new();

    let center_idx = word.len() / 2;

    [
        Direction::DiagonalTLBR,
        Direction::DiagonalTLBRReverse,
        Direction::DiagonalBLTR,
        Direction::DiagonalBLTRReverse,
    ]
    .into_iter()
    .flat_map(|dir| {
        word_search
            .iter(word.len(), dir)
            .filter(|item| item.word == word)
    })
    .for_each(|item| {
        let center = item.positions.get(center_idx);
        if let Some(pos) = center {
            center_position_counts
                .entry(*pos)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    });

    center_position_counts
        .into_values()
        .filter(|value| *value > 1)
        .count()
}

fn main() -> AppResult<()> {
    let input = read_input()?;

    let word_search = WordSearch::from(input);

    {
        let sum = sum_words(&word_search, "XMAS");
        println!("Sum: {}", sum);
    }

    {
        let sum = count_word_crosses(&word_search, "MAS");
        println!("Crosses: {}", sum);
    }

    Ok(())
}

fn read_input() -> io::Result<String> {
    let is_dev = env::var("DEV").is_ok();
    let file_name = if is_dev { "dev-input.txt" } else { "input.txt" };

    let project_dir = get_project_dir()?;
    let input_file = project_dir.join(file_name);

    fs::read_to_string(input_file)
}

// https://github.com/neilwashere/rust-project-root/blob/main/src/lib.rs
fn get_project_dir() -> io::Result<PathBuf> {
    let current_exe = env::current_exe()?;
    let path = current_exe.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to find parent directory of current executable",
        )
    })?;
    let mut path_ancestors = path.ancestors();

    while let Some(parent) = path_ancestors.next() {
        dbg!(parent);
        let has_cargo = fs::read_dir(parent)?.into_iter().any(|entry_res| {
            entry_res
                .map(|entry| {
                    entry.file_name() == std::ffi::OsString::from("Cargo.toml")
                })
                .unwrap_or(false)
        });
        if has_cargo {
            return Ok(PathBuf::from(parent));
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Failed to find Cargo.toml",
    ))
}
