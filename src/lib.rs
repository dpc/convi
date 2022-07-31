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

impl<I> CastInto<I> for I  where I: CastFrom<Self> {
    fn cast_into(self) -> I {
        I::cast_from(self)
    }
}

impl<F, T> CastFrom<F> for T where T : From<F> {
    fn cast_from(other: F) -> Self {
        Self::from(other)
    }
}

/// Checked cast like `TryFrom` but panicking, instead of returning an error.
///
/// Let's face it, usually you know that the cast won't fail, but
/// just don't want to risk that you've got something wrong and thus corrupt
/// the result. So you can't use `as`, or [`From`], but don't want
/// to type `u64::try_from(x).expect("can't fail");` over and over.
///
/// Literally implemented for all `TryFrom` impls like this:
///
/// ```ignore
/// fn cast_from_checked(other: F) -> Self {
///     Self::try_from(other).expect("data conversion invariant")
/// }
/// ```
pub trait CheckedFrom<I> {
    fn checked_from(other: I) -> Self;
}

impl<F, T> CheckedFrom<F> for T where T : TryFrom<F> , <T as TryFrom<F>>::Error : std::fmt::Debug {
    fn checked_from(other: F) -> Self {
        Self::try_from(other).expect("data conversion invariant")
    }
}

#[allow(unused)]
macro_rules! impl_cast_into {
    // `()` indicates that the macro takes no argument.
    ($from:ty, $into:ty) => {

        impl CastInto<$into> for $from {
            fn cast_into(self) -> $into  {
                self as $into
            }
        }
    };
}

#[cfg(all(min_target_pointer_width_16, target_pointer_width = "8"))]
compile_error!("One of the dependencies of `convi` requires at least 16 bit architecture target.");
#[cfg(all(min_target_pointer_width_32, any(target_pointer_width = "8", target_pointer_width = "16")))]
compile_error!("One of the dependencies of `convi` requires at least 32 bit architecture target.");
#[cfg(all(min_target_pointer_width_64, any(target_pointer_width = "8", target_pointer_width = "16", target_pointer_width = "32")))]
compile_error!("One of the dependencies of `convi` requires at least 64 bit architecture target.");
#[cfg(all(min_target_pointer_width_128, any(target_pointer_width = "8", target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("One of the dependencies of `convi` requires at least 64 bit architecture target.");

// #[cfg(all(target_pointer_width = "8", any(min_target_pointer_width_16, min_target_pointer_width_32, min_target_pointer_width_64, min_target_pointer_width_128)]
// LOL, copy&paste, but whatever - cleanup later, PRs welcome
#[cfg(any(min_target_pointer_width_128))]
impl_cast_into!(u128, usize);
#[cfg(any(min_target_pointer_width_128))]
impl_cast_into!(i128, isize);
#[cfg(any(min_target_pointer_width_128))]
impl_cast_into!(u64, isize);

#[cfg(any(min_target_pointer_width_64, min_target_pointer_width_128))]
impl_cast_into!(u64, usize);
#[cfg(any(min_target_pointer_width_64, min_target_pointer_width_128))]
impl_cast_into!(i64, isize);
#[cfg(any(min_target_pointer_width_64, min_target_pointer_width_128))]
impl_cast_into!(u32, isize);

#[cfg(any(min_target_pointer_width_32, min_target_pointer_width_64, min_target_pointer_width_128))]
impl_cast_into!(u32, usize);
#[cfg(any(min_target_pointer_width_32, min_target_pointer_width_64, min_target_pointer_width_128))]
impl_cast_into!(i32, isize);
#[cfg(any(min_target_pointer_width_32, min_target_pointer_width_64, min_target_pointer_width_128))]
impl_cast_into!(u16, isize);

#[cfg(any(min_target_pointer_width_16, min_target_pointer_width_32, min_target_pointer_width_64, min_target_pointer_width_128))]
impl_cast_into!(u16, usize);
#[cfg(any(min_target_pointer_width_16, min_target_pointer_width_32, min_target_pointer_width_64, min_target_pointer_width_128))]
impl_cast_into!(i16, isize);
#[cfg(any(min_target_pointer_width_16, min_target_pointer_width_32, min_target_pointer_width_64, min_target_pointer_width_128))]
impl_cast_into!(u8, isize);

impl_cast_into!(u8, usize);
impl_cast_into!(i8, isize);
