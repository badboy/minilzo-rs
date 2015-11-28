#[allow(non_camel_case_types)]
mod minilzo;

/* Manually added */
pub const LZO1X_1_MEM_COMPRESS : usize = (16384 * 8);
pub const LZO1X_MEM_COMPRESS : usize = LZO1X_1_MEM_COMPRESS;

pub const LZO_E_OK : i32                 = 0;
pub const LZO_E_ERROR : i32              = -1;
pub const LZO_E_OUT_OF_MEMORY : i32      = -2;    /* [lzo_alloc_func_t failure] */
pub const LZO_E_NOT_COMPRESSIBLE : i32   = -3;    /* [not used right now] */
pub const LZO_E_INPUT_OVERRUN: i32       = -4;
pub const LZO_E_OUTPUT_OVERRUN: i32      = -5;
pub const LZO_E_LOOKBEHIND_OVERRUN: i32  = -6;
pub const LZO_E_EOF_NOT_FOUND: i32       = -7;
pub const LZO_E_INPUT_NOT_CONSUMED: i32  = -8;
pub const LZO_E_NOT_YET_IMPLEMENTED: i32 = -9;    /* [not used right now] */
pub const LZO_E_INVALID_ARGUMENT: i32    = -10;
pub const LZO_E_INVALID_ALIGNMENT: i32   = -11;   /* pointer argument is not properly aligned */
pub const LZO_E_OUTPUT_NOT_CONSUMED: i32 = -12;
pub const LZO_E_INTERNAL_ERROR: i32      = -99;

pub use minilzo::*;
