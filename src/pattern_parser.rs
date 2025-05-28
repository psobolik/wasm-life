
use crate::cell::Cell;
use crate::pattern::Pattern;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PatternParser;

#[wasm_bindgen]
impl PatternParser {
    pub fn parse_cells_data(data: &str) -> Pattern {
        fn parse_line(line: &str, row: i32) -> Vec<Cell> {
            let mut cells: Vec<Cell> = vec![];
            for (col, ch) in line.chars().enumerate() {
                if ch != '.' { 
                    cells.push(Cell::new(row as u32, col as u32))
                }
            }
            cells
        }
        let mut metadata: Vec<String> = vec![];
        let mut cells: Vec<Cell> = vec![];
        
        let mut row = 0;
        
        for line in data.lines() {
            if line.starts_with('!') {
                metadata.push(line.to_string());
            } else {
                cells.append(&mut parse_line(line, row));
                row += 1;
            }
        }
        Pattern::new(metadata, cells)
    }
    pub fn parse_rle_data(data: &str) -> Pattern {
        let mut metadata: Vec<String> = vec![];
        let mut cells: Vec<Cell> = vec![];
        
        let mut header_found = false;
        let mut row = 0;
        let mut col = 0;
        let mut count = 0;
        
        for line in data.lines() {
            if line.starts_with('#') {
                metadata.push(line.to_string());
            } else if !header_found {
                // The first line that doesn't start with "#" is the header,
                // which we include in the metadata
                metadata.push(line.to_string());
                header_found = true;
            } else {
                for ch in line.to_lowercase().chars() {
                    if ch == '!' { break; }
                    if char::is_digit(ch, 10) {
                        count = (count * 10) + ch.to_digit(10).unwrap();
                    } else {
                        match ch {
                            'o' | 'x' | 'y' | 'z' => {
                                if count == 0 {
                                    cells.push(Cell::new(row as u32, col as u32));
                                    col += 1;
                                } else {
                                    while count > 0 {
                                        cells.push(Cell::new(row as u32, col as u32));
                                        col += 1;
                                        count -= 1;
                                    }
                                }
                            }
                            'b' => {
                                if count == 0 {
                                    col += 1;
                                } else {
                                    col += count;
                                    count = 0;
                                }
                            }
                            '$' => {
                                col = 0;
                                if count == 0 {
                                    row += 1;
                                } else {
                                    row += count;
                                    count = 0;
                                }
                            }
                            _ => { /* ignore unexpected character */}
                        }
                    }
                }
            }
        }
        Pattern::new(metadata, cells)
    }
}