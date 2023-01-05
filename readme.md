# Yet another terbilang 🤷

Converts `u32` number to said Indonesian.

```rust
use yaterbilang as terbilang;

assert_eq!(terbilang::from(0), "nol");
assert_eq!(terbilang::from(11), "sebelas");
assert_eq!(terbilang::from(32), "tiga puluh dua");
assert_eq!(terbilang::from(998), "sembilan ratus sembilan puluh delapan");
```

Version `0.1.1` only supports up to 9.999 at the moment.
