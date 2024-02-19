# Jagua-rs

This crate contains everything necessary to solve 2D irregular cutting and packing problems without the combinatorial decision-making (i.e. which items to place where).

It provides all necessary entities and components to create a dynamic model of a 2D irregular C&P instance and provide a collision detection engine to check the feasibility of a placement.

`jagua-rs` is a standalone algorithm, but rather is designed to be used as a building block by optimization algorithms which employ `jagua-rs` to provide efficient collision detection.
This optimization algorithm would tackle combinatorial challenge, while `jagua-rs` would handle the geometric challenge.

See [lbf crate](../lbf) for a reference implementation of an optimization algorithm making use of `jagua-rs`.

## Design Goals

- **General purpose** 
  - [x] Bin- and strip-packing problems
  - [x] Irregular shaped items and bins
  - [x] Continuous rotation and translation of items
  - [x] Support for holes and quality zones in the bin
- **Robust**
  - [x] Performs collision detection using the polygonal representation of shapes
  - [x] Mimics the results of a naive trigonometric approach, but much faster
  - [x] Avoids infeasibility due floating point arithmetic errors by erring on the side of caution in edge cases
- **Adaptable** 
  - [x] Unifying concept of `Hazards` allows for easy extension of the engine to incorporate extra *spatial* constraints
  - [x] Define new C&P problems by creating a custom `Instance` and accompanying `Problem` variants
- **Performant**
  - [x] Focus on maximum `query` and `update` performance
  - [x] Able to resolve millions of collision queries per second
  - [x] Simplify polygons in preprocessing

## Documentation

The code is documented with rustdoc.
The docs can be build using `cargo doc --open` from the root of the repository.

## Testing

`jagua-rs` contains a suite of assertions which are enabled by default in debug builds to ensure the correctness of the engine.
These assertions are disabled in release builds to maximize performance.

TODO: unit testing