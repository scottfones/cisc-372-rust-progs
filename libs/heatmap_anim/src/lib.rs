use core::fmt;
use std::error::Error;

use plotters::coord::Shift;
use plotters::prelude::*;

mod palettes_256;

pub struct GifCanvas<'a> {
    canvas: DrawingArea<BitMapBackend<'a>, Shift>,
    width: u32,
    // height: u32,
    max_temp: f64,
}

impl<'a> GifCanvas<'a> {
    pub fn new(filename: &str, width: u32, height: u32, max_temp: f64) -> Self {
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
    TWO(&'b [Vec<f64>]),
}

pub fn save_frame<const N: usize>(
    gif_canvas: &GifCanvas,
    data_dim: DataDim<N>,
) -> Result<(), Box<dyn Error>> {
    match data_dim {
        DataDim::ONE(data) => save_frame_1d(gif_canvas, data),
        DataDim::TWO(data) => save_frame_2d(gif_canvas, data),
    }
}

fn save_frame_1d<const N: usize>(
    gif_canvas: &GifCanvas,
    data: &[f64; N],
) -> Result<(), Box<dyn Error>> {
    if (gif_canvas.width as usize) < data.len() {
        return Err(Box::new(DataLengthError(
            "canvas width should be at least as wide as the data".into(),
        )));
    }
    let plot_slices = gif_canvas.canvas.split_evenly((1, gif_canvas.width as usize));

    for (idx, slice) in plot_slices.iter().enumerate() {
        let color_code = (data[idx] / gif_canvas.max_temp * 255.0) as usize;
        slice.fill(&palettes_256::PaletteInferno256::pick(color_code))?;
    }
    gif_canvas.canvas.present()?;
    Ok(())
}

fn save_frame_2d(gif_canvas: &GifCanvas, data: &[Vec<f64>]) -> Result<(), Box<dyn Error>> {
    if (gif_canvas.width as usize) < data.len() {
        return Err(Box::new(DataLengthError(
            "canvas width should be at least as wide as the data".into(),
        )));
    }

    let plot_slices = gif_canvas.canvas.split_evenly((gif_canvas.width as usize, gif_canvas.width as usize));

    for idx_x in 0..gif_canvas.width {
        for idx_y in 0..gif_canvas.width {
        let color_code = (data[idx_x as usize][idx_y as usize] / gif_canvas.max_temp * 255.0) as usize;
        plot_slices[(idx_y * gif_canvas.width + idx_x) as usize].fill(&palettes_256::PaletteInferno256::pick(color_code))?;
        }
    }
    gif_canvas.canvas.present()?;
    Ok(())
    // let range_w = gif_canvas.canvas.get_pixel_range().0;
    //
    // for x in range_w {
    //     for y in gif_canvas.canvas.get_pixel_range().1 {
    //         let red = (data[x as usize][y as usize] / gif_canvas.max_temp * 255.0) as u8;
    //         let c = RGBColor(red, 0, 255 - red);
    //         gif_canvas.canvas.draw_pixel((x, y), &c)?;
    //     }
    // }
    //
    // gif_canvas.canvas.present()?;
    // Ok(())
}

#[derive(Debug)]
struct DataLengthError(String);

impl fmt::Display for DataLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incompatible data: {}", self.0)
    }
}

impl Error for DataLengthError {}
