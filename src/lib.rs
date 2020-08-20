//! Unsafe versions of standard library `From<T>` and `Into<T>`.

/// Unsafe version of `Into<T>` trait from `std`.
///
/// Prefer implementing `UnsafeFrom` because it gives you `UnsafeInto` freely.
///
/// # Example
/// ```
/// use unsafe_from::UnsafeInto;
///
/// struct MyUnsafeType(i32);
///
/// unsafe impl UnsafeInto<MyUnsafeType> for i32 {
///     unsafe fn unsafe_into(self) -> MyUnsafeType {
///         MyUnsafeType(self)
///     }
/// }
/// ```
pub unsafe trait UnsafeInto<T>: Sized {
    unsafe fn unsafe_into(self) -> T;
}

unsafe impl<T, U> UnsafeInto<U> for T
    where
        U: UnsafeFrom<T>,
{
    unsafe fn unsafe_into(self) -> U {
        U::unsafe_from(self)
    }
}

/// Unsafe version of `From<T>` trait from `std`.
///
/// Implementing this trait also gives you `UnsafeInto` freely.
///
/// # Example
/// ```
/// use unsafe_from::UnsafeFrom;
///
/// struct MyUnsafeType(i32);
///
/// unsafe impl UnsafeFrom<i32> for MyUnsafeType {
///     unsafe fn unsafe_from(t: i32) -> MyUnsafeType {
///         MyUnsafeType(t)
///     }
/// }
/// ```
pub unsafe trait UnsafeFrom<T>: Sized {
    unsafe fn unsafe_from(t: T) -> Self;
}

unsafe impl<T> UnsafeFrom<T> for T {
    unsafe fn unsafe_from(t: T) -> Self {
        t
    }
}