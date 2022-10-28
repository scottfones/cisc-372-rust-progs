# Breakout 07: p2p5

## Commands

To run the code with four processors and pass in "3 1 4 1 5":
```bash
cargo mpirun -n 4  --oversubscribe --bin p2p5  -- 3 1 4 1 5
```

## Directions

Write an MPI program "args.c" in which proc 0 reads the command
line args, both argc and argv. Proc 0 first sends to all other procs
the number of args (i.e., argc - 1). Then it sends the args themselves
to all other procs, one at a time. All procs of positive rank receive
these messages, then print a message like this:

Proc 7: the 3 args are: arg1 arg2 arg3

You can assume 256 is a safe upper bound on the length of any one
command line argument. Note that positive rank procs are not allowed
to look at argc and argv in this exercise! (In general, you are only
guaranteed that process 0 can access the command line arguments,
though many MPI implementations will allow other processes to also
access those arguments.)

