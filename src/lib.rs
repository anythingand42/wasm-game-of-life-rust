mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;
use web_sys::console;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub enum UniverseMode {
    FixedSizePeriodic,
    FixedSizeNonPeriodic,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    mode: UniverseMode,
    webgl_vertices: Vec<f32>,
}

fn create_webgl_vertices(width: u32, height: u32) -> Vec<f32> {
    let mut webgl_vertices = Vec::new();
    let row_offset = 2.0 / (width + 1) as f32;
    let col_offset = 2.0 / (height + 1) as f32;
    let cells_offset_koef = 0.5;
    for i in 0..height {
        let row_north = ((i + 1) as f32) * row_offset + row_offset * cells_offset_koef - 1.0;
        let row_south = ((i + 1) as f32) * row_offset - row_offset * cells_offset_koef - 1.0;
        for j in 0..width {
            let column_east = ((j + 1) as f32) * col_offset + col_offset * cells_offset_koef - 1.0;
            let column_west = ((j + 1) as f32) * col_offset - col_offset * cells_offset_koef - 1.0;
            webgl_vertices.push(row_north);
            webgl_vertices.push(column_west);
            webgl_vertices.push(row_north);
            webgl_vertices.push(column_east);
            webgl_vertices.push(row_south);
            webgl_vertices.push(column_east);

            webgl_vertices.push(row_north);
            webgl_vertices.push(column_west);
            webgl_vertices.push(row_south);
            webgl_vertices.push(column_west);
            webgl_vertices.push(row_south);
            webgl_vertices.push(column_east);
        }
    }
    webgl_vertices
}

// fn create_webgl_vertices(width: u32, height: u32) -> Vec<f32> {
//     let mut webgl_vertices = Vec::new();
//     let row_offset = 2.0 / (width + 1) as f32;
//     let col_offset = 2.0 / (height + 1) as f32;
//     for i in 0..height {
//         for j in 0..width {

//         }
//     }
//     webgl_vertices
// }

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        match self.mode {
            UniverseMode::FixedSizePeriodic => {

                let north = if row == 0 {
                    self.height - 1
                } else {
                    row - 1
                };
            
                let south = if row == self.height - 1 {
                    0
                } else {
                    row + 1
                };
            
                let west = if column == 0 {
                    self.width - 1
                } else {
                    column - 1
                };
            
                let east = if column == self.width - 1 {
                    0
                } else {
                    column + 1
                };
            
                let nw = self.get_index(north, west);
                count += self.cells[nw] as u8;
            
                let n = self.get_index(north, column);
                count += self.cells[n] as u8;
            
                let ne = self.get_index(north, east);
                count += self.cells[ne] as u8;
            
                let w = self.get_index(row, west);
                count += self.cells[w] as u8;
            
                let e = self.get_index(row, east);
                count += self.cells[e] as u8;
            
                let sw = self.get_index(south, west);
                count += self.cells[sw] as u8;
            
                let s = self.get_index(south, column);
                count += self.cells[s] as u8;
            
                let se = self.get_index(south, east);
                count += self.cells[se] as u8;
            
                count
            },

            UniverseMode::FixedSizeNonPeriodic => {
                let north = row - 1;
                let south = row + 1;
                let west = column - 1;
                let east = column + 1;

                let is_not_first_row = row != 0;
                let is_not_first_column = column != 0;
                let is_not_last_row = row != (self.height - 1);
                let is_not_last_column = column != (self.width - 1);
            
                if is_not_first_row {
                    let n = self.get_index(north, column);
                    count += self.cells[n] as u8;

                    if is_not_first_column {
                        let nw = self.get_index(north, west);
                        count += self.cells[nw] as u8;
                    }

                    if is_not_last_column {
                        let ne = self.get_index(north, east);
                        count += self.cells[ne] as u8;
                    }
                }

                if is_not_first_column {
                    let w = self.get_index(row, west);
                    count += self.cells[w] as u8;
                }
            
                if is_not_last_column {
                    let e = self.get_index(row, east);
                    count += self.cells[e] as u8;
                }
            
                if is_not_last_row {
                    let s = self.get_index(south, column);
                    count += self.cells[s] as u8;

                    if is_not_first_column {
                        let sw = self.get_index(south, west);
                        count += self.cells[sw] as u8;
                    }

                    if is_not_last_column {
                        let se = self.get_index(south, east);
                        count += self.cells[se] as u8;
                    }
                }
            
                count
            },
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {

        let _timer = Timer::new("Universe::tick");

        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new(width: u32, height: u32, mode: UniverseMode) -> Universe {
        // let cells = vec![Cell::Dead; (width * height) as usize];

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        
        let webgl_vertices = create_webgl_vertices(width, height);

        Universe {
            width,
            height,
            cells,
            mode,
            webgl_vertices,
        }
    }

    pub fn reinit_cells(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        // self.cells = vec![Cell::Dead; (width * height) as usize];

        self.webgl_vertices = create_webgl_vertices(width, height);

        self.cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
    }

    pub fn set_mode(&mut self, mode: UniverseMode) {
        self.mode = mode;
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx] = match self.cells[idx] {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }

    pub fn set_alive(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx] = Cell::Alive;
    }

    pub fn set_dead(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx] = Cell::Dead;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn webgl_cells(&self) -> *const u8 {
        let mut webgl_cells = Vec::new();
        for cell in self.cells.as_slice() {
            let c = *cell as u8;
            for _ in 0..6 {
                webgl_cells.push(c);
            }
        }
        webgl_cells.as_ptr()
    }

    pub fn render_string(&self) -> String {
        self.to_string()
    }

    pub fn webgl_vertices(&self) -> *const f32 {
        self.webgl_vertices.as_ptr()
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}