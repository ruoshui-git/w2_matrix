use std::fmt;

#[derive(Clone, Debug)]
/// Row major rectangular matrix
/// Each row represents a new point
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

// constructor, get, set
impl Matrix {
    /// Row major index
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
        // col * self.rows + row
    }

    #[allow(dead_code)]
    pub fn new_clone_vec(rows: usize, cols: usize, data: &Vec<f64>) -> Matrix {
        assert_eq!(rows * cols, data.len(), "rows * cols must == data.len()");

        Matrix {
            rows,
            cols,
            data: data.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Matrix {
        assert_eq!(rows * cols, data.len(), "rows * cols must == data.len()");
        Matrix { rows, cols, data }
    }

    #[allow(dead_code)]
    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row > self.rows || col > self.cols {
            None
        } else {
            Some(self.data[self.index(row, col)])
        }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, row: usize, col: usize, data: f64) {
        assert!(row < self.rows && col < self.cols, "Index out of bound");
        let i = self.index(row, col);
        self.data[i] = data;
    }
}

#[allow(dead_code)]
// add edge (row)
impl Matrix {
    pub fn append_row(&mut self, row: &mut Vec<f64>) {
        assert_eq!(
            self.cols,
            row.len(),
            "Length of edge and matrix column size don't match"
        );
        self.data.append(row);
        self.rows += 1;
    }

    #[allow(dead_code)]
    pub fn append_edge(&mut self, edge: &mut Vec<f64>) {
        assert_eq!(
            self.cols,
            edge.len() + 1,
            "Length of edge and matrix column size don't match"
        );
        edge.push(1.0);
        self.data.append(edge);
        self.rows += 1;
    }
}

#[allow(dead_code)]
// row and col iter
impl Matrix {

    /// Iterate over a certain row
    pub fn row_iter<'a>(&'a self, r: usize) -> impl Iterator<Item = &f64> {
        let start = r * self.cols;
        self.data[start..start + self.cols].iter()
    }

    /// Iterate over a certain column
    pub fn col_iter<'a>(&'a self, c: usize) -> impl Iterator<Item = &f64> {
        self.data.iter().skip(c).step_by(self.cols)
    }

    /// Interate over the matrix by row, one row at a time
    /// 
    /// Returns an iterator for the row
    pub fn iter_by_row(&self) -> std::slice::Chunks<'_, f64> {
        self.data.as_slice().chunks(self.cols)
    }
}

#[allow(dead_code)]
// mul
impl Matrix {
    fn index_to_rc(i: usize, cols: usize) -> (usize, usize) {
        (i / cols, i % cols)
    }

    /// Multiplies self matrix by other matrix
    pub fn mul(&self, other: &Self) -> Self {
        // other * self
        assert_eq!(self.cols, other.rows, "cols of m1 must == rows of m2");
        let (frows, fcols) = (self.rows, other.cols);
        let mut fdata = vec![0.0; frows * fcols];
        for (i, d) in fdata.iter_mut().enumerate() {
            let (r, c) = Self::index_to_rc(i, fcols);
            *d = self
                .row_iter(r)
                .zip(other.col_iter(c))
                .fold(0.0, |sum, (a, b)| sum + a * b);
        }
        Matrix::new(frows, fcols, fdata)
    }

    pub fn mul_mut_b(a: &Matrix, b: &mut Matrix) {
        *b = a.mul(b);
        // println!("result: {}", b);
    }
}

// identity
impl Matrix {

    #[allow(dead_code)]
    /// Make a new identity matrix with size `size`
    pub fn ident(size: usize) -> Self {
        let mut m = Matrix::new(size, size, vec![0.0; size * size]);
        for i in 0..size {
            m.set(i, i, 1.0);
        }
        m
    }

    #[allow(dead_code)]
    /// Transforms self into an identity matrix
    pub fn to_ident(&mut self) {
        let cols = self.cols;
        for (i, d) in self.data.iter_mut().enumerate() {
            *d = if {
                let (r, c) = Matrix::index_to_rc(i, cols);
                r == c
            } {
                1.0
            } else {
                0.0
            }
        }
    }
}

