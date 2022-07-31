# Convi - convenient (but safe) conversion (`From`-like) traits for Rust


Tired of the pain of casting `usize` and other types in Rust safely? This crate is here to help.

By enabling cargo features on this crate like this:

```norust
convi = { version = "*", features = [ "min_target_pointer_width_32" ] }
```

you can mark your code as not compatible with architectures with
pointer size less than 32 bits. This will enable additional infallible `usize`
conversions like:

```ignore
use convi::CastFrom;


fn main() {
  let some_u32 = 3u32;
  println!("{}",  usize::cast_from(some_u32));
}
```

and display appropriate error message if anyone
tries to build your code for unsupported architecture.

In addition, for times where you know that your value
will always be smaller than the given type, instead of
`u32::try_from(some_usize).expect("must not fail")`,
you can:

```
use convi::ExpectFrom;

fn main() {
  let some_u32 = 3u32;
  println!("{}",  usize::expect_from(some_u32));
}
```


Zero dependencies, small code, [review it yourself](./src/lib.rs).
