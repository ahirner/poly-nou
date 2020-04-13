# poly-nou
**poly-nou** is an experimental integration of [nphysics2d](https://nphysics.org)
and [nannou](https://nannou.cc). After `cargo run`, randomly generated
polygons will hit the ground and each other.

![screenshot](https://raw.githubusercontent.com/ahirner/poly-nou/master/media/screenshot.png)
You can *shake* them up by moving the window!

Conversion traits in `src/geometry.rs` facilitate interoperability. However, use
at your own risk and don't expect idiomatic **rust** just yet!

## Todos
- Break up concave polygons into [compound shapes](https://ncollide.org/geometric_representations/#compound)
- Generalize shape drawing
- Add more collision shapes
- Add shaders that signify motion