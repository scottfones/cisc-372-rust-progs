#!/bin/bash
hyperfine -L nprocs 1,2,4,6,8,10 'cargo mpirun -n {nprocs} --oversubscribe --release --bin pi_rect_mpi' --export-json pi_strong.json
python plot_parametrized.py pi_strong.json
