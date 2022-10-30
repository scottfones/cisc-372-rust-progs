use color_eyre::eyre::Result;
use mpi::traits::*;
use rand::prelude::*;

fn main() -> Result<()> {
    color_eyre::install()?;

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let nprocs = world.size();
    let rank = world.rank();

    let root_rank = 3;
    assert!(nprocs > root_rank);

    let mut rng = thread_rng();
    let r_vals: Vec<i32> = (0..5).map(|_| rng.gen_range(1..100)).collect();

    if rank == root_rank {
        for i in 0..nprocs {
            if i == root_rank {
                println!("Process {i} has: {r_vals:?}");
            } else {
                let (msg, _) = world.process_at_rank(i).receive_vec::<i32>();
                println!("Process {i} has: {msg:?}");
            }
        }
    } else {
        world.process_at_rank(root_rank).send(&r_vals);
    }

    Ok(())
}
