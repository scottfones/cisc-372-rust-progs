# Advect

While this program is sequential, it is the foundation for several future assignments. Its corresponding library, [heatmap_anim](../../../libs/heatmap_anim/), is responsible for generating the output, an animated heat map.

## Summary

This program simulates the diffusion and advection of heat through a one-dimensional metal rod with cyclic boundary conditions. Given a discretized model of the rod as an array $u$ of length $n$, we define $u'$ with values at the next time step according to,

$$u_{i}' = u_i + k(u_{i+1} + u_{i-1} - 2u_i) - c(u_{i+1} - u_{i-1})$$

where:

- $k$ is a diffusion constant
- $c$ is an advection constant

In this simulation, the middle 50% of the rod begins at 100 degrees while the surrounding rod is at 0 degrees.

## Commands

To run:

```bash
cargo run --release
```

Output:

![animated heat map](../../../imgs/hw2_advect_anim.gif)

(Optional) To convert the gif to a video:
```bash
sh ./gif_to_webm.sh
```
