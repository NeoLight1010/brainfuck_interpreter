use std::collections::HashMap;

pub struct Data {
    pointer: u32,
    cells: HashMap<u32, u32>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            pointer: 0,
            cells: HashMap::new()
        }
    }

    pub fn inc_pointer(&mut self) {
        self.pointer += 1;
    }

    pub fn dec_pointer(&mut self) {
        self.pointer -= 1;
    }

    /**Get value stored in current cell.*/
    pub fn get_cell(&self) -> u32 {
        match self.cells.get(&self.pointer) {
            Some(&v) => v,
            None => 0
        }
    }

    pub fn inc_cell(&mut self) {
        match self.cells.get_mut(&self.pointer) {
            Some(v) => { *v += 1; },
            None => { self.cells.insert(self.pointer, 1); }
        };
    }
 
    pub fn dec_cell(&mut self) {
        match self.cells.get_mut(&self.pointer) {
            Some(v) => { *v -= 1; },
            None => { self.cells.insert(self.pointer, u32::MAX); }
        };
    }

    pub fn set_cell(&mut self, v: u32) {
        self.cells.insert(self.pointer, v);
    }
}
