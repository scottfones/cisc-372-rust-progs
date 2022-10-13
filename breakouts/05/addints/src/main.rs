use mpi::collective::SystemOperation;
use mpi::traits::*;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let nprocs = world.size();
    let rank = world.rank();
    let start_time = mpi::time();

    let stop = 10_i32.pow(9);
    let step: usize = usize::try_from(nprocs).unwrap();
    let mut local_sum = 0_u64;
    for i in (rank..=stop).step_by(step) {
        local_sum += u64::try_from(i).unwrap();
    }

    let root_rank = 0;
    if rank == root_rank {
        let mut global_sum = 0_u64;
        world.process_at_rank(root_rank).reduce_into_root(
            &local_sum,
            &mut global_sum,
            SystemOperation::sum(),
        );
        let total_time = mpi::time() - start_time;
        println!("Time: {total_time}");
        println!("Sum: {global_sum}");
    } else {
        world
            .process_at_rank(root_rank)
            .reduce_into(&local_sum, SystemOperation::sum());
    }
}
