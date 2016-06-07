//
//  SOS: the Stupid Operating System
//  by Hawk Weisman (hi@hawkweisman.me)
//
//  Copyright (c) 2015 Hawk Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//
//! Parsing and loading Executable and Linkable Format (ELF) 32- and 64-bit
//! binaries.
//!
//! For more information on the ELF format, refer to:
//!  - [Wikipedia](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
//!  - The [OS Dev Wiki](http://wiki.osdev.org/ELF)
//!  - The [ELF Format Specification](http://www.skyfree.org/linux/references/ELF_Format.pdf)
use core::mem;
use core::intrinsics;
use core::slice;

pub mod section;
pub mod file;

pub type Section<'a> = section::Header<'a>;
pub type FileHeader = file::Header;
pub type ElfResult<T> = Result<T, &'static str>;

/// A handle on an ELF binary
#[derive(Debug)]
pub struct Binary<'a> {
    pub header: &'a FileHeader
  , binary: &'a [u8]
}

/// if `n` == 0, this will give you an `&[]`. just a warning.
//  thanks to Max for making  me figure this out.
unsafe fn extract_from_slice<T: Sized>( data: &[u8]
                                      , offset: usize
                                      , n: usize)
                                      -> &[T] {
    assert!( data.len() - offset >= mem::size_of::<T>() * n
           , "Slice too short to contain {} objects of type {}"
           , n
           , intrinsics::type_name::<T>()
           );
    slice::from_raw_parts(data[offset..].as_ptr() as *const T, n)
}
