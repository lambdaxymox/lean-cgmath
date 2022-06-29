/*!
# Computer Graphics Linear Algebra Library

## Introduction
**cglinalg** is a low-dimensional linear algebra library aimed at 
real-time computer graphics, game engine development, and real-time collision 
detection. This library provides a strongly typed system for computer graphics 
applications.

The library is designed with the following goals in mind:
* **Ergonomics** -- The system should be easy to understand and use.
* **Cross Platform** -- It should be portable to other ecosystems like 
  other C/C++ and Rust libraries. Every data type can be treated like a 
  fixed-sized array so they can be sent to across FFI boundaries.
* **Few Dependencies** -- The library should be relatively self-contained. To 
  support portability and maintainability, **cglinalg** is designed with few 
  external dependencies. The biggest dependencies---`proptest` and 
  `criterion`---are development dependencies only. Also, the library only uses
  stable Rust.
* **Type Safety** -- Leverage Rust's type system and zero-cost abstractions 
  to ensure code correctness, abstraction, and intelligibility do not come 
  at the cost of performance.
* **Flexibility** -- The library should serve as a type-agnostic cornerstone 
  for computer graphics applications. The data types in **cglinalg** are
  generic over their scalars so they can operate on multiple scalar types.
* **Speed And Efficiency** -- Operations should be fast and efficient. SIMD 
  instructions and architecture specific optimizations should be used where 
  possible. Moreover the library should compile fast.

## Getting Started
To use the library in your project, add **cglinalg** as a dependency in your 
`Cargo.toml` file:
```ignore
[dependencies]
cglinalg = "0.16.1"
```
After that, place the crate declaration in either your `lib.rs` file or 
your `main.rs` file
```rust
extern crate cglinalg;
```

## Features
**cglinalg** is a low-dimensional linear-algebra library aimed at specific 
application domains that make heavy use of computer graphics. It includes the 
most common linear algebra operations for implementing rendering algorithms, 
real-time collision detection, etc. All data types are designed to be exportable 
to external interfaces such as foreign function interfaces or external hardware. 
This serves the **cglinalg** goal to be a platform agnostic foundation for 
computer graphics applications in other languages and ecosystems as well. 
Specific features of the library include:
* Basic linear algebra with matrices and vectors up to dimension four.
* Quaternions, Euler angles, and rotation matrices for doing rotations.
* All data types are parametrized to work over a large range of numerical types.
* An optional transformation system for working with affine and projective 
  transformations on points and vectors. This library distinguishes points from 
  vectors and locations in space vs. displacements in space. This matters when 
  working with working with affine transformations in homogeneous coordinates.
* Transformations including translation, reflection, shear, scale, 
  and rotation operations.
* Orthographic projections and perspective projections for camera models.
* Typed angles and typed angle trigonometry that statically guarantee that 
  trigonometry is done in the right units.
* The library makes heavy use of property testing via the `proptest` crate
  in addition to Rust's type system to ensure code correctness.
* The library makes heavy use of statistical microbenchmarking using the 
  `criterion` crate.

## Limitations On The Design
The library has design limitations for a number of reasons. 
* **cglinalg** is designed specifically with graphics applications in mind, which 
  tend to be mathematically simpler than other modeling and simulation applications. 
  As a consequence this library does not support most of the operations commonly used 
  in modeling and simulation tasks. Moreover it can only do linear algebra up to 
  dimension four. It is a Rust counterpart to `DirectXMath` or `glm`.
* In keeping with simplicity as one of the project goals, the underlying storage of 
  all data types in this library are statically allocated arrays. This is advantageous 
  in the low-dimensional case when the data types have small sizes, but this is a 
  limitation in the higher-dimensional case where dynamic storage allocation of storage 
  or using the heap may be desirable.

*/

// #![no_std]
extern crate core;

extern crate approx;
extern crate num_traits;


mod common;

mod angle;
mod matrix;
mod vector;

mod euler;
mod projection;
mod point;
mod quaternion;
mod complex;

mod isometry;
mod reflection;
mod rotation;
mod scale;
mod shear;
mod translation;
mod transform;
mod similarity;


pub use common::*;

pub use angle::*;
pub use euler::*;
pub use matrix::*;
pub use projection::*;
pub use quaternion::*;
pub use vector::*;
pub use point::*;
pub use complex::*;

pub use isometry::*;
pub use reflection::*;
pub use rotation::*;
pub use scale::*;
pub use shear::*;
pub use translation::*;
pub use transform::*;
pub use similarity::*;

