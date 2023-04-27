# HW4: Advect 2D 

## Summary

This program simulates the diffusion and advection of heat through a two-dimensional metal sheet with cyclic boundary conditions. Given a discretized model of the sheet as an array $u$ of length $n$, we define $u'$ with values at the next time step according to,

```rust
u_new[i][j] = u[i][j]
    + K * (u[(i + 1) % N][j]
         + u[(i + N - 1) % N][j]
         + u[i][(j + 1) % N]
         + u[i][(j + N - 1) % N]
         - 4.0 * u[i][j])
   - C * (u[(i + 1) % N][j] 
        - u[(i + N - 1) % N][j]
        + u[i][(j + 1) % N]
        - u[i][(j + N - 1) % N]);
```

where:

- $K$ is a diffusion constant
- $C$ is an advection constant

In this simulation, the middle two-thirds of the sheet begin at 100 degrees while the surrounding sheet is at 0 degrees.

## Commands

To run (on 6 procs):

```bash
cargo mpirun -n 6 --bin advect2d --release
```

Output:

[hw4_advect2d_anim.webm](https://user-images.githubusercontent.com/17322143/234488525-9f08061f-536e-4a81-a5c0-7e21299c3942.webm)

(Optional) To convert the gif to a video:
```bash
sh ./gif_to_webm.sh
```
