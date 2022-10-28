use mpi::traits::*;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let nprocs = world.size();
    let rank = world.rank();

    if nprocs < 4 {
        panic!("Size of MPI_COMM_WORLD must be at least 4, but is {nprocs}!");
    }

    let send_rank = 1;
    let recv_rank = 3;
    match rank {
        s if s == send_rank => {
            let msg = "Greetings to proc 3 from proc 1!".as_bytes();
            world.process_at_rank(recv_rank).send(&msg[..]);
        },
        r if r == recv_rank => {
            let (msg, _) = world.process_at_rank(send_rank).receive_vec::<u8>();
            println!("Received from proc {send_rank}: {}", String::from_utf8(msg).unwrap());
        }
        _ => {}
    }
}
