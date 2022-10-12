use mpi::traits::*;
use mpi::collective::CommunicatorCollectives;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();

    for _ in 0..10 {
        println!("{}: hi", rank);
    }

    world.barrier();

    for _ in 0..10 {
        println!("{}: bye", rank);
    }
}
