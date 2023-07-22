#![no_std]
//! A `no_std` library to simpify chaining methods when you are returning a [`Result`]. It is a
//! trivial library which sould be compatible with all environments.
//!
//! This is specially noticeable when using crates like [`anyhow`](https://crates.io/crates/anyhow)
//! which provide a "catch all" error type, so you need to convert all errors you recieve.
//!
//! It is also helpful when you have many custom errors constructed with
//! [`thiserror`](https://crates.io/crates/thiserror) or
//! [`justerror`](https://crates.io/crates/justerror), or use many libraries with different error
//! types.
//!
//! # Usage
//!
//! Import the traits and you can benefit from it immediately:
//!
//! ```rust
//! use err_into::MapInto;
//! use err_into::ErrorInto;
//! use err_into::ResultInto;
//!
//! // ...
//!
//! let _: Option<i32> = Some(0u8).map_into();
//! let _: Result<i32, ()> = Ok(0u8).map_into();
//! let _: Result<(), i32> = Err(0u8).err_into();
//! let _: Result<u16, i32> = (if false { Ok(0u8) } else { Err(0i8) }).res_into();
//! ```
//!
//! ## Motivating example
//!
//! This is slightly contrived because I don't want to depend on any libraries but showcases where
//! `err_into` excels:
//!
//! ```rust
//! use err_into::ErrorInto;
//!
//! fn process(val: u16) -> Result<u8, u8> {
//!     if val > 255 {
//!         Err((val >> 8) as u8)
//!     } else {
//!         Ok(val as _)
//!     }
//! }
//!
//! fn long_chain() -> Result<u8, i32> {
//!     (0u16..16u16).map(|x| x * x * x * x)
//!         .filter(|x| x % 2 == 0)
//!         .map(process)
//!         .next()
//!         .unwrap_or(Err(0))
//!         .err_into()
//! }
//!
//! fn long_chain_no_err_into() -> Result<u8, i32> {
//!     // May be confusing
//!     (0u16..16u16).map(|x| x * x * x * x)
//!         .filter(|x| x % 2 == 0)
//!         .map(process)
//!         .next()
//!         .unwrap_or(Err(0))
//!         .map_err(Into::into)
//! }
//!
//! fn long_chain_no_map_err() -> Result<u8, i32> {
//!     // Please don't do this
//!     Ok((0u16..16u16).map(|x| x * x * x * x)
//!         .filter(|x| x % 2 == 0)
//!         .map(process)
//!         .next()
//!         .unwrap_or(Err(0))?)
//! }
//! ```

/// Maps an error using [`Into::into`]
///
/// Short version of `Result::map_err(self, Into::into)` that simplifies operation chains like
///
/// ```rust
/// use err_into::ErrorInto;
///
/// fn get_data() -> Result<(), u8> {
///     Err(0u8)
/// }
///
/// fn handle_data_question_mark() -> Result<(), i32> {
///     // Can't be split into multiple lines
///     Ok(get_data()?)
/// }
///
/// fn handle_data_map_err() -> Result<(), i32> {
///     // Slightly harder to read
///     get_data().map_err(Into::into)
/// }
///
/// fn handle_data_err_into() -> Result<(), i32> {
///     get_data().err_into()
/// }
///
/// assert_eq!(handle_data_err_into(), handle_data_question_mark());
/// assert_eq!(handle_data_err_into(), handle_data_map_err());
/// ```
pub trait ErrorInto<T, E> {
    fn err_into(self) -> Result<T, E>;
}

/// Maps both the Value and the Error of a [`Result`] using [`Into::into`]
///
/// Shorthand for `result.map(Into::into).map_err(Into::into)`
///
/// ```rust
/// use err_into::ResultInto;
///
/// let res: Result<u8, i8> = Ok(0);
/// let _: Result<i32, i16> = res.res_into();
/// ```
pub trait ResultInto<T, E> {
    fn res_into(self) -> Result<T, E>;
}

/// Maps a value using [`Into::into`]
///
/// Shorthand for `Option::map(self, Into::into)` and `Result::map(self, Into::into)`
///
/// ```rust
/// use err_into::MapInto;
///
/// let value = Some(0u8);
/// let map_into: Option<i32> = value.map_into();
/// let map_into_std: Option<i32> = value.map(Into::into);
/// assert_eq!(map_into, map_into_std);
///
/// let result = Ok(0u8);
/// let map_into: Result<i32, ()> = result.map_into();
/// let map_into_std: Result<i32, ()> = result.map(Into::into);
/// assert_eq!(map_into, map_into_std);
/// ```
pub trait MapInto<T> {
    fn map_into(self) -> T;
}

impl<T, E, F> ErrorInto<T, E> for Result<T, F>
where
    F: Into<E>,
{
    fn err_into(self) -> Result<T, E> {
        self.map_err(Into::into)
    }
}

impl<T, U, E, F> ResultInto<T, E> for Result<U, F>
where
    F: Into<E>,
    U: Into<T>,
{
    fn res_into(self) -> Result<T, E> {
        self.map(Into::into).map_err(Into::into)
    }
}

impl<T, U, E> MapInto<Result<U, E>> for Result<T, E>
where
    T: Into<U>,
{
    fn map_into(self) -> Result<U, E> {
        self.map(Into::into)
    }
}

impl<T, U> MapInto<Option<U>> for Option<T>
where
    T: Into<U>,
{
    fn map_into(self) -> Option<U> {
        self.map(Into::into)
    }
}
