#!/bin/bash
rm -f pi_rect_mpi.dat
for i in 1 2 4 6 8 10; do cargo mpirun -n $i --oversubscribe --release --bin pi_rect_mpi >> pi_rect_mpi.dat; done;
gnuplot pi_rect_mpi.gnu
