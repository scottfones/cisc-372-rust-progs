use std::env;

use mpi::collective::SystemOperation;
use mpi::traits::*;

fn is_perfect(num: u32) -> bool {
    if num < 2 {
        return false;
    }

    let mut sum = 1;
    let mut i = 2;
    loop {
        let i_sqrd = i * i;
        if i_sqrd < num {
            if num % i == 0 {
                sum += i + num / i;
            }
            i += 1;
        } else {
            if i_sqrd == num {
                sum += i;
            }
            break;
        }
    }
    sum == num
}

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let nprocs = world.size();
    let rank = world.rank();
    let t_start = mpi::time();
    let root_rank = 0;

    let args: Vec<String> = env::args().collect();
    if rank == root_rank && args.len() != 2 {
        panic!("Error: Program requires upper-bound argument");
    }

    let bound = args[1].parse::<u32>().unwrap();
    let mut num_perfect: u8 = 0;

    for num in ((rank as u32 + 1)..=bound).step_by(nprocs as usize) {
        if num % 1000000 == 0 {
            println!("i = {num}");
        }
        if is_perfect(num) {
            println!("Found a perfect number: {num}");
            num_perfect += 1;
        }
    }

    if rank == root_rank {
        let mut total_perfect = 0;

        world.process_at_rank(root_rank).reduce_into_root(
            &num_perfect,
            &mut total_perfect,
            SystemOperation::sum(),
        );
        let duration = mpi::time() - t_start;
        println!("Perfect numbers found: {total_perfect}");
        println!("Duration: {duration}");
    } else {
        world
            .process_at_rank(root_rank)
            .reduce_into(&num_perfect, SystemOperation::sum());
    }
}
