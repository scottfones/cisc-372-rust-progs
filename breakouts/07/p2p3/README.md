# Breakout 07: p2p3

## Commands

To run the code with four processors:
```bash
cargo mpirun -n 4  --oversubscribe --bin p2p3
```

## Directions

Write an MPI program p2p3.c in which every process creates a random
int. (Note: you will need to set the random seed differently on each
process, else they will all generate the same sequence of "random"
ints. That's how a pseudo-random number generator works. Man srand()
and rand().) Every process other than process 0 sends its int to
process 0. Process 0 prints out these ints in order, starting with its
own, as follows:

  Process 0 has: X0
  Process 1 has: X1
  ...
  Process np-1 has: Xnp-1

where X0, X1, ..., Xnp-1 are the actual ints. Note that process 0 is
the only process that prints. This is the only way to guarantee the
output is correct.

