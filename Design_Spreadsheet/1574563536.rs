
/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
use std::collections::HashMap;

struct Spreadsheet {
    cells: HashMap<String, i32>,
}

impl Spreadsheet {
    fn new(_rows: i32) -> Self {
        Self { cells: HashMap::new() }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        self.cells.insert(cell, value);
    }

    fn reset_cell(&mut self, cell: String) {
        self.cells.insert(cell, 0);
    }

    fn get_value(&self, formula: String) -> i32 {
        if !formula.starts_with('=') {
            return formula.parse::<i32>().unwrap_or(0);
        }

        let expr = &formula[1..]; // Remove '='
        let tokens: Vec<&str> = expr.split('+').collect();
        
        let mut sum = 0;
        for token in tokens {
            if let Ok(num) = token.parse::<i32>() {
                sum += num;
            } else {
                sum += *self.cells.get(token).unwrap_or(&0);
            }
        }
        sum
    }
}


/**
 * Your Spreadsheet object will be instantiated and called as such:
 * let obj = Spreadsheet::new(rows);
 * obj.set_cell(cell, value);
 * obj.reset_cell(cell);
 * let ret_3: i32 = obj.get_value(formula);
 */