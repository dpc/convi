#![doc = include_str!("../README.md")]


/// Like `From`, but defined for extra types
///
/// Depending on some cargo features being enabled on this
/// crate (like minimum target pointer width), this
/// enables extra available conversions for types
/// implementing [`std::convert::From`]
pub trait CastFrom<I> {
    fn cast_from(other: I) -> Self;
}

/// Like [`Into`] but for [`CastFrom`].
pub trait CastInto<I> {
    fn cast_into(self) -> I;
}

impl<F, I> CastInto<I> for F where I: CastFrom<Self> {
    fn cast_into(self) -> I {
        I::cast_from(self)
    }
}

// WELP: this is not going to fly...
// impl<F, T> CastFrom<F> for T where T : From<F> {
//     fn cast_from(other: F) -> Self {
//         Self::from(other)
//     }
// }

/// Expect cast like `TryFrom` but panicking, instead of returning an error.
///
/// Let's face it, usually you know that the cast won't fail, but
/// just don't want to risk that you've got something wrong and thus corrupt
/// the result. So you can't use `as`, or [`From`], but don't want
/// to type `u64::try_from(x).expect("can't fail");` over and over.
///
/// Literally implemented for all `TryFrom` impls like this:
///
/// ```ignore
/// fn expect_from(other: F) -> Self {
///     Self::try_from(other).expect("data conversion invariant")
/// }
/// ```
pub trait ExpectFrom<I> {
    fn expect_from(other: I) -> Self;
}

impl<F, T> ExpectFrom<F> for T where T : TryFrom<F> , <T as TryFrom<F>>::Error : std::fmt::Debug {
    fn expect_from(other: F) -> Self {
        Self::try_from(other).expect("data conversion invariant")
    }
}

#[allow(unused)]
macro_rules! impl_cast_into {
    ($from:ty, $into:ty) => {

        impl CastFrom<$from> for $into {
            fn cast_from(v: $from) -> $into  {
                v as $into
            }
        }
    };
}

#[cfg(all(feature = "min_target_pointer_width_16", target_pointer_width = "8"))]
compile_error!("One of the dependencies of `convi` requires at least 16 bit architecture target.");
#[cfg(all(feature = "min_target_pointer_width_32", any(target_pointer_width = "8", target_pointer_width = "16")))]
compile_error!("One of the dependencies of `convi` requires at least 32 bit architecture target.");
#[cfg(all(feature = "min_target_pointer_width_64", any(target_pointer_width = "8", target_pointer_width = "16", target_pointer_width = "32")))]
compile_error!("One of the dependencies of `convi` requires at least 64 bit architecture target.");
#[cfg(all(feature = "min_target_pointer_width_128", any(target_pointer_width = "8", target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("One of the dependencies of `convi` requires at least 128 bit architecture target.");

// #[cfg(all(target_pointer_width = "8", any(min_target_pointer_width_16, min_target_pointer_width_32, min_target_pointer_width_64, min_target_pointer_width_128)]
// LOL, copy&paste, but whatever - cleanup later, PRs welcome
#[cfg(any(feature = "min_target_pointer_width_128"))]
mod impls_128 {
    use super::*;

    impl_cast_into!(u128, usize);
    impl_cast_into!(i128, isize);
    impl_cast_into!(u64, isize);
}

#[cfg(any(feature = "min_target_pointer_width_64", feature = "min_target_pointer_width_128"))]
mod impls_64 {
    use super::*;

    impl_cast_into!(u64, usize);
    impl_cast_into!(i64, isize);
    impl_cast_into!(u32, isize);
}


#[cfg(any(feature = "min_target_pointer_width_32", feature = "min_target_pointer_width_64", feature = "min_target_pointer_width_128"))]
mod impls_32 {
    use super::*;

    impl_cast_into!(u32, usize);
    impl_cast_into!(i32, isize);
    impl_cast_into!(u16, isize);
}

#[cfg(any(feature = "min_target_pointer_width_16", feature = "min_target_pointer_width_32", feature = "min_target_pointer_width_64", feature = "min_target_pointer_width_128"))]
mod impls_16 {
    use super::*;

    impl_cast_into!(u16, usize);
    impl_cast_into!(i16, isize);
    impl_cast_into!(u8, isize);

    impl_cast_into!(u8, usize);
    impl_cast_into!(i8, isize);
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

    #[allow(unused_macros)]
    macro_rules! cast_from_word_size_16 {
        () => {
            #[allow(dead_code)] // We only check if this builds.
            fn can_cast_from_when_wordsize_is_16() {
                let x = 1_u8;
                let _ = usize::cast_from(x);
                let _ = isize::cast_from(x);
                let _: usize = x.cast_into();
                let _: isize = x.cast_into();

                let x = 1_i8;
                let _ = isize::cast_from(x);
                let _: isize = x.cast_into();

                let x = 1_i16;
                let _ = isize::cast_from(x);
                let _: isize = x.cast_into();

                let x = 1_u16;
                let _ = usize::cast_from(x);
                let _: usize = x.cast_into();
            }
        }
    }

    #[allow(unused_macros)]         // Only used if wordsize is >= 32
    macro_rules! cast_from_word_size_32 {
        () => {
            #[allow(dead_code)] // We only check if this builds.
            fn can_cast_from_when_wordsize_is_32() {
                let x = 1_u16;
                let _ = isize::cast_from(x);
                let _: isize = x.cast_into();

                let x = 1_i32;
                let _ = isize::cast_from(x);
                let _: isize = x.cast_into();

                let x = 1_u32;
                let _ = usize::cast_from(x);
                let _: usize = x.cast_into();
            }
        }
    }

    #[allow(unused_macros)]         // Only used if wordsize is >= 64
    macro_rules! cast_from_word_size_64 {
        () => {
            #[allow(dead_code)] // We only check if this builds.
            fn can_cast_from_when_wordsize_is_64() {
                let x = 1_u32;
                let _ = isize::cast_from(x);
                let _: isize = x.cast_into();

                let x = 1_i64;
                let _ = isize::cast_from(x);
                let _: isize = x.cast_into();

                let x = 1_u64;
                let _ = usize::cast_from(x);
                let _: usize = x.cast_into();
            }
        }
    }

    #[test]
    #[cfg(feature = "min_target_pointer_width_16")]
    fn can_cast_from_when_wordsize_is_16() {
        cast_from_word_size_16!();
    }

    #[test]
    #[cfg(feature = "min_target_pointer_width_32")]
    fn can_cast_from_when_wordsize_is_32() {
        cast_from_word_size_16!();
        cast_from_word_size_32!();
    }

    #[test]
    #[cfg(feature = "min_target_pointer_width_64")]
    fn can_cast_from_when_wordsize_is_64() {
        cast_from_word_size_16!();
        cast_from_word_size_32!();
        cast_from_word_size_64!();
    }
}
