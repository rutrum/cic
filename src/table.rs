use csv;

use std::io::{self, Write};

use crate::{Location, Cursor};

/// Contains the data read from a csv
/// Assumes nonzero columns and rows
#[derive(Debug)]
pub struct Table {
    data: Vec<Vec<String>>,
}

impl Table {
    /// Construct new table from the csv at the provided path.
    pub fn from_path(path: &str) -> Self {
        let mut rdr = csv::Reader::from_path(path).unwrap();
        let mut data: Vec<Vec<String>> = Vec::new();

        let headers: Vec<String> = rdr
            .headers()
            .unwrap()
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        data.push(headers);

        for record in rdr.into_records() {
            let strs = record
                .unwrap()
                .into_iter()
                .map(|x| x.to_owned())
                .collect();
            data.push(strs);
        }
        
        Table { data }
    }

    /// Returns the dimensions of the data within the table.
    /// Cache this value
    pub fn dims(&self) -> (usize, usize) {
        let dimy = self.data.len();
        let dimx = self.data[0].len();
        (dimx, dimy)
    }

    /// Calculates the max characters needed to display all values in a column.
    /// Needs rewrite to be more efficient (after removing headers)
    /// Also cache this value
    pub fn col_widths(&self) -> Vec<i32> {
        self.data.iter().fold(
            self.data[0].iter().map(|h| h.len()).map(|x| x as i32).collect::<Vec<i32>>(),
            |maxes, cur| {
                maxes
                    .iter()
                    .zip(cur.iter())
                    .map(|(m, c)| std::cmp::max::<i32>(*m, c.len() as i32))
                    .collect::<Vec<i32>>()
            },
        )
    }

    /// Add iterator for column widths along with row value?
    /// for (cell, width) in row.iter().zip(table.col_widths().iter()) {
    /// actually do fmt
    pub fn fmt_iter(&self) -> Vec<Vec<String>>{
        let widths = self.col_widths();
        self.data.iter().map(|row|
            row.iter().zip(widths.iter()).map(|(cell, width)| {
                let mut p = cell.clone();
                for _ in cell.len()..(*width as usize) {
                    p.push(' ');
                }
                p
            }).collect()
        ).collect()
    }

    /// Gets the internal data
    /// Should be formalized as iterators
    pub fn rows(&self) -> Vec<Vec<String>> {
        self.data.clone()
    }

    /// Gets a value in the table.
    pub fn get(&self, r: usize, c: usize) -> String {
        self.data[r][c].clone()
    }

    /// Updates a value in the table.
    pub fn update(&mut self, c: Cursor, new: String) {
        self.data[c.y][c.x] = new;
    }

    /// Adds new row before the given index
    pub fn add_row(&mut self, r: usize) {
        let (w, _) = self.dims();
        let new_row = vec![String::new(); w];
        self.data.insert(r, new_row);
    }

    /// Clears the value in the table.
    pub fn clear(&mut self, c: Cursor) {
        self.update(c, String::new());
    }

    pub fn delete_row(&mut self, c: &mut Cursor) {
        self.data.remove(c.y);
    }

    /// Writes the data as a csv to the given path.
    pub fn save_to_path(&self, path: String) {
        let mut wtr = csv::Writer::from_path(path).unwrap();
        for row in self.data.clone().into_iter() {
            wtr.write_record(row).unwrap();
        }
        wtr.flush().unwrap();
    }

    /// Prints the table for debug usage.
    pub fn print(&self) {
        let mut stdout = io::stdout();

        for row in self.data.iter() {
            for item in row {
                write!(stdout, "{:<14.12}", item).unwrap();
            }
            writeln!(stdout).unwrap();
            stdout.flush().unwrap();
        }
    }
}
