use image::DynamicImage;
use rand::{thread_rng, Rng};

#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn random(rows: usize, cols: usize) -> Self {
        let mut rng = thread_rng();
        let mut res = Matrix::zeros(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f32>() * 2.0 - 1.0
            }
        }
        res
    }

    pub fn from(data: Vec<Vec<f32>>) -> Matrix {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data,
        }
    }

    pub fn multiply(&mut self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Attempted to multiply by matrix of incorrect dimensions")
        }
        let mut res = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j]
                }
                res.data[i][j] = sum;
            }
        }
        res
    }

    pub fn add(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to add by matrix with inconsistent rows or cols")
        }
        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j]
            }
        }
        res
    }

    pub fn dot_multiply(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to dot multiply by matrix with inconsistent rows or cols")
        }
        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j]
            }
        }
        res
    }

    pub fn substract(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to subtract by matrix with inconsistent rows or cols")
        }
        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j]
            }
        }
        res
    }

    pub fn map(&mut self, function: &dyn Fn(f32) -> f32) -> Matrix {
        Matrix::from(
            self.data
                .clone()
                .into_iter()
                .map(|row| row.into_iter().map(|value| function(value)).collect())
                .collect(),
        )
    }

    pub fn transpose(&mut self) -> Matrix {
        let mut res = Matrix::zeros(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j]
            }
        }
        res
    }
}

impl Into<Matrix> for DynamicImage {
    fn into(self) -> Matrix {
        let img_buffer = self.to_rgba32f();

        let rows_stride = img_buffer.as_flat_samples().layout.height_stride;

        let e: Vec<Vec<f32>> = img_buffer
            .into_vec()
            .chunks_exact(rows_stride)
            .map(|c| c.to_vec())
            .collect();

        Matrix::from(e)
    }
}
