use image::{io::Reader as ImageReader, DynamicImage};

use super::matrix::{self, Matrix};

#[derive(Debug)]
pub struct ImgFx {
    image: DynamicImage,
    matrix: Matrix,
    width: usize,
    height: usize,
}

impl ImgFx {
    pub fn new(file_name: &'static str) -> Result<Self, Box<dyn std::error::Error>> {
        let img = ImageReader::open(file_name)?
            .with_guessed_format()?
            .decode()?;

        let matrix: Matrix = img.clone().into();

        // let img_buffer = img.to_rgba32f();

        // let rows_stride = img_buffer.as_flat_samples().layout.height_stride;
        // let  = img_buffer.into_vec();
        // println!("{height_stride:?}");
        Ok(Self {
            image: img,
            width: matrix.cols,
            height: matrix.rows,
            matrix,
        })
    }

    pub fn pixelate(&mut self, strength: Option<usize>) -> Self {
        let strength = strength.unwrap_or(10);

        todo!()
    }

    pub fn save(&mut self, file_name: &'static str) -> Self {
        todo!();
    }
}
