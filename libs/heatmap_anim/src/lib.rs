use core::fmt;
use std::error::Error;

use plotters::coord::Shift;
use plotters::prelude::*;

pub struct GifCanvas<'a> {
    canvas: DrawingArea<BitMapBackend<'a>, Shift>,
    width: u32,
    // height: u32,
    max_temp: f64,
}

impl<'a> GifCanvas<'a> {
    fn new(filename: &str, width: u32, height: u32, max_temp: f64) -> Self {
        let canvas = BitMapBackend::gif(filename, (width, height), 50)
            .unwrap()
            .into_drawing_area();

        GifCanvas {
            canvas,
            width,
            // height,
            max_temp,
        }
    }
}

pub enum DataDim<'b, const N: usize> {
    ONE(&'b [f64; N]),
    TWO(&'b [[f64; N]; N]),
}

/// Returns a GifCanvas
pub fn create_canvas(filename: &str, width: u32, height: u32, max_temp: f64) -> GifCanvas {
    GifCanvas::new(filename, width, height, max_temp)
}

pub fn save_frame<const N: usize>(gif_canvas: &GifCanvas, data_dim: DataDim<N>) -> Result<(), Box<dyn Error>> {
    match data_dim {
        DataDim::ONE(data) => save_frame_1d(gif_canvas, data),
        DataDim::TWO(_) => todo!(),
    }
}

fn save_frame_1d<const N: usize>(
    gif_canvas: &GifCanvas,
    data: &[f64; N],
) -> Result<(), Box<dyn Error>> {
    if gif_canvas.width as usize != data.len() {
        return Err(Box::new(DataLengthError(
            "canvas width and data lengh must be equal".into(),
        )));
    }

    let range_w = gif_canvas.canvas.get_pixel_range().0;

    for x in range_w {
        let red = (data[x as usize] / gif_canvas.max_temp * 255.0) as u8;
        let c = RGBColor(red, 0, 255 - red);

        for y in gif_canvas.canvas.get_pixel_range().1 {
            gif_canvas.canvas.draw_pixel((x, y), &c)?;
        }
    }
    gif_canvas.canvas.present()?;
    Ok(())
}

#[derive(Debug)]
struct DataLengthError(String);

impl fmt::Display for DataLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incompatible data: {}", self.0)
    }
}

impl Error for DataLengthError {}
