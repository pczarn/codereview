hi, and welcome to the Rust community!

Your code is quite good. There are a couple areas where the code could be improved, but not by much. It would be helpful if you implement more code for review all at once, for example datetime parsing code.

## `DateTime::create`

```rust
pub fn create(
        year: impl Into<u16>,
        month: impl Into<u8>,
        day: impl Into<u8>,
        hour: impl Into<u8>,
        minute: impl Into<u8>,
        second: impl Into<u8>
)
```

The function name is not descriptive. It is idiomatic to name constructors e.g. `with_date_and_time`.

I do not like that your arguments are `impl Into<u16>` or `impl Into<u8>`. Why is it better to accept precisely `u16` or `u8` here? See the following error to figure out why. Very little convenience is lost -- the caller can do `.into()` if needed.

```rust
// error[E0283]: type annotations needed
//     --> src/main.rs:17:16
//     |
// 17 |     let time = Time::new("03".parse().unwrap(), "51".parse().unwrap(), "00".parse().unwrap());
//     |                ^^^^^^^^^ cannot infer type for type parameter `impl Into<u8>` declared on the associated function `new`
//     |
//     = note: cannot satisfy `_: Into<u8>`
// note: required by a bound in `Time::new`
let time = Time::new("03".parse().unwrap(), "51".parse().unwrap(), "00".parse().unwrap());
println!("Time: {}", time);
```

Basically, `impl Trait` in argument position does not inform type inference. Some functions such as `str::parse` return generic results parameterized by a free parameter, so it may be best to guide inference.

Accepting `impl Trait` in argument position is usually unidiomatic.

Furthermore, `impl Into<u16>` is why you have to write `2022u16` and can't write just `2022`.

So we have:

```rust
pub fn with_date_and_time(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8
)
```

## `isoformat`

I recommend a better method name such as `to_iso_format` or `to_rfc3339`. Name `to_rfc3339` is consistent with chrono's DateTime.

## more

Your code is not formatted with rustfmt. I recommend `cargo fmt`.

For lints, I recommend `cargo clippy`.

## github


