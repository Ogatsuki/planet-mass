# planet-mass

This is a physics simulation created for studying Rust that calculates the mass of a planet.
Based on several parameters specified in the code, it approximates a planet as a collection of thin spherical shells and calculates its total mass.

## Overview

This program assumes that the internal density of a planet changes linearly from the center toward the outside.
It divides the planet along the radial direction using a fixed step size, then accumulates the mass of each spherical shell from its volume and density to calculate the final total mass.

It also displays the result as a ratio to Earth's mass (`Mass Ratio`).

## Input Parameters

You can enter the following values when running the program. If you leave an entry blank or enter an invalid value, the default value will be used.

| Item | Description | Default Value |
| --- | --- | --- |
| `rho_o` | Core density `The core density[g/cm^3]` | `4.5` |
| `rho_outer` | Outer-layer density `The outer density[g/cm^3]` | `2.0` |
| `r` | Planet radius `The radius[km]` | `6340.0` |
| `step` | Calculation step size `The step size[km]` | `1.0` |

## How to Run

```bash
cargo run
```

When you run it, you will be prompted to enter the parameters one by one.

## Output

- `Total mass`: The total mass of the planet calculated by the simulation
- `Mass Ratio`: The ratio relative to Earth's mass (`5.9742e24`)

## Calculation Method

1. Divide the planet's radius into intervals of `step`
2. Linearly interpolate the density at each radial position
3. Calculate the spherical shell volume for each interval
4. Calculate the mass of each interval using density × volume
5. Sum the masses of all intervals

## Notes

- This is currently a simple implementation for learning purposes
- The density distribution assumes a linear change
- There is still room for improvement in how input values and units are handled

## Technologies Used

- Rust
- Interactive CLI using standard input and output
