use std::env;

use color_eyre::eyre::{eyre, Result};
use color_eyre::Help;
use mpi::traits::*;

fn main() -> Result<()> {
    color_eyre::install()?;

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let nprocs = world.size();
    let rank = world.rank();

    if nprocs < 2 {
        return Err(
            eyre!("Size of MPI_COMM_WORLD must be at least 2, but is {nprocs}!",)
                .with_suggestion(|| format!("Increase `-n` value from {} to {}", nprocs, 2)),
        );
    }

    if rank == 0 {
        let args: Vec<i32> = env::args()
            .skip(1)
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let args_len = args.len();

        for i in 1..nprocs {
            world.process_at_rank(i).send(&args_len);
            world.process_at_rank(i).send(&args);
        }
    } else {
        let (arr_len, _) = world.process_at_rank(0).receive::<usize>();
        let (arr, _) = world.process_at_rank(0).receive_vec::<i32>();
        println!("Proc {rank}: the {arr_len} args are: {arr:?}");
    }

    Ok(())
}
