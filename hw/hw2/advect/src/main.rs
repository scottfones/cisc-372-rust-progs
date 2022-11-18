use std::fs::File;
use std::io::Write;
use std::time::Instant;

use plotters::coord::Shift;
use plotters::prelude::*;

const M: f64 = 100.0; // initial temperature of rod interior 
const N: usize = 1500; // number of discrete points including endpoints
const H0: usize = N / 4; // left endpoint of the heatsource 
const H1: usize = 3 * N / 4; // right endpoint of heat source
const C: f64 = 0.001; // advect constant 
const K: f64 = 0.05; // ddt/(dx*dx), diffusivity constant
const NSTEP: i32 = 4_000_000; // number of time steps
const WSTEP: f64 = 8_000.0; // time between animation update

fn setup(v: &mut [f64; N], v_new: &mut [f64; N]) {
    for i in H0..H1 {
        v[i] = M;
        v_new[i] = M;
    }
}

fn update(w: &mut [f64; N], w_new: &mut [f64; N]) {
    w_new
        .iter_mut()
        .skip(1)
        .take(N - 2)
        .enumerate()
        .for_each(|(i, val)| {
            let i = i + 1;
            *val = w[i] + K * (w[i + 1] + w[i - 1] - 2.0 * w[i]) - C * (w[i + 1] - w[i - 1]);
        });

    w_new[0] = w[0] + K * (w[1] + w[N - 1] - 2.0 * w[0]) - C * (w[1] - w[N - 1]);
    w_new[N - 1] = w[N - 1] + K * (w[0] + w[N - 2] - 2.0 * w[N - 1]) - C * (w[0] - w[N - 2]);

    std::mem::swap(w, w_new);
}

fn draw_heatmap(
    canvas: &DrawingArea<BitMapBackend, Shift>,
    w: &u32,
    h: &u32,
    u: &[f64; N],
) -> Result<(), Box<dyn std::error::Error>> {
    // loop invarient conversions for gif
    let width = *w as f64;
    let height = *h as i32;
    let slice_x_ratio = width / N as f64;
    let slice_width = slice_x_ratio.recip() as i32;

    for idx in 0..N {
        let i = idx as f64;
        let slice_x = (i * slice_x_ratio) as i32;
        let red = (u[idx] / M * 255.0) as u8;
        let c = RGBColor(red, 0, 255 - red);

        for x in slice_x..=(slice_x + slice_width) {
            for y in 0..=height {
                canvas.draw_pixel((x, y), &c)?;
            }
        }
    }
    canvas.present()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let start_time = Instant::now();
    let mut f_out = File::options()
        .append(true)
        .create(true)
        .open("advect_values.csv")?;

    let mut u = [0.0_f64; N];
    let mut u_new = [0.0_f64; N];
    setup(&mut u, &mut u_new);

    let w: u32 = 1500;
    let h: u32 = 100;
    let canvas = BitMapBackend::gif("advect_anim.gif", (w, h), 50)
        .unwrap()
        .into_drawing_area();

    draw_heatmap(&canvas, &w, &h, &u).unwrap();

    for i in 1..=NSTEP {
        update(&mut u, &mut u_new);
        if (i as f64) % WSTEP == 0.0 {
            draw_heatmap(&canvas, &w, &h, &u).unwrap();
        }
    }

    // writeln!(f_out, "{u:?}")?;
    let stop_time = start_time.elapsed().as_secs_f32();
    println!("Done in {stop_time} seconds");
    draw_heatmap(&canvas, &w, &h, &u).unwrap();

    Ok(())
}
