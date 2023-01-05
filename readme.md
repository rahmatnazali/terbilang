# Yet another terbilang 🤷

Converts `u64` number to said Indonesian.

```rust
use yaterbilang as terbilang;

assert_eq!(terbilang::from(0), "nol");
assert_eq!(terbilang::from(11), "sebelas");
assert_eq!(terbilang::from(32), "tiga puluh dua");
assert_eq!(terbilang::from(998), "sembilan ratus sembilan puluh delapan");
```

> Version `0.1.5` supports up to 999.999.999.999 (triliun).

This is just a brute force way to exhaustively determine all the possible outcome.
Given that the possible input is finite and deterministic, we think that this one should not differ much in terms of 
performance-wise when compared to other "smarter" solution. It just works.

The source code should also be relatively dead simple and verbose to ease any future changes.