// print Matrix
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.rows == 0 || self.cols == 0 {
            write!(f, "Empty matrix ({} by {})", self.rows, self.cols)?;
        } else {
            writeln!(f, "Matrix ({} by {}) {{", self.rows, self.cols)?;

            for col_offset in 0..self.cols {
                write!(f, "  ")?; // indentation
                for d in self.data.iter().skip(col_offset).step_by(self.cols) {
                    write!(f, "{arg:.prec$} ", arg = d, prec = 2)?;
                }
                writeln!(f)?; // line change
            }
            write!(f, "}}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix_equal(m1: &Matrix, m2: &Matrix) -> bool {
        m1.rows == m2.rows
            && m1.cols == m2.cols
            && m1.data.iter().zip(m2.data.iter()).all(|(a, b)| a == b)
    }

    #[test]
    #[ignore]
    fn print_matrix() {
        let m = Matrix::new(
            7,
            5,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, 3.0, 4.0, 5.0, 1.0,
                2.0, 3.0, 4.0, 5.0, 1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0,
                3.0, 4.0, 5.0,
            ],
        );
        println!("M: {}", m);
        println!("M: {:?}", m);
    }

    #[test]
    fn add_edge() {
        let mut m = Matrix::new(0, 4, vec![]);
        println!("m: {}", m);
        println!("Adding (1, 2, 4) and (5, 6, 7) to empty matrix",);
        m.append_edge(&mut vec![1.0, 2.0, 4.0]);
        m.append_edge(&mut vec![5.0, 6.0, 7.0]);
        println!("m: {}", m);
        assert!(
            matrix_equal(
                &m,
                &Matrix::new(2, 4, vec![1.0, 2.0, 4.0, 1.0, 5.0, 6.0, 7.0, 1.0,])
            ),
            "Matrix not equal"
        );
    }

    #[test]
    fn multiply_with_method() {
        let m1 = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let m2 = Matrix::new(3, 2, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
        let mp = m1.mul(&m2);
        println!("{} mul by {} = {}", m1, m2, m1.mul(&m2));
        assert!(matrix_equal(
            &mp,
            &Matrix::new(2, 2, vec![58.0, 64.0, 139.0, 154.0,])
        ));
    }

    #[test]
    fn multiple_and_mutate_b() {
        let a = Matrix::new(1, 3, vec![3.0, 4.0, 2.0]);
        let mut b = Matrix::new(
            3,
            4,
            vec![13.0, 9.0, 7.0, 15.0, 8.0, 7.0, 4.0, 6.0, 6.0, 4.0, 0.0, 3.0],
        );
        println!("a: {}", a);
        println!("b: {}", b);
        println!("multiplying...",);
        Matrix::mul_mut_b(&a, &mut b);
        println!("b: {}", b);
        assert!(matrix_equal(
            &b,
            &Matrix::new(1, 4, vec![83.0, 63.0, 37.0, 75.0])
        ));
    }

    #[test]
    fn test_new_ident()
    {
        let ident = Matrix::ident(3);
        assert!(matrix_equal(&ident, &Matrix::new(3, 3, vec![
            1.0, 0.0, 0.0, 
            0.0, 1.0, 0.0, 
            0.0, 0.0, 1.0, 
        ])), "3 x 3 matrix");

        assert!(matrix_equal(&Matrix::ident(1), &Matrix::new(1, 1, vec![1.0])), "1 x 1 matrix edge case");
    }

    #[test]
    fn test_inplace_ident()
    {
        let mut m = Matrix::new(5, 5, vec![120.0; 25]);
        println!("m init: {}", m);
        println!("Mutating m...", );
        m.to_ident();
        println!("m is now {}", m);
        assert!(matrix_equal(&m, &Matrix::ident(5)), "5 x 5 matrix");
        
        let mut m = Matrix::new(1, 1, vec![50.0]);
        m.to_ident();
        assert!(matrix_equal(&m, &Matrix::ident(1)), "1 x 1 matrix edge case");
    }

}
