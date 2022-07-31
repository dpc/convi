# Convi - convenient (but safe) conversion (`From`-like) traits for Rust


Tired of the pain of casting `usize` in Rust? This crate is here to help.

First, by enabling cargo features on this crate like this:

```
convi = { version = "*", version = [ "min_target_ptr_width_32"] }
```

you can mark your code as not compatible with architectures with
pointer size less than 32 bits. This will enable conversions like:

```
use convi::CastFrom;


fn main() {
  let some_u32 = 3u32;
  println!("{}",  usize::cast_from(some_u32);
}
```

and display appropriate error message if anyone
tries to build your code for unsupported architecture.

In addition, for times where you know that your value
will always be smaller than the given type, instead of
`u32::try_from(some_usize).expect("must not fail")`,
you can:

```
use convi::CheckedFrom;

fn main() {
  let some_u32 = 3u32;
  println!("{}",  usize::checked_from(some_u32);
}
```


Zero dependencies, small code, [review it yourself](./src/lib.rs).
