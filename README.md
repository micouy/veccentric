# `veccentric`

[![docs.rs](https://docs.rs/veccentric/badge.svg)](https://docs.rs/veccentric) [![crates.io](https://img.shields.io/badge/crates.io-veccentric-green.svg)](https://crates.io/crates/veccentric)

[![demo](/assets/heccentric.gif)](/examples/heccentric.rs)

Tiny 2D vector library. Inspired by [p5.js](https://p5js.org/)'s
[`p5.Vector`](https://p5js.org/reference/#/p5.Vector).

The main type, `Vecc<T>`, is a generic struct implementing
many useful traits and operator overloading. `Fecc` is a type alias for
`Vecc<f64>`. It has an extended API, heavily inspired by [`p5.Vector`](https://p5js.org/reference/#/p5.Vector).


# Examples

Basic arithmetic on `Vecc<T>`.

```rust
use veccentric::Vecc;

let a = Vecc::new(3_i32, 4);
let b = a * 5;
let c = Vecc::new(-10, -8);
let d = b - c;
let e = -d;
```

`Fecc`'s extended API.

```rust
use veccentric::Fecc;

let a: Fecc = (3.0, 4.0).into();
let b = a / 0.2;
let c = b.limit(20.0);
let d = c.rotate(PI);
let e = d.turn(0.0);
```

[See more examples.](examples/)
