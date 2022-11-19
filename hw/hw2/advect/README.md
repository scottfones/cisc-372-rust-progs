# Advect

While this program is sequential, it is the foundation for several future assignments. Its corresponding library, [heatmap_anim](https://github.com/scottfones/cisc-372-rust-progs/tree/main/libs/heatmap_anim), is responsible for generating the output, an animated heat map.

## Summary

This program simulates the diffusion and advection of heat through a one-dimensional, metal rod with cyclic boundary conditions. Given a discretized model of the rod as an array $u$ of length $n$, we define $u'$ as values at the next time step according to,

$$u'_i = u_i + k(u_{i+1} + u_{i-1} - 2u_i) - c(u_{i+1} - u_{i-1})$$

where:

- $k$ is a diffusion constant
- $c$ is an advection constant
