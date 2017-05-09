//! LZO is a compression library with focus on decompression speed.
//! minilzo is a lightweight subset of the full LZO library.
//!
//! It is available [online](http://www.oberhumer.com/opensource/lzo/#minilzo).
//!
//! This rust library is a wrapper around the minilzo library
//! and is fully compatible with LZO/minilzo compressed data.
//!
//! # Basic Operation
//!
//! ```rust,no_run
//! # use minilzo;
//! let data = b"foobar";
//!
//! let compressed = minilzo::compress(&data[..]).unwrap();
//!
//! let decompressed = minilzo::decompress(&compressed, data.len()).unwrap();
//! ```

extern crate minilzo_sys;
extern crate libc;

use std::mem::size_of;
use std::ptr;

use libc::{c_int, c_short, c_long};
use minilzo_sys::{
    // Types
    lzo_uint,
    lzo_callback_t,

    // Helpers
    LZO1X_1_MEM_COMPRESS,
    lzo_version,
    __lzo_init_v2,

    // (De)compress
    lzo1x_1_compress,
    lzo1x_decompress_safe,
};

/// Errors of Compression or Decompression
///
/// These are the same as minilzo returns,
/// just nicely wrapped as an enum.
#[derive(Debug, PartialEq)]
pub enum Error {
    Error,
    OutOfMemory,
    NotCompressible,
    InputOverrun,
    OutputOverrun,
    LookbehindOverrun,
    EOFNotFound,
    InputNotConsumed,
    NotYetImplemented,
    InvalidArgument,
    InvalidAlignment,
    OutputNotConsumed,
    InternalError,
}

impl Error {
    pub fn from_code(code: i32) -> Error {
        match code {
             -1 => Error::Error,
             -2 => Error::OutOfMemory,
             -3 => Error::NotCompressible,
             -4 => Error::InputOverrun,
             -5 => Error::OutputOverrun,
             -6 => Error::LookbehindOverrun,
             -7 => Error::EOFNotFound,
             -8 => Error::InputNotConsumed,
             -9 => Error::NotYetImplemented,
            -10 => Error::InvalidArgument,
            -11 => Error::InvalidAlignment,
            -12 => Error::OutputNotConsumed,
            -99 => Error::InternalError,
            _ => Error::Error,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::OutOfMemory => "out of memory",
            Error::NotCompressible => "not compressible",
            Error::InputOverrun => "input overrun",
            Error::OutputOverrun => "output overrun",
            Error::LookbehindOverrun => "lookbehind overrun",
            Error::EOFNotFound => "EOF not found",
            Error::InputNotConsumed => "input not consumed",
            Error::NotYetImplemented => "not yet implemented",
            Error::InvalidArgument => "invalid argument",
            Error::InvalidAlignment => "invalid alignment",
            Error::OutputNotConsumed => "output not consumed",
            Error::InternalError => "internal error",
            Error::Error => "error",
        }
    }
}

fn _lzo_init() -> i32 {
    unsafe {
        __lzo_init_v2(lzo_version(),
                      size_of::<c_short>() as c_int,
                      size_of::<c_int>() as c_int,
                      size_of::<c_long>() as c_int,
                      size_of::<u32>() as c_int, // lzo_uint32_t
                      size_of::<lzo_uint>() as c_int,
                      size_of::<usize>() as c_int, // lzo_sizeof_dict_t
                      size_of::<usize>() as c_int, // char*
                      size_of::<usize>() as c_int, // lzo_voidp
                      size_of::<lzo_callback_t>() as c_int
                     )
    }
}

/// Compress the given data, if possible.
/// An error will be returned if compression fails.
///
/// Example
///
/// ```rust
/// let data = b"foobar";
/// let compressed = minilzo::compress(&data[..]);
/// ```
pub fn compress(indata: &[u8]) -> Result<Vec<u8>, Error> {
    let mut wrkmem : [u8; LZO1X_1_MEM_COMPRESS] = unsafe {
        std::mem::uninitialized()
    };

    let inlen = indata.len();
    let outlen = inlen + inlen / 16 + 64 + 3;
    let mut outdata = Vec::with_capacity(outlen);

    unsafe {
        let r = lzo1x_1_compress(
            indata.as_ptr(),
            inlen as lzo_uint,
            outdata.as_mut_ptr(),
            &outlen as *const _ as *mut _,
            wrkmem.as_mut_ptr() as *mut _);

        if r == 0 {
            outdata.set_len(outlen);
            return Ok(outdata)
        }

        return Err(Error::from_code(r))
    }
}

/// Decompress the given data, if possible.
/// An error will be returned if decompression fails.
///
/// The length of the output buffer can be specified.
/// If the output buffer is not large enough to hold the decompressed data,
/// a `OutputOverrun` is returned.
///
/// If an error in the compressed data is detected, a minilzo error is returned.
///
/// Example:
///
/// ```rust,no_run
/// let data = b"[your-compressed-data]";
/// let decompressed = minilzo::decompress(&data[..], 100);
/// ```
pub fn decompress(indata: &[u8], newlen: usize) -> Result<Vec<u8>, Error> {
    let inlen = indata.len();
    let mut outdata = Vec::with_capacity(newlen);

    unsafe {
        let r = lzo1x_decompress_safe(
            indata.as_ptr(),
            inlen as lzo_uint,
            outdata.as_mut_ptr(),
            &newlen as *const _ as *mut _,
            ptr::null_mut());

        if r == 0 {
            outdata.set_len(newlen);
            return Ok(outdata)
        }

        return Err(Error::from_code(r))
    }
}

#[test]
fn init() {
    // We test this, but we don't export it to the user right now
    assert_eq!(0, _lzo_init());
}

#[test]
fn test_compress_short() {
let data = "foo".as_bytes();
    let compressed = compress(data).unwrap();
    assert_eq!(compressed.len(), 7);

    let decompressed = decompress(&compressed, 3).unwrap();
    assert_eq!(decompressed.len(), 3);
    assert_eq!(decompressed, data);
}

#[test]
fn test_compress_fails_with_short_output() {
    let data = [0; 128*1024];
    let compressed = compress(&data[..]).unwrap();

    assert_eq!(Err(Error::OutputOverrun),
               decompress(&compressed, 128));
}

#[test]
fn simple_compress_decompress() {
    let data = [0; 128*1024];
    let compressed = compress(&data[..]).unwrap();

    assert_eq!(593, compressed.len());

    let decompressed = decompress(&compressed, 128*1024).unwrap();
    assert_eq!(128*1024, decompressed.len());
}

#[test]
fn test_compress_decompress_lorem_round() {
    let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod \
                 tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At \
                 vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, \
                 no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit \
                 amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut \
                 labore et dolore magna aliquyam erat, sed diam voluptua.";

    let compressed = compress(lorem.as_bytes()).unwrap();
    let decompressed = decompress(&compressed, lorem.len()).unwrap();

    assert_eq!(lorem.len(), decompressed.len());
    assert_eq!(lorem.as_bytes(), &decompressed[..]);
}

#[test]
fn test_alice_wonderland_both() {
    let alice = "\r\n\r\n\r\n\r\n                ALICE'S ADVENTURES IN WONDERLAND\r\n";

    let compressed = compress(alice.as_bytes()).unwrap();
    let decompressed = decompress(&compressed, alice.len()).unwrap();

    assert_eq!(alice.len(), decompressed.len());
    assert_eq!(alice.as_bytes(), &decompressed[..]);
}
