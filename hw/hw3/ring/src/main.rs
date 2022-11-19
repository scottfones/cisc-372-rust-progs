use mpi::point_to_point as p2p;
use mpi::traits::*;
use rand::Rng;

fn gen_msg() -> [u8; 5] {
    let mut rng = rand::thread_rng();

    let mut r_vals = [0_u8; 5];
    rng.fill(&mut r_vals);
    r_vals
}

fn print_msg(is_send: bool, msg: &[u8; 5], rank: i32) {
    let intro_opt = if is_send { "has" } else { "received" };
    let intro = format!("Process {rank} {intro_opt}");
    println!(
        "{intro:<20}: {:>3}  {:>3}  {:>3}  {:>3}  {:>3}",
        msg[0], msg[1], msg[2], msg[3], msg[4]
    );
}

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let nprocs = world.size();
    let rank = world.rank();

    let mut my_rands = gen_msg();
    print_msg(true, &my_rands, rank);

    let dst_rank = (rank + 1) % nprocs;
    let dst_proc = world.process_at_rank(dst_rank);

    let src_rank = (rank - 1 + nprocs) % nprocs;
    let src_proc = world.process_at_rank(src_rank);

    p2p::send_receive_replace_into(&mut my_rands, &dst_proc, &src_proc);
    print_msg(false, &my_rands, rank);
}
