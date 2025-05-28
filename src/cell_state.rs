use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellState {
    Populated,
    Vacant,
    Invalid,
}
