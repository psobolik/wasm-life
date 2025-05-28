use crate::cell::Cell;
use crate::cell_state::CellState;
use crate::consts::Constants;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LifeGrid {
    cell_states: Vec<CellState>,
    cell_count: u32, // Number of rows and columns of cells
    grid_size: f64,  // The width and height of the grid in pixels
    context: web_sys::CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl LifeGrid {
    pub fn new(grid_size: u32, cell_count: u32, canvas_id: &str) -> Self {
        crate::utils::set_panic_hook();
        let canvas_element = Self::get_canvas_element(canvas_id);
        canvas_element.set_height(grid_size);
        canvas_element.set_width(grid_size);

        let context = Self::get_canvas_rendering_context_2d(&canvas_element);
        let cell_states = (0..cell_count * cell_count)
            .map(|_i| CellState::Vacant)
            .collect();
        Self {
            cell_states,
            cell_count,
            grid_size: grid_size as f64,
            context,
        }
    }
    pub fn draw(&self) {
        self.draw_grid();
    }
    pub fn cell_state(&self, row: u32, col: u32) -> CellState {
        if self.in_grid(row as i32, col as i32) {
            self.cell_states[self.cell_index(row, col)].clone()
        } else {
            CellState::Invalid
        }
    }
    pub fn set_cell_state(&mut self, row: u32, col: u32, state: CellState) {
        if self.in_grid(row as i32, col as i32) {
            let index = self.cell_index(row, col);
            self.cell_states[index] = state;
        }
    }
    pub fn toggle_cell_state(&mut self, row: u32, col: u32) {
        let new_state = match self.cell_state(row, col) {
            CellState::Populated => CellState::Vacant,
            _ => CellState::Populated,
        };
        self.set_cell_state(row, col, new_state);
    }
    pub fn cell_from_point(&self, x: f64, y: f64) -> Cell {
        Cell::new(
            ((y - Constants::BORDER_WIDTH) / self.cell_size()) as u32,
            ((x - Constants::BORDER_WIDTH) / self.cell_size()) as u32,
        )
    }
    pub fn vacate_all_cells(&mut self) {
        (0..self.cell_count * self.cell_count).for_each(|index| {
            self.cell_states[index as usize] = CellState::Vacant;
        })
    }
    pub fn evolve(&mut self) {
        let mut next_generation: Vec<Cell> = vec![];
        (0..self.cell_count).for_each(|row| {
            (0..self.cell_count).for_each(|col| {
                let state = self.cell_state(row, col);
                let neighbors = self.count_neighbors(row, col);
                if state == CellState::Populated && (neighbors == 2 || neighbors == 3) {
                    next_generation.push(Cell::new(row, col));
                } else if state == CellState::Vacant && neighbors == 3 {
                    next_generation.push(Cell::new(row, col));
                }
            })
        });
        self.set_populated_cells(next_generation);
    }
    pub fn rotate_clockwise(&mut self) {
        let mut rotated: Vec<Cell> = vec![];

        let lower_bounds = self.lower_bounds();
        let upper_bounds = self.upper_bounds();

        let height = upper_bounds.row() - lower_bounds.row() + 1;
        let width = upper_bounds.col() - lower_bounds.col() + 1;

        let size = if width > height { Self::make_odd(width) } else { Self::make_odd(height) };
        let min_row = lower_bounds.row() - (size - height) / 2;
        let min_col = lower_bounds.col() - (size - width) / 2;

        (0..self.cell_count).for_each(|row| {
            (0..self.cell_count).for_each(|col| {
                if self.cell_state(min_row + row, min_col + col) == CellState::Populated {
                    rotated.push(Cell::new(min_row + col, min_col + (size - 1 - row)));
                }
            })
        });
        self.set_populated_cells(rotated);
    }
    pub fn rotate_counter_clockwise(&mut self) {
        let mut rotated: Vec<Cell> = vec![];

        let lower_bounds = self.lower_bounds();
        let upper_bounds = self.upper_bounds();

        let height = upper_bounds.row() - lower_bounds.row() + 1;
        let width = upper_bounds.col() - lower_bounds.col() + 1;

        let size = if width > height { Self::make_odd(width) } else { Self::make_odd(height) };
        let min_row = lower_bounds.row() - (size - height) / 2;
        let min_col = lower_bounds.col() - (size - width) / 2;

        (0..self.cell_count).for_each(|row| {
            (0..self.cell_count).for_each(|col| {
                if self.cell_state(min_row + row, min_col + col) == CellState::Populated {
                    rotated.push(Cell::new(min_row + (size - 1 - col), min_col + row));
                }
            })
        });
        self.set_populated_cells(rotated);
    }
    pub fn flip_horizontal(&mut self) {
        let mut flipped: Vec<Cell> = vec![];
        
        let lower_bounds = self.lower_bounds();
        let upper_bounds = self.upper_bounds();
        
        let max_row_index = upper_bounds.row() - lower_bounds.row();
        
        (0..=max_row_index).for_each(|row| {
            (lower_bounds.col()..=upper_bounds.col()).for_each(|col| {
                if self.cell_state(lower_bounds.row() + row, col) == CellState::Populated {
                    flipped.push(Cell::new(lower_bounds.row() + max_row_index - row, col));
                }
            })
        });
        self.set_populated_cells(flipped);
    }
    pub fn flip_vertical(&mut self) {
        let mut flipped: Vec<Cell> = vec![];

        let lower_bounds = self.lower_bounds();
        let upper_bounds = self.upper_bounds();

        let max_col_index = upper_bounds.col() - lower_bounds.col();

        (lower_bounds.row()..=upper_bounds.row()).for_each(|row| {
            (0..=max_col_index).for_each(|col| {
                if self.cell_state(row, lower_bounds.col() + col) == CellState::Populated {
                    flipped.push(Cell::new(row, lower_bounds.col() + max_col_index - col));
                }
            })
        });
        self.set_populated_cells(flipped);
    }
    pub fn shift_up(&mut self) {
        let max_row = self.cell_count - 1;
        
        // Remember the top row states
        let mut top_row: Vec<CellState> = vec![];
        (0..self.cell_count).for_each(|col| {
            top_row.push(self.cell_state(0, col));
        });
        // Shift the states up
        (0..max_row).for_each(|row| {
            (0..self.cell_count).for_each(|col| {
                self.set_cell_state(row, col, self.cell_state(row + 1, col));
            })
        });
        // Put the top row states into the bottom row
        (0..top_row.len()).for_each(|col| {
            self.set_cell_state(max_row, col as u32, top_row[col]);
        });
    }
    pub fn shift_down(&mut self) {
        let max_row = self.cell_count - 1;

        // Remember the bottom row states
        let mut bottom_row: Vec<CellState> = vec![];
        (0..self.cell_count).for_each(|col| {
            bottom_row.push(self.cell_state(max_row, col));
        });
        // Shift the states up
        (0..=max_row - 1).rev().for_each(|row| {
            (0..self.cell_count).for_each(|col| {
                self.set_cell_state(row + 1, col, self.cell_state(row, col));
            })
        });
        // Put the bottom row states into the top row
        (0..bottom_row.len()).for_each(|col| {
            self.set_cell_state(0, col as u32, bottom_row[col]);
        })
    }
    pub fn shift_left(&mut self) {
        let max_col = self.cell_count - 1;
        
        // Remember the left column states
        let mut left_column: Vec<CellState> = vec![];
        (0..self.cell_count).for_each(|row| {
            left_column.push(self.cell_state(row, 0));
        });
        // Shift states left
        (0..self.cell_count).for_each(|col| {
            (0..self.cell_count).for_each(|row| {
                self.set_cell_state(row, col, self.cell_state(row, col + 1));
            })
        });
        // Put the left states into the right row
        (0..left_column.len()).for_each(|row| {
            self.set_cell_state(row as u32, max_col, left_column[row]);
        })
    }
    pub fn shift_right(&mut self) {
        let max_col = self.cell_count - 1;
        
        // Remember the right column states
        let mut right_column: Vec<CellState> = vec![];
        (0..self.cell_count).for_each(|row| {
            right_column.push(self.cell_state(row, max_col));
        });
        // Shift states right
        (0..=max_col - 1).rev().for_each(|col| {
            (0..self.cell_count).for_each(|row| {
                self.set_cell_state(row, col + 1, self.cell_state(row, col));
            })
        });
        // Put the right states into the left row
        (0..right_column.len()).for_each(|row| {
            self.set_cell_state(row as u32, 0, right_column[row]);
        })
    }
}
impl LifeGrid {
    fn make_odd(n: u32) -> u32 {
        if n % 2 == 0 { n + 1 } else { n }
    }
    fn lower_bounds(&self) -> Cell {
        let mut low_row = u32::MAX;
        let mut low_col = u32::MAX;
        (0..self.cell_count).for_each(|row| {
            (0..self.cell_count).for_each(|col| {
                if self.cell_state(row, col) == CellState::Populated {
                    if row < low_row { low_row = row; }
                    if col < low_col { low_col = col; }
                }
            })
        });
        Cell::new(low_row, low_col)
    }
    fn upper_bounds(&self) -> Cell {
        let mut high_row = 0;
        let mut high_col = 0;
        (0..self.cell_count).for_each(|row| {
            (0..self.cell_count).for_each(|col| {
                if self.cell_state(row, col) == CellState::Populated {
                    if row > high_row { high_row = row; }
                    if col > high_col { high_col = col; }
                }
            })
        });
        Cell::new(high_row, high_col)
    }
    fn active_grid_size(&self) -> f64 {
        self.grid_size - Constants::BORDER_WIDTH - Constants::BORDER_WIDTH
    }
    fn cell_size(&self) -> f64 {
        self.active_grid_size() / self.cell_count as f64
    }
    fn cell_index(&self, row: u32, col: u32) -> usize {
        (row * self.cell_count + col) as usize
    }
    fn set_populated_cells(&mut self, populated: Vec<Cell>) {
        self.vacate_all_cells();
        populated.into_iter().for_each(|cell| {
            self.set_cell_state(cell.row(), cell.col(), CellState::Populated);
        })
    }
    fn in_bounds(&self, value: i32) -> bool {
        value >= 0 && value < self.cell_count as i32
    }
    fn in_grid(&self, row: i32, col: i32) -> bool {
        self.in_bounds(row) && self.in_bounds(col)
    }
    fn is_populated(&self, row: i32, col: i32) -> bool {
        self.in_grid(row, col) && self.cell_state(row as u32, col as u32) == CellState::Populated
    }
    fn count_neighbors(&self, row: u32, col: u32) -> usize {
        let mut neighbors = 0;
        (row as i32 - 1..=row as i32 + 1).for_each(|neighbor_row| {
            (col as i32 - 1..=col as i32 + 1).for_each(|neighbor_col| {
                // Don't compare to same cell
                if !(neighbor_col == col as i32 && neighbor_row == row as i32)
                    && self.is_populated(neighbor_row, neighbor_col)
                {
                    neighbors += 1;
                }
            })
        });
        neighbors
    }
    fn get_canvas_element(canvas_id: &str) -> web_sys::HtmlCanvasElement {
        let window = web_sys::window().expect("should be a global `window`");
        let document = window.document().expect("window should have a document");
        let element = document
            .get_element_by_id(canvas_id)
            .expect(format!("document should have an element with ID '{}'", canvas_id).as_str());
        element
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .expect(
                format!(
                    "element with ID '{}' should be an HTML canvas element",
                    canvas_id
                )
                .as_str(),
            )
    }
    fn get_canvas_rendering_context_2d(
        canvas_element: &web_sys::HtmlCanvasElement,
    ) -> web_sys::CanvasRenderingContext2d {
        canvas_element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("HTML canvas element should have a 2D context")
    }
    fn draw_grid(&self) {
        self.context.save();

        // Draw the outer border
        self.context.set_fill_style_str(Constants::BORDER_COLOR);
        self.context
            .fill_rect(0.0, 0.0, self.grid_size, self.grid_size);

        // Draw the inner border
        self.context
            .set_fill_style_str(Constants::BORDER_HIGHLIGHT_COLOR);
        let inset = (Constants::BORDER_WIDTH / 4.0) * 3.0;
        self.context.fill_rect(
            inset,
            inset,
            self.grid_size - inset - inset,
            self.grid_size - inset - inset,
        );

        // Draw the cells
        self.context
            .set_stroke_style_str(Constants::CELL_BORDER_COLOR);
        self.context.set_stroke_style_str("black");
        self.context.set_line_width(0.25);
        self.cell_states
            .iter()
            .enumerate()
            .for_each(|(index, cell_state)| {
                // Get the color of the cell, based on its status
                let color = match cell_state {
                    CellState::Populated => Constants::POPULATED_CELL_COLOR,
                    _ => Constants::VACANT_CELL_COLOR,
                };
                self.context.set_fill_style_str(color);

                // Calculate the top left corner of the cell
                let cell_size = self.cell_size();
                let x =
                    Constants::BORDER_WIDTH + (index % self.cell_count as usize) as f64 * cell_size;
                let y =
                    Constants::BORDER_WIDTH + (index / self.cell_count as usize) as f64 * cell_size;

                // Draw the cell
                self.context.fill_rect(x, y, cell_size, cell_size);
                self.context.stroke_rect(x, y, cell_size, cell_size);
            });
        self.context.restore();
    }
}
