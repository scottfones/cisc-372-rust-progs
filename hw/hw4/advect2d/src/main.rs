use heatmap_anim::{save_frame, DataDim, GifCanvas};
// use indicatif::{ProgressBar, ProgressStyle};
use mpi::datatype::PartitionMut;
use mpi::topology::{Process, SystemCommunicator};
use mpi::traits::*;
use mpi::{point_to_point as p2p, Count};

const M: f64 = 100.0; // initial temperature of rod interior 
const N: usize = 800; // number of discrete points including endpoints
const C: f64 = 0.01; // advect constant 
const K: f64 = 0.05; // ddt/(dx*dx), diffusivity constant
const NSTEP: i32 = 300_000; // number of time steps
const WSTEP: f64 = 400.0; // time between animation update

const H0: usize = N / 2 - N / 3;
const H1: usize = N / 2 + N / 3;

fn heat_up(i_start: usize, i_stop: usize, u: &mut [Vec<f64>], u_new: &mut [Vec<f64>]) {
    for j in H0..H1 {
        for i in i_start..i_stop {
            u[j][i] = M;
            u_new[j][i] = M;
        }
    }
}

fn setup(first: usize, n_local: usize, u: &mut [Vec<f64>], u_new: &mut [Vec<f64>]) {
    // slice is completely within initial heat zone
    if first >= H0 && first + n_local <= H1 {
        heat_up(1, n_local + 2, u, u_new);

        // slice crosses the end of initial heat zone
    } else if first >= H0 && first + n_local > H1 {
        heat_up(1, H1 - first + 1, u, u_new);

        // slice crosses into inital heat zone
    } else if first < H0 && first + n_local <= H1 {
        heat_up(H0 - first + 1, n_local + 2, u, u_new);
    }
}

fn sync_n_save(
    gif_canvas: &GifCanvas,
    rank: usize,
    u: &[Vec<f64>],
    u_counts: &[usize],
    u_displs: &[usize],
    world: &SystemCommunicator,
) -> Result<(), Box<dyn std::error::Error>> {
    let root_process = world.process_at_rank(0);
    let mut buf = vec![vec![0.0_f64; N]; N];

    let counts: Vec<Count> = u_counts.iter().map(|x| *x as Count).collect();
    let displs: Vec<Count> = u_displs.iter().map(|x| *x as Count).collect();
    let data_len = u_counts[rank] + 1;

    for i in 0..N {
        if rank == 0 {
            if i == 0 {
                dbg!(&u[i][..].len());
                println!("gather len: {}", &u[i][1..data_len].len());
            }
            let mut partition = PartitionMut::new(&mut buf[i][..], counts.clone(), &displs[..]);
            root_process.gather_varcount_into_root(&u[i][1..data_len], &mut partition);
        } else {
            root_process.gather_varcount_into(&u[i][1..data_len])
        }
    }
    if rank == 0 {
        save_frame(gif_canvas, DataDim::TWO::<N>(&buf))?;
    }
    Ok(())
}

fn update(n_local: usize, u: &mut Vec<Vec<f64>>, u_new: &mut Vec<Vec<f64>>) {
    for i in 0..N {
        for j in 1..=n_local {
            u_new[i][j] = u[i][j]
                + K * (u[(i + 1) % N][j]
                    + u[(i + N - 1) % N][j]
                    + u[i][(j + 1) % N]
                    + u[i][(j + N - 1) % N]
                    - 4.0 * u[i][j])
                - C * (u[(i + 1) % N][j] - u[(i + N - 1) % N][j] + u[i][(j + 1) % N]
                    - u[i][(j + N - 1) % N]);
        }
    }

    std::mem::swap(u, u_new);
}

fn update_exchange(
    proc_left: &Process<SystemCommunicator>,
    proc_right: &Process<SystemCommunicator>,
    n_local: usize,
    u: &mut [Vec<f64>],
) {
    p2p::send_receive_into(&u[1].clone(), proc_left, &mut u[n_local - 1], proc_right);
    p2p::send_receive_into(&u[n_local].clone(), proc_right, &mut u[0], proc_left);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let nprocs = world.size() as usize;
    let rank = world.rank() as usize;
    // let t_start = mpi::time();
    let root_rank = 0;

    let gif_canvas = GifCanvas::new("advect2d_anim.gif", N as u32, N as u32, M);
    // let pb = ProgressBar::new(NSTEP as u64);
    // pb.set_style(ProgressStyle::with_template(
    //     "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]
    // {pos:>7}/{len:7} ({msg:>7})"     )
    //         .unwrap()
    //         .progress_chars("#>-"));

    let left = if rank > 0 {
        rank as i32 - 1
    } else {
        nprocs as i32 - 1
    };
    let left_proc = world.process_at_rank(left);

    let right = if rank < nprocs - 1 {
        rank as i32 + 1
    } else {
        0_i32
    };
    let right_proc = world.process_at_rank(right);

    let displs: Vec<usize> = (0..nprocs).map(|r| r * N / nprocs).collect();
    let n_local: Vec<usize> = (0..nprocs)
        .map(|r| ((r + 1) * N / nprocs) - displs[r])
        .collect();
    if rank == 0 {
        dbg!(&displs);
        dbg!(&n_local);
    }
    let mut u = vec![vec![0.0_f64; n_local[rank] + 2]; N];
    let mut u_new = vec![vec![0.0_f64; n_local[rank] + 2]; N];
    setup(displs[rank], n_local[rank], &mut u, &mut u_new);

    world.barrier();
    sync_n_save(&gif_canvas, rank, &u, &n_local, &displs, &world)?;

    println!("update loop");
    for i in 1..=NSTEP {
        update_exchange(&left_proc, &right_proc, n_local[rank], &mut u);
        update(n_local[rank], &mut u, &mut u_new);

        if (i as f64) % WSTEP == 0.0 {
            world.barrier();
            sync_n_save(&gif_canvas, rank, &u, &n_local, &displs, &world)?;
            if rank == root_rank {
                println!("{:03.2}%", i / NSTEP * 100);
                // pb.set_message(format!("{:3.2}%", (i as f64) / (NSTEP as f64)
                // * 100.0)); pb.inc(WSTEP as u64);
            }
        }
    }

    // sync_n_save(&gif_canvas, nprocs, rank, &u, &n_local, &displs);
    // pb.finish();

    Ok(())
}
