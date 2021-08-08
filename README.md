# `veccentric`

A small 2D vector library inspired by [p5.Vector](https://p5js.org/reference/#/p5.Vector). Meant to be used in small gamedev projects.

```rust
use veccentric::Vecctor;

let a = Vecctor::new(3.0, 4.0);
assert_eq!(a.mag(), 5.0);

let five_a = a * 5.0;
assert_eq!(five_a.mag(), 25.0);

let b = Vecctor::new(-3.0, 0.0);
let c = a + b; // (0, 4.0)
assert_eq!(c.angle(), PI / 2.0);
```
