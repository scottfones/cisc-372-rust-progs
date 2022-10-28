# Breakout 07: p2p2

## Commands

To run the code with four processors:
```bash
cargo mpirun -n 4  --oversubscribe --bin p2p2
```

## Directions

Write an MPI program p2p2.c which is same as above, except the
message sent is the string "Greetings to proc 3 from proc 1!".
