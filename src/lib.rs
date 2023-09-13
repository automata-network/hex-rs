
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "tstd")]
#[macro_use]
extern crate sgxlib as std;

mod hexbytes;
pub use hexbytes::*;

pub use std_hex::*;