# checkdigit

A small Rust library and CLI for the check digits on books and barcodes:
ISBN-10, ISBN-13, UPC-A, EAN-8, and EAN-13.

These formats exist so a single mistyped or misscanned digit gets caught
before it turns into the wrong book showing up at a warehouse, or the wrong
price ringing up at a register. Two different algorithms cover all five
formats:

- **ISBN-10** uses weights 10 down to 1 across its ten digits, summed mod 11.
  Because the remainder can be 10, the check digit position is allowed to
  hold `X` instead of a numeral - nowhere else in the code.
- **ISBN-13, EAN-13, EAN-8, and UPC-A** all use the same GS1 algorithm:
  starting from the digit just left of the check digit and moving left,
  weights alternate 3, 1, 3, 1, ... summed mod 10. ISBN-13 is exactly EAN-13
  with a `978` or `979` prefix, so it needs no special case at all.

## Library usage

```rust
use checkdigit::{gs1, isbn10};

// ISBN-10: compute a check digit, or validate a full code.
assert_eq!(isbn10::check_digit("012345678").unwrap(), '9');
assert!(isbn10::validate("0123456789").unwrap());

// ISBN-13 / EAN-13 / EAN-8 / UPC-A: same functions, one algorithm.
assert_eq!(gs1::check_digit("978030640615").unwrap(), '7');
assert!(gs1::validate("9780306406157").unwrap());
```

Both modules return a `Result` with an error describing exactly what was
wrong with the input (wrong length, or a character that wasn't a digit),
rather than silently returning `false`.

## CLI usage

```
$ cargo run -- validate 978-0-306-40615-7
valid

$ cargo run -- validate 0-13-468599-8
invalid

$ cargo run -- check-digit 978030640615
9780306406157

$ cargo run -- check-digit 012345678
0123456789

$ cargo run -- batch < codes.txt
978-0-306-40615-7: valid
0-13-468599-8: invalid
036000291459: invalid
```

`validate` picks the algorithm from the cleaned length of the input (10
digits for ISBN-10, 8/12/13/14 for the GS1-based formats). Hyphens and
whitespace are stripped before that check, so codes can be pasted in as
printed on the book or package.

`batch` runs the same check over a whole list of codes, one per line, read
from a file if given (`checkdigit batch codes.txt`) or from stdin otherwise.
Blank lines and lines starting with `#` are skipped. It exits successfully
only if every code in the list was valid.

## Building

```
cargo build --release
```

No third-party dependencies; the standard library is all this needs.

## License

MIT, see `LICENSE`.
