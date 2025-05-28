use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Cell {
    row: u32,
    col: u32,
}

#[wasm_bindgen]
impl Cell {
    pub fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }
    pub fn row(&self) -> u32 {
        self.row
    }
    pub fn col(&self) -> u32 {
        self.col
    }
}