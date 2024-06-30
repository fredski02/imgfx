use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageBuffer, Rgba};

mod lib;
use lib::imgfx::ImgFx;

fn main() {
    let image_processor = ImgFx::new("input.png").unwrap();
    // println!("{image_processor:?}");
    // let image_processor = ImgFx::new("input.png").pixelate().save("output.png");
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let img = ImageReader::open("input.png")?.decode()?;
//     let height = img.height() as usize;
//     let width = img.width() as usize;

//     let rgba_data = to_rgba(&img);
//     let new_data = to_raw(convolve(&rgba_data, height, width));

//     // Create an ImageBuffer from the raw data
//     let new_image = ImageBuffer::<Rgba<u8>, _>::from_raw(width as u32, height as u32, new_data)
//         .expect("Failed to create image buffer");

//     // Convert the ImageBuffer to DynamicImage
//     let dynamic_image = DynamicImage::ImageRgba8(new_image);

//     // Save the image
//     dynamic_image.save("output.png")?;

//     Ok(())

// }

pub fn to_rgba(img: &DynamicImage) -> Vec<Rgba<u8>> {
    let image_data: Vec<Rgba<u8>> = img
        .to_rgba8()
        .chunks_exact(4)
        .map(|c| Rgba([c[0], c[1], c[2], c[3]]))
        .collect();
    image_data
}

pub fn to_raw(img: Vec<Rgba<u8>>) -> Vec<u8> {
    let mut u8_vec: Vec<u8> = Vec::with_capacity(img.len() * 4);

    for Rgba(rgba_arr) in img {
        u8_vec.extend_from_slice(rgba_arr.as_slice());
    }
    u8_vec
}

pub fn convolve(image_data: &Vec<Rgba<u8>>, height: usize, width: usize) -> Vec<Rgba<u8>> {
    let kernel_size = 10usize;
    let mut image = image_data.clone();

    for row in (0..height).step_by(kernel_size) {
        for col in (0..width).step_by(kernel_size) {
            let mut sum_red: u32 = 0;
            let mut sum_green: u32 = 0;
            let mut sum_blue: u32 = 0;
            let mut sum_alpha: u32 = 0;
            let mut count = 0;

            for k_row in 0..kernel_size {
                for k_col in 0..kernel_size {
                    let p_row = row + k_row;
                    let p_col = col + k_col;

                    if p_row < height && p_col < width {
                        let index = p_row * width + p_col;
                        sum_red += image[index].0[0] as u32;
                        sum_green += image[index].0[1] as u32;
                        sum_blue += image[index].0[2] as u32;
                        sum_alpha += image[index].0[3] as u32;

                        count += 1;
                    }
                }
            }

            if count > 0 {
                let avg_r = (sum_red / count) as u8;
                let avg_g = (sum_green / count) as u8;
                let avg_b = (sum_blue / count) as u8;
                let avg_a = (sum_alpha / count) as u8;

                for k_row in 0..kernel_size {
                    for k_col in 0..kernel_size {
                        let p_row = row + k_row;
                        let p_col = col + k_col;

                        if p_row < height && p_col < width {
                            let index = p_row * width + p_col;
                            image[index] = Rgba([avg_r, avg_g, avg_b, avg_a]);
                        }
                    }
                }
            }
        }
    }
    image
}
