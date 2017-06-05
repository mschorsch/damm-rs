# damm-rs
_"In error detection, the Damm algorithm is a check digit algorithm that detects all single-digit errors and all adjacent transposition errors."_ ([Wikipedia](https://en.wikipedia.org/wiki/Damm_algorithm))

# example
```rust
use damm_rs::encode;

let number = "572";
assert_eq!(encode(number), Some(4));
```