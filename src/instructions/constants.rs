#![allow(dead_code)]

pub const STACK_POINTER_REG: usize = 15;

// these values are borrowed from Musashi
// and not yet fully understood
// Matt - I believe these are setup for best perf on 8 bit operations
//        They don't really make sense as most operations require a shift anyway?
pub const SFLAG_SET: u32 = 0x2000;
pub const MFLAG_SET: u32 = 0x1000;
pub const XFLAG_SET: u32 = 0x100;
pub const ZFLAG_SET: u32 = 0x00;
pub const NFLAG_SET: u32 =  0x80;
pub const VFLAG_SET: u32 =  0x80;
pub const CFLAG_SET: u32 = 0x100;
pub const CPU_SFLAG_MASK: u16 = 0xdfff;     /* -- -- S  -- -- -- -- -- -- -- -- -- -- -- -- -- */
pub const CPU_MFLAG_MASK: u16 = 0xdfff;     /* -- -- -- M- -- -- -- -- -- -- -- -- -- -- -- -- */
pub const CPU_TFLAG_MASK: u16 = 0xdfff;     /* T1 T0 -- -- -- -- -- -- -- -- -- -- -- -- -- -- */
pub const CPU_SR_MASK: u16 = 0xf71f;        /* T1 T2 S  M  -- I2 I1 I0 -- -- -- X  N  Z  V  C  */
pub const CPU_SR_INT_MASK: u32 = 0x0700;    /* -- -- -- -- -- I2 I1 I0 -- -- -- -- -- -- -- -- */
pub const CPU_CACR_MASK: u32 = 0x000f;      /* -- -- -- -- -- -- -- -- -- -- -- -- C  CE F  E  */
pub const CPU_CAAR_MASK: u32 = 0x00ff;      /* -- -- -- -- -- -- -- -- I5 I4 I3 I2 I1 I0 -- -- */

pub const SFLAG_BIT: u16 = 13;
pub const MFLAG_BIT: u16 = 12;
pub const TFLAG_BITS: u16 = 14;
pub const INT_BITS: u16 = 8;

pub const VFLAG_CLEAR: u32 =  0x00;
pub const XFLAG_CLEAR: u32 =  0x00;
pub const NFLAG_CLEAR: u32 =  0x00;
pub const CFLAG_CLEAR: u32 =  0x00;
pub const SFLAG_CLEAR: u32 =  0x00;
pub const MFLAG_CLEAR: u32 =  0x00;
pub const ZFLAG_CLEAR: u32 =  0xffffffff; // used as "non-z-flag"

// Control Registers
// '010+
pub const SFC:  u16 = 0x000;
pub const DFC:  u16 = 0x001;
pub const USP:  u16 = 0x800;
pub const VBR:  u16 = 0x801;
// '020+
pub const CACR: u16 = 0x002;
pub const CAAR: u16 = 0x802;
pub const MSP:  u16 = 0x803;
pub const ISP:  u16 = 0x804;
// '040+
// TODO !

// Exception Vectors
//pub const EXCEPTION_BUS_ERROR: u8               =  2;
pub const EXCEPTION_ADDRESS_ERROR: u8           =  3;
pub const EXCEPTION_ILLEGAL_INSTRUCTION: u8     =  4;
pub const EXCEPTION_ZERO_DIVIDE: u8             =  5;
pub const EXCEPTION_CHK: u8                     =  6;
pub const EXCEPTION_TRAPV: u8                   =  7;
pub const EXCEPTION_PRIVILEGE_VIOLATION: u8     =  8;
// pub const EXCEPTION_TRACE: u8                   =  9;
pub const EXCEPTION_UNIMPLEMENTED_1010: u8      = 10;
pub const EXCEPTION_UNIMPLEMENTED_1111: u8      = 11;
// pub const EXCEPTION_FORMAT_ERROR: u8            = 14;
// pub const EXCEPTION_UNINITIALIZED_INTERRUPT: u8 = 15;
// pub const EXCEPTION_SPURIOUS_INTERRUPT: u8      = 24;
// pub const EXCEPTION_INTERRUPT_AUTOVECTOR: u8    = 24;
pub const EXCEPTION_TRAP_BASE: u8               = 32;

