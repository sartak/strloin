# strloin

`strloin` gives you copy-on-write
([cow](https://doc.rust-lang.org/std/borrow/enum.Cow.html)) slices of a string.
If the provided ranges form a single contiguous region, then you'll get back a
borrowed slice of the string. Otherwise, you'll get back an owned concatenation
of each range. Note that this crate is intended for cases where borrowing is
far more common than cloning. If that's not the case, it's likely that the
overhead is not worth it and you should consider unconditionally cloning.

The basic functionality looks like this:

```rust
use strloin::Strloin;

let strloin = Strloin::new("hello world");

assert_eq!(strloin.from_ranges(&[0..5]), "hello"); // borrowed
assert_eq!(strloin.from_ranges(&[0..5, 5..11]), "hello world"); // borrowed
assert_eq!(strloin.from_ranges(&[0..5, 6..11]), "helloworld"); // owned
```
