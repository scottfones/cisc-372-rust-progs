use std::fs::File;
use std::io::Write;
use std::time::Instant;

use plotly::layout::{Axis, Margin};
use plotly::{HeatMap, Layout, Plot};

const M: f64 = 100.0; // initial temperature of rod interior 
const N: usize = 1500; // number of discrete points including endpoints
const H0: usize = N / 4; // Left endpoint of the heatsource 
const H1: usize = 3 * N / 4; // Right endpoint of heat source
const C: f64 = 0.001; // Advect constant 
const K: f64 = 0.05; // Ddt/(dx*dx), diffusivity constant
const NSTEP: i32 = 4_000_000; // Number of time steps
const WSTEP: f64 = 8_000.0; // Time between animation update

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

fn draw_heatmap(z0: &[f64; N]) {
    let z = z0.to_vec();
    let z_n: i32 = z.len().try_into().unwrap();
    let x: Vec<i32> = (1..z_n).collect();
    let y: Vec<i32> = vec![0; z.len()];
    let trace = HeatMap::new(x, y, z).zmax(100.0).zmin(0.0);

    let x_axis = Axis::new().tick_values(vec![]);
    let y_axis = Axis::new().tick_values(vec![]);
    let c_layout = Layout::new()
        .width(1200)
        .height(400)
        .x_axis(x_axis)
        .y_axis(y_axis)
        // .title(Title::new("Advection: 1D Heatmap"))
        .margin(Margin::new().left(0).right(0).top(0).bottom(0));
    // .plot_background_color(NamedColor::Black)
    // .paper_background_color(NamedColor::Black);

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(c_layout);

    // plot.save("a1d_0.png", ImageFormat::PNG,  400, 100, 1.0);
    plot.show();
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
    draw_heatmap(&u);
    writeln!(f_out, "{u:?}")?;

    for i in 1..=NSTEP {
        update(&mut u, &mut u_new);
        if (i as f64) % WSTEP == 0.0 {
            println!("{:.02}%", (i as f64) / (NSTEP as f64) * 100.0);
            writeln!(f_out, "{u:?}")?;
            // draw_heatmap(&u);
        }
    }

    writeln!(f_out, "{u:?}")?;
    let stop_time = start_time.elapsed().as_secs_f32();
    println!("Done in {stop_time} seconds");
    draw_heatmap(&u);

    Ok(())
}
