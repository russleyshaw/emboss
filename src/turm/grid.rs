
#[derive(Clone)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(rows: usize, cols: usize, value: T) -> Self {
        let empty_row = vec![value; cols];
        let data = vec![empty_row; rows];
        
        Self {
            rows,
            cols,
            data
        }
    }

    pub fn resize(&mut self, rows: usize, cols: usize, value: T) {
        for row in self.data.iter_mut() {
            row.resize(cols, value.clone());
        }

        let empty_row = vec![value; cols];
        self.data.resize(rows, empty_row);
        self.rows = rows;
        self.cols = cols;
    }

    pub fn in_bounds(&self, r: usize, c: usize) -> bool {
        if r >= self.data.len() { return false }
        let row = &self.data[r];
        if c >= row.len() { return false }
        return true;
    }

}



