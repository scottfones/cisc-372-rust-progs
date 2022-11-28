use std::io::{self, Write};

use heatmap_anim::{save_frame, DataDim, GifCanvas};
use mpi::datatype::PartitionMut;
use mpi::topology::{Process, SystemCommunicator};
use mpi::traits::*;
use mpi::{point_to_point as p2p, Count};

/// Initial Conditions
const M: f64 = 100.0; // initial temperature of rod interior 
const N: usize = 800; // number of discrete points including endpoints
const C: f64 = 0.01; // advect constant 
const K: f64 = 0.05; // ddt/(dx*dx), diffusivity constant
const NSTEP: i32 = 300_000; // number of time steps
const WSTEP: f64 = 400.0; // time between animation update
const H0: usize = N / 2 - N / 3; // initial hot zone start
const H1: usize = N / 2 + N / 3; // initial hot zone stop

const ROOT_RANK: usize = 0; // root processor id

/// Applies the init heat value `M` to elements of `u` and `u_new`, determined
/// by `setup()`.
fn heat_up(i_start: usize, i_stop: usize, u: &mut [Vec<f64>], u_new: &mut [Vec<f64>]) {
    for i in i_start..i_stop {
        for j in H0..H1 {
            u[i][j] = M;
            u_new[i][j] = M;
        }
    }
}

/// Determines if a given proc contains elements within the initial hot zone. If
/// so, call heat_up()` to apply the values.
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

/// Gathers all processor data, `u`, into the root processor before sending the
/// collected data to `heatmap_anim::save_frame()`.
///
/// Data comes in as a nested `Vec<Vec<f64>>`, which isn't supported by the
/// `mpi` library. Therefore, `u` is flattened for the `mpi` gather operation
/// and reformated to an N-by-N `Vec<Vec<f64>>` before calling `save_frame()`.
fn sync_n_save(
    gif_canvas: &GifCanvas,
    rank: usize,
    u: Vec<Vec<f64>>,
    u_counts: &[usize],
    u_displs: &[usize],
    world: &SystemCommunicator,
) -> Result<(), Box<dyn std::error::Error>> {
    let root_process = world.process_at_rank(ROOT_RANK as i32);

    // exclude the ghost cells on either side and flatten
    let u_share: Vec<f64> = u
        .into_iter()
        .skip(1)
        .take(u_counts[rank])
        .flatten()
        .collect();

    if rank == ROOT_RANK {
        // accomodate flattened changes
        let counts: Vec<Count> = u_counts.iter().map(|x| (*x * N) as Count).collect();
        let displs: Vec<Count> = u_displs.iter().map(|x| (*x * N) as Count).collect();

        // gather flattened data
        let mut buf = vec![0.0_f64; N * N];
        let mut partition = PartitionMut::new(&mut buf[..], counts, &displs[..]);
        root_process.gather_varcount_into_root(&u_share, &mut partition);

        // reformat to N-by-N nested `Vec`s and plot
        let u_framed: Vec<Vec<f64>> = buf.chunks(N).map(|x: &[f64]| x.to_vec()).collect();
        save_frame(gif_canvas, DataDim::TWO::<N>(&u_framed))?;
    } else {
        root_process.gather_varcount_into(&u_share)
    }
    Ok(())
}

/// Applies the advection and diffusion equation.
fn update(n_local: usize, u: &mut Vec<Vec<f64>>, u_new: &mut Vec<Vec<f64>>) {
    for i in 1..=n_local {
        for j in 0..N {
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

/// Exchanges data to update ghost cells.
fn update_exchange(
    proc_left: &Process<SystemCommunicator>,
    proc_right: &Process<SystemCommunicator>,
    n_local: usize,
    u: &mut [Vec<f64>],
) {
    p2p::send_receive_into(&u[1].clone(), proc_left, &mut u[n_local + 1], proc_right);
    p2p::send_receive_into(&u[n_local].clone(), proc_right, &mut u[0], proc_left);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // mpi setup and vars
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let nprocs = world.size() as usize;
    let rank = world.rank() as usize;
    let t_start = mpi::time();

    let gif_canvas = GifCanvas::new("advect2d_anim.gif", N as u32, N as u32, M);

    // processor value displacements
    let displs: Vec<usize> = (0..nprocs).map(|r| r * N / nprocs).collect();
    // local values per processor
    let n_local: Vec<usize> = (0..nprocs)
        .map(|r| ((r + 1) * N / nprocs) - displs[r])
        .collect();

    // setup data vars
    let mut u = vec![vec![0.0_f64; N]; n_local[rank] + 2];
    let mut u_new = vec![vec![0.0_f64; N]; n_local[rank] + 2];
    setup(displs[rank], n_local[rank], &mut u, &mut u_new);

    // plot initial state
    sync_n_save(&gif_canvas, rank, u.clone(), &n_local, &displs, &world)?;

    let left_proc = if rank > 0 {
        world.process_at_rank(rank as i32 - 1)
    } else {
        world.process_at_rank(nprocs as i32 - 1)
    };

    let right_proc = if rank < nprocs - 1 {
        world.process_at_rank(rank as i32 + 1)
    } else {
        world.process_at_rank(0)
    };

    if rank == ROOT_RANK {
        print!("Progress:\n{:.0}%", 0.00);
        io::stdout().flush().unwrap();
    }

    for i in 1..=NSTEP {
        update_exchange(&left_proc, &right_proc, n_local[rank], &mut u);
        update(n_local[rank], &mut u, &mut u_new);

        // update animation every `WSTEP` iterations
        if (i as f64) % WSTEP == 0.0 {
            sync_n_save(&gif_canvas, rank, u.clone(), &n_local, &displs, &world)?;

            if rank == ROOT_RANK && (i as f64) % (WSTEP * 8.0) == 0.0 {
                print!(".");
                io::stdout().flush().unwrap();

            if (i as f64) % (WSTEP * 25.0) == 0.0 {
                    print!("{}%", f64::trunc(i as f64 / NSTEP as f64 * 100.0));
                    io::stdout().flush().unwrap();
                }
            }
        }
    }

    // plot final state
    sync_n_save(&gif_canvas, rank, u.clone(), &n_local, &displs, &world)?;

    if rank == ROOT_RANK {
        println!("100%\nDone in {} seconds.", mpi::time() - t_start);
    }

    Ok(())
}
