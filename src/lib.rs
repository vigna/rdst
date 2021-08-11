//! # rdst
//!
//! rdst is a flexible native Rust implementation of unstable radix sort.
//!
//! ## Usage
//!
//! In the simplest case, you can use this sort by simply calling `my_vec.radix_sort_unstable()`. If you have a custom type to sort, you may need to implement `RadixKey` for that type.
//!
//! ## Default Implementations
//!
//! `RadixKey` is implemented for `Vec` of the following types out-of-the-box:
//!
//!  * `u8`
//!  * `u16`
//!  * `u32`
//!  * `u64`
//!  * `u128`
//!  * `usize`
//!  * `[u8; N]`
//!
//! The default implementations can be disabled by disabling default features on the crate.
//!
//! ### Implementing `RadixKey`
//!
//! To be able to sort custom types, implement `RadixKey` as below.
//!
//!  * `LEVELS` should be set to the total number of bytes you will consider for each item being sorted
//!  * `get_level` should return the corresponding bytes from the most significant byte to the least significant byte
//!
//! Notes:
//! * This allows you to implement radix keys that span multiple values, or to implement radix keys that only look at part of a value.
//! * You should try to make this as fast as possible, so consider using branchless implementations wherever possible
//!
//! ```ignore
//! use rdst::RadixKey;
//!
//! impl RadixKey for u16 {
//!     const LEVELS: usize = 2;
//!
//!     #[inline]
//!     fn get_level(&self, level: usize) -> u8 {
//!         let b = self.to_le_bytes();
//!
//!         match level {
//!             0 => b[1],
//!             _ => b[0],
//!         }
//!     }
//! }
//! ```
//!
//! #### Partial `RadixKey`
//!
//! If you know your type has bytes that will always be zero, you can skip those bytes to speed up the sorting process. For instance, if you have a `u32` where values never exceed `10000`, you only need to consider two of the bytes. You could implement this as such:
//!
//! ```ignore
//! impl RadixKey for u32 {
//!     const LEVELS: usize = 2;
//!
//!     #[inline]
//!     fn get_level(&self, level: usize) -> u8 {
//!         (self >> ((Self::LEVELS - 1 - level) * 8)) as u8
//!     }
//! }
//! ```
//!
//! Note that to replace the default implementations provided by the crate, you must disable the default crate features.
//!
//! #### Multi-value `RadixKey`
//!
//! If your type has multiple values you need to search by, simply create a `RadixKey` that spans both values.
//!
//! ```ignore
//! impl RadixKey for MyStruct {
//!     const LEVELS: usize = 4;
//!
//!     #[inline]
//!     fn get_level(&self, level: usize) -> u8 {
//!         match level {
//!           0 => self.key_1[0],
//!           1 => self.key_1[1],
//!           2 => self.key_2[0],
//!           3 => self.key_2[1],
//!         }
//!     }
//! }
//! ```
//!
//! ## License
//!
//! Licensed under either of
//!
//! * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ### Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

#[cfg(test)]
mod tests;

mod lsb_radix_sort;
mod msb_ska_sort;
mod radix_key;
#[cfg(feature = "default-implementations")]
mod radix_key_impl;
mod scanning_radix_sort;
mod utils;

use crate::lsb_radix_sort::lsb_radix_sort_bucket;
use crate::scanning_radix_sort::scanning_radix_sort_bucket;
pub use radix_key::RadixKey;

// Exposed for benchmarking
#[cfg(feature = "bench")]
pub use utils::*;

#[cfg(not(feature = "bench"))]
use crate::utils::par_get_msb_counts;

fn radix_sort_bucket_start<T>(bucket: &mut [T])
where
    T: RadixKey + Sized + Send + Ord + Copy + Sync,
{
    if bucket.len() < 2 {
        return;
    } else if bucket.len() < 32 {
        bucket.sort_unstable();
        return;
    }

    if bucket.len() >= 300_000 {
        let msb_counts = par_get_msb_counts(bucket);
        scanning_radix_sort_bucket(bucket, msb_counts);
    } else {
        lsb_radix_sort_bucket(bucket, T::LEVELS - 1, 0);
    }
}

fn radix_sort_inner<T>(bucket: &mut [T])
where
    T: RadixKey + Sized + Send + Ord + Copy + Sync,
{
    if T::LEVELS == 0 {
        panic!("RadixKey must have at least 1 level");
    }

    radix_sort_bucket_start(bucket);
}

pub trait RadixSort {
    /// radix_sort_unstable runs the actual radix sort based upon the `rdst::RadixKey` implementation
    /// of `T` in your `Vec<T>` or `[T]`.
    fn radix_sort_unstable(&mut self);
}

impl<T> RadixSort for Vec<T>
where
    T: RadixKey + Sized + Send + Ord + Copy + Sync,
{
    fn radix_sort_unstable(&mut self) {
        radix_sort_inner(self);
    }
}

impl<T> RadixSort for [T]
where
    T: RadixKey + Sized + Send + Ord + Copy + Sync,
{
    fn radix_sort_unstable(&mut self) {
        radix_sort_inner(self);
    }
}
