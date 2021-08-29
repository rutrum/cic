use csv;

use std::io::{self, Write};

use crate::{Cursor};

/// Contains the data read from a csv
/// Assumes nonzero columns and rows
#[derive(Debug)]
pub struct Table {
    data: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            data: vec![vec![String::new()]],
        }
    }

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
    pub fn get(&self, c: Cursor) -> String {
        self.data[c.y][c.x].clone()
    }

    /// Updates a value in the table.
    pub fn update(&mut self, c: Cursor, new: String) {
        self.data[c.y][c.x] = new;
    }

    /// Adds row before the cursor location.
    pub fn add_row_before(&mut self, c: Cursor) {
        self.add_row(c.y);
    }

    /// Adds row after the cursor location.
    pub fn add_row_after(&mut self, c: Cursor) {
        self.add_row(c.y + 1);
    }

    /// Adds new row before the given index.
    fn add_row(&mut self, r: usize) {
        let (w, _) = self.dims();
        let new_row = vec![String::new(); w];
        self.data.insert(r, new_row);
    }

    /// Adds column before the cursor location.
    pub fn add_col_before(&mut self, c: Cursor) {
        self.add_col(c.x);
    }

    /// Adds column after the cursor location.
    pub fn add_col_after(&mut self, c: Cursor) {
        self.add_col(c.x + 1);
    }

    /// Inserts new column at index
    fn add_col(&mut self, c: usize) {
        for row in &mut self.data {
            row.insert(c, String::new());
        }
    }

    /// Clears the value in the table.
    pub fn clear(&mut self, c: Cursor) {
        self.update(c, String::new());
    }

    /// Deletes the row the cursor lies.
    pub fn delete_row(&mut self, c: &mut Cursor) {
        self.data.remove(c.y);
    }

    /// Deletes the column the cursor lies.
    pub fn delete_col(&mut self, c: &mut Cursor) {
        for row in &mut self.data {
            row.remove(c.x);
        }
    }

    /// Writes the data as a csv to the given path.
    pub fn save_to_path(&self, path: String) {
        let mut wtr = csv::Writer::from_path(path).unwrap();
        for row in self.data.clone().into_iter() {
            wtr.write_record(row).unwrap();
        }
        wtr.flush().unwrap();
    }
}
