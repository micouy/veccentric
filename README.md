# `veccentric`

[![docs.rs](https://docs.rs/veccentric/badge.svg)](https://docs.rs/veccentric) [![crates.io](https://img.shields.io/badge/crates.io-veccentric-green.svg)](https://crates.io/crates/veccentric)

Tiny 2D vector library. Inspired by [p5.js](https://p5js.org/)'s
[`p5.Vector`](https://p5js.org/reference/#/p5.Vector).


# Usage

Add `veccentric` to your `Cargo.toml`.

```toml
[dependencies]
veccentric = "0.3"
```

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

[See more examples.](https://github.com/micouy/veccentric/tree/master/examples)
