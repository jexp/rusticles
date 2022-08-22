extern crate fixedbitset;
use fixedbitset::FixedBitSet;

mod utils;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!",name));
}

const Dead:bool = false;
const Alive:bool = true;

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    // ...

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}
impl Universe {

    pub fn cell_tuples(&self) -> Vec::<(usize, usize)> {
        self.cells.ones().map(|idx| (idx/self.width, idx % self.width)).collect()
    }
}

impl Universe {
    fn get_index(&self, row: usize, column: usize) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                if self.cells[idx] {
                    count += 1;
                }
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Alive, x) if x < 2 => Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Alive, 2) | (Alive, 3) => Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Alive, x) if x > 3 => Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Dead, 3) => Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next.set(idx,next_cell);
            }
        }

        self.cells = next;
    }

    // ...
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    // ...

    pub fn new(rate:f32) -> Universe {
        let mut rng = rand::thread_rng();
        let width = 64;
        let height = 64;
        let size = width*height;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            if rng.gen::<f32>() < rate {
                cells.set(i, Alive);
            }
        }
        Universe {
            width,
            height,
            cells,
        }
    }
}
impl Universe {
    pub fn init(width: usize, height: usize, fixture: &[(usize, usize)]) -> Universe {
        let size = width*height;
        let mut cells = FixedBitSet::with_capacity(size);
        for (row,column) in fixture {
            cells.set(row * width + column,true);
        }
        Universe {
            width,
            height,
            cells,
        }
    }
}