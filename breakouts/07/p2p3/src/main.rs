use color_eyre::eyre::{eyre, Result};
use color_eyre::Help;
use mpi::traits::*;
use rand::prelude::*;

fn main() -> Result<()> {
    color_eyre::install()?;

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let nprocs = world.size();
    let rank = world.rank();

    let root_rank = 3;
    if nprocs < root_rank + 1 {
        return Err(eyre!(
            "Size of MPI_COMM_WORLD must be at least {}, but is {nprocs}!",
            root_rank + 1
        )
        .with_suggestion(|| {
            format!(
                "Increase `-n` value from {} to {}",
                nprocs,
                root_rank + 1
            )
        }));
    }

    let r_val: i32 = thread_rng().gen_range(1..100);

    if rank == root_rank {
        for i in 0..nprocs {
            if i == root_rank {
                println!("Process {i} has: {r_val}");
            } else {
                let (msg, _) = world.process_at_rank(i).receive::<i32>();
                println!("Process {i} has: {msg}");
            }
        }
    } else {
        world.process_at_rank(root_rank).send(&r_val);
    }

    Ok(())
}
