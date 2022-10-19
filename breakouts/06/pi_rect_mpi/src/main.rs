use mpi::collective::SystemOperation;
use mpi::traits::*;

const INTERVALS: i32 = 1_000_000_000;

fn main() {
    let delta = 1.0 / f64::from(INTERVALS);
    let delta4 = 4.0 * delta;
    let mut my_area: f64 = 0.0;

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let nprocs = world.size();
    let rank = world.rank();
    let start_time = mpi::time();

    for i in (rank..INTERVALS).step_by(nprocs.try_into().unwrap()) {
        let x = (f64::from(i) + 0.5) * delta;
        my_area += delta4 / (1.0 + x * x);
    }

    let root_rank = 0;
    if rank == root_rank {
        let mut total_area: f64 = 0.0;
        world.process_at_rank(root_rank).reduce_into_root(
            &my_area,
            &mut total_area,
            SystemOperation::sum(),
        );
        let total_time = mpi::time() - start_time;
        eprintln!("Pi is {total_area:20.17}. Time: {total_time}");
        println!("{nprocs} {total_time}");
    } else {
        world
            .process_at_rank(root_rank)
            .reduce_into(&my_area, SystemOperation::sum());
    }
}
