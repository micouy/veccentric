# `veccentric`

2D vector library inspired by [p5.js](https://p5js.org/)'s
[`p5.Vector`](https://p5js.org/reference/#/p5.Vector), meant to be used in small gamedev projects.

The main type, [`Vecc`](crate::vecc::Vecc), is a generic struct implementing
many useful traits. [`Fecc`](crate::fecc::Fecc) is a type alias for
`Vecc<f64>`. Its API is heavily inspired by [`p5.Vector`](https://p5js.org/reference/#/p5.Vector)'s,
although some of the methods are named differently.

[Docs](https://docs.rs/veccentric)


# Examples

Basic arithmetic.

```rust
use veccentric::Vecc;

let a = Vecc::new(3_i32, 4);
let b = a * 5; // (15, 20)
let c = Vecc::new(-10, -8);
let d = b - c; // (5, 12)
let e = -d; // (-5, -12)
```

```rust
use veccentric::Vecc;

let a: Vecc<i32> = (10, 5).into();
```

```rust
use veccentric::Fecc;

let a: Fecc = (3.0, 4.0).into();
let b = a / 0.2; // (15.0, 20.0)
let c = b.limit(20.0); // (12.0, 16.0)
let d = c.rotate(PI); // (-12.0, -16.0)
let e = d.turn(0.0); // (20.0, 0.0)
```
