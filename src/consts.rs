#![allow(dead_code)]

pub const R_MOS_IMM8: u32 = 1;
pub const R_MOS_ADDR8: u32 = 2;
pub const R_MOS_ADDR16: u32 = 3;
// The least significant byte of a 16-bit address. All MOS processors are little endian, so this would also be the leftmost byte in a 16-bit address.
pub const R_MOS_ADDR16_LO: u32 = 4;
// The most significant byte of a 16-bit address.
pub const R_MOS_ADDR16_HI: u32 = 5;
// An 8-bit PC-relative jump value, calculated from the end of the instruction. From the beginning of the instruction, the jump range is [-126, +129]. Used for the B?? series of MOS instructions.
pub const R_MOS_PCREL_8: u32 = 6;
pub const R_MOS_ADDR24: u32 = 7;
// A 24-bit address, used in the 65816 series.
pub const R_MOS_ADDR24_BANK: u32 = 8;
pub const R_MOS_ADDR24_SEGMENT: u32 = 9;
// The 8-bit bank value from a 24-bit address. The most significant byte.
pub const R_MOS_ADDR24_SEGMENT_LO: u32 = 10;
// The 16-bit segment value from a 24-bit address.
pub const R_MOS_ADDR24_SEGMENT_HI: u32 = 11;
// The low byte from the segment value in a 24-bit address.
pub const R_MOS_FK_DATA_4: u32 = 12;
// The high byte from the segment value in a 24-bit address.
pub const R_MOS_FK_DATA_8: u32 = 13;
// A generic four-byte, 32-bit value. Although MOS does not use native 32-bit relocations, DWARF may use this relocation type when describing debug information.
pub const R_MOS_ADDR_ASCIZ: u32 = 14;
// A generic eight-byte, 64-bit value. Although MOS does not use native 64-bit relocations, DWARF may use this relocation type when describing debug information.
