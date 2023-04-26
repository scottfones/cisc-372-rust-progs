# HW4: Advect 2D 

## Summary

This program simulates the diffusion and advection of heat through a two-dimensional metal sheet with cyclic boundary conditions. Given a discretized model of the rod as an array $u$ of length $n$, we define $u'$ with values at the next time step according to,

where:

- $k$ is a diffusion constant
- $c$ is an advection constant

In this simulation, the middle two-thirds of the sheet begin at 100 degrees while the surrounding sheet is at 0 degrees.

## Commands

To run (on 6 procs):

```bash
cargo mpirun -n 6 --bin advect2d --release
```

Output:

<video src='../../../imgs/hw4_advect2d_anim.webm' />

(Optional) To convert the gif to a video:
```bash
sh ./gif_to_webm.sh
```
