# Breakout 07: p2p1

## Commands

To run the code with four processors:
```bash
cargo mpirun -n 4  --oversubscribe --bin p2p1
```

## Directions

Write an MPI program p2p1.c in which process 1 sends a message to
process 3. The message is the single int 497. Process 3 receives the
message and prints: "Received from proc 3: X", where X is the int
received. Other processes do nothing.
