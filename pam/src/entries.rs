use std::fs::File;

use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::path::Path;

pub struct Entries<T> {
    cursor: BufReader<File>,
    marker: PhantomData<T>,
}

impl<T> Entries<T> {
    pub fn new(file: &Path) -> Entries<T> {
        let inner = if !Path::new(file).exists() {
            File::create(file).ok().unwrap()
        } else {
            File::open(file).ok().unwrap()
        };

        let reader = BufReader::new(inner);
        Entries {
            cursor: reader,
            marker: PhantomData,
        }
    }
}

impl<T: Entry> Iterator for Entries<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut line = String::new();
        loop {
            line.clear();
            match self.cursor.read_line(&mut line) {
                Ok(0) => return None,
                Ok(_) => (),
                _ => return None,
            }

            if line.starts_with("#") {
                continue;
            }

            match T::from_line(&line) {
                Ok(entry) => return Some(entry),

                _ => (),
            }
        }
    }
}

pub trait Entry: Sized {
    fn from_line(line: &str) -> Result<Self, ParseIntError>;
    fn to_line(&self) -> String;
}
