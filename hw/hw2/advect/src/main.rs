use heatmap_anim::*;
use indicatif::{ProgressBar, ProgressStyle};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pb = ProgressBar::new(NSTEP as u64);
    pb.set_style(ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({msg:>6})"
    )
        .unwrap()
        .progress_chars("#>-"));

    let mut u = [0.0_f64; N];
    let mut u_new = [0.0_f64; N];
    setup(&mut u, &mut u_new);

    let w: u32 = 1500;
    let h: u32 = 128;
    let gif_canvas = heatmap_anim::create_canvas("advect_anim.gif", w, h, M);

    save_frame(&gif_canvas, &u)?;

    for i in 1..=NSTEP {
        update(&mut u, &mut u_new);
        if (i as f64) % WSTEP == 0.0 {
            save_frame(&gif_canvas, &u)?;
            pb.set_message(format!("{:3.2}", (i as f64) / (NSTEP as f64) * 100.0));
            pb.inc(WSTEP as u64);
        }
    }

    save_frame(&gif_canvas, &u)?;
    pb.finish();

    Ok(())
}
