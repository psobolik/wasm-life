use wasm_bindgen::prelude::wasm_bindgen;
use crate::cell::Cell;

#[wasm_bindgen]
pub struct Pattern {
    metadata: Vec<String>,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Pattern {
    pub fn new(metadata: Vec<String>, cells: Vec<Cell>) -> Self {
        Self { metadata, cells }
    }
    pub fn cells(&self) -> Vec<Cell> {
        self.cells.clone()
    }
    pub fn metadata(&self) -> Vec<String> {
        self.metadata.clone()
    }
    pub fn name(&self) -> Option<String> {
        let mut name: Option<String> = None;
        
        if !self.metadata.is_empty() {
            // If there's metadata, look for a name specification
            for line in self.metadata.iter() {
                if line.starts_with("!Name:") {
                    name = Some(line.trim_start_matches("!Name:").trim().to_owned());
                    break;
                } else if line.starts_with("#N") {
                    name = Some(line.trim_start_matches("#N").trim().to_owned());
                    break;
                }
            }
            // Fallback to first line if no name specified in metadata
            if let Some(first_line) = self.metadata.first() {
                if first_line.starts_with("!") || first_line.starts_with("#") {
                    name = Some(first_line[1..].trim().to_owned());
                }
            }
        }
        name
    }
    pub fn dimensions(&self) -> Cell {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        for cell in self.cells.iter() {
            if cell.row() > height { height = cell.row(); }
            if cell.col() > width { width = cell.col(); }
        }
        Cell::new(width, height)
    }
}