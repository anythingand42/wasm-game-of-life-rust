mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate fixedbitset;
use fixedbitset::FixedBitSet;

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
pub enum UniverseMode {
    FixedSizePeriodic,
    FixedSizeNonPeriodic,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    size: usize,
    cells: FixedBitSet,
    mode: UniverseMode,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        match self.mode {
            UniverseMode::FixedSizePeriodic => {
                for delta_row in [self.height - 1, 0, 1].iter().cloned() {
                    for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                        if delta_row == 0 && delta_col == 0 {
                            continue;
                        }
        
                        let neighbor_row = (row + delta_row) % self.height;
                        let neighbor_col = (column + delta_col) % self.width;
                        let idx = self.get_index(neighbor_row, neighbor_col);
                        count += self.cells[idx] as u8;
                    }
                }
                count
            },
            UniverseMode::FixedSizeNonPeriodic => {
                for delta_row in [-1, 0, 1].iter().cloned() {
                    for delta_col in [-1, 0, 1].iter().cloned() {
                        if delta_row == 0 && delta_col == 0 {
                            continue;
                        }
                        if (delta_row == -1 && row == 0) || (delta_row == 1 && row == self.height - 1) {
                            continue
                        }
                        if (delta_col == -1 && column == 0) || (delta_col == 1 && column == self.width - 1) {
                            continue
                        }
        
                        let neighbor_row = row + delta_row as u32;
                        let neighbor_col = column + delta_col as u32;
                        let idx = self.get_index(neighbor_row, neighbor_col);
                        count += self.cells[idx] as u8;
                    }
                }
                count
            },
        }
    }
}

// Public methods, exported to JavaScript
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

                next.set(idx, match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise
                });
            }
        }

        self.cells = next;
    }

    pub fn new(width: u32, height: u32, mode: UniverseMode) -> Universe {
        let size = (width * height) as usize;

        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, false);
        }

        Universe {
            width,
            height,
            size,
            cells,
            mode,
        }
    }

    pub fn reinit_cells(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.size = (width * height) as usize;
        for i in 0..self.size {
            self.cells.set(i, false);
        }
    }

    pub fn set_mode(&mut self, mode: UniverseMode) {
        self.mode = mode;
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.toggle(idx);
    }

    pub fn set_alive(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.set(idx, true);
    }

    pub fn set_dead(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.set(idx, false);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn render_string(&self) -> String {
        self.to_string()
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                let symbol = if self.cells.contains(idx) { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}