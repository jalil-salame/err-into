# Error Into

Extension traits for [`core::convert::Into`](https://doc.rust-lang.org/core/convert/trait.Into.html)
that allow `Result` and `Option` to convert their contained values inline.

## Usage

Import the traits and you are done:

```rust
use err_into::*;
// use err_into::MapInto;    // .map(Into::into)                     -> .map_into()
// use err_into::ErrorInto;  // .map_err(Into::into)                 -> .err_into()
// use err_into::ResultInto; // .map(Into::into).map_err(Into::into) -> .res_into()
```

Look at the [documentation](https://docs.rs/err-into/latest/err-into) for more information.
