//! # Pascal boot blocks
//! 
//! Blocks are grabbed from UCSD Pascal 1.2 disk 1 (APPLE1).


pub const PASCAL_525_BLOCK0: [u8;512] = [
    0x01, 0xE0, 0x60, 0xF0, 0x03, 0x4C, 0xE3, 0x08, 0xAD, 0x00, 0x08, 0xC9, 0x04, 0xB0, 0x0A, 0x69,
    0x02, 0x8D, 0x00, 0x08, 0xE6, 0x3D, 0x4C, 0x5C, 0xC6, 0xA9, 0x00, 0x8D, 0x78, 0x04, 0xA9, 0x0A,
    0x85, 0x0E, 0xA9, 0x80, 0x85, 0x3F, 0x85, 0x11, 0xA9, 0x00, 0x85, 0x10, 0xA9, 0x08, 0x85, 0x02,
    0xA9, 0x02, 0x85, 0x0F, 0xA9, 0x00, 0x20, 0xFF, 0x08, 0xA2, 0x4E, 0xA0, 0x06, 0xB1, 0x10, 0xD9,
    0xD0, 0x08, 0xF0, 0x27, 0x18, 0xA5, 0x10, 0x69, 0x1A, 0x85, 0x10, 0x90, 0x02, 0xE6, 0x11, 0xCA,
    0xD0, 0xE9, 0xC6, 0x0E, 0xD0, 0xCC, 0x20, 0xF8, 0x08, 0xAD, 0xE8, 0xC0, 0xA2, 0x00, 0xBD, 0xC1,
    0x08, 0x20, 0xFD, 0xFB, 0xE8, 0xE0, 0x15, 0xD0, 0xF5, 0xF0, 0xFE, 0xC8, 0xC0, 0x13, 0xD0, 0xCD,
    0xAD, 0x81, 0xC0, 0xAD, 0x81, 0xC0, 0xA9, 0xD0, 0x85, 0x3F, 0xA9, 0x30, 0x85, 0x02, 0xA0, 0x00,
    0xB1, 0x10, 0x85, 0x0F, 0xC8, 0xB1, 0x10, 0x20, 0xFF, 0x08, 0xAD, 0x89, 0xC0, 0xA9, 0xD0, 0x85,
    0x3F, 0xA9, 0x10, 0x85, 0x02, 0xA0, 0x00, 0xB1, 0x10, 0x18, 0x69, 0x18, 0x85, 0x0F, 0xC8, 0xB1,
    0x10, 0x69, 0x00, 0x20, 0xFF, 0x08, 0xAD, 0x80, 0xC0, 0x6C, 0xF8, 0xFF, 0xCD, 0xD5, 0xD3, 0xD4,
    0xA0, 0xC2, 0xCF, 0xCF, 0xD4, 0xA0, 0xC6, 0xD2, 0xCF, 0xCD, 0xA0, 0xD3, 0xCC, 0xCF, 0xD4, 0xA0,
    0xB6, 0xCE, 0xCF, 0xA0, 0xC6, 0xC9, 0xCC, 0xC5, 0xA0, 0xD3, 0xD9, 0xD3, 0xD4, 0xC5, 0xCD, 0xAE,
    0xC1, 0xD0, 0xD0, 0xCC, 0xC5, 0xA0, 0x0C, 0x53, 0x59, 0x53, 0x54, 0x45, 0x4D, 0x2E, 0x41, 0x50,
    0x50, 0x4C, 0x45, 0xBD, 0x88, 0xC0, 0x20, 0xF8, 0x08, 0xA2, 0x00, 0xBD, 0xAC, 0x08, 0x20, 0xFD,
    0xFB, 0xE8, 0xE0, 0x15, 0xD0, 0xF5, 0xF0, 0xFE, 0xA9, 0x0A, 0x4C, 0x24, 0xFC, 0xEA, 0xEA, 0x4A,
    0x08, 0xA5, 0x0F, 0x29, 0x07, 0x0A, 0x85, 0x00, 0xA5, 0x0F, 0x28, 0x6A, 0x4A, 0x4A, 0x85, 0xF0,
    0xA9, 0x00, 0x85, 0x3E, 0x4C, 0x2B, 0x09, 0xA6, 0x02, 0xF0, 0x22, 0xC6, 0x02, 0xE6, 0x3F, 0xE6,
    0x00, 0xA5, 0x00, 0x49, 0x10, 0xD0, 0x04, 0x85, 0x00, 0xE6, 0xF0, 0xA4, 0x00, 0xB9, 0x3E, 0x09,
    0x85, 0xF1, 0xA2, 0x00, 0xE4, 0x02, 0xF0, 0x05, 0x20, 0x4E, 0x09, 0x90, 0xDA, 0x60, 0x00, 0x02,
    0x04, 0x06, 0x08, 0x0A, 0x0C, 0x0E, 0x01, 0x03, 0x05, 0x07, 0x09, 0x0B, 0x0D, 0x0F, 0xA2, 0x60,
    0xA5, 0xF0, 0x0A, 0x0E, 0x78, 0x04, 0x20, 0x56, 0x0A, 0x4E, 0x78, 0x04, 0x20, 0xFA, 0x09, 0xB0,
    0xFB, 0xA4, 0x2E, 0x8C, 0x78, 0x04, 0xC4, 0xF0, 0xD0, 0xE6, 0xA5, 0x2D, 0xC5, 0xF1, 0xD0, 0xEC,
    0x20, 0x92, 0x09, 0xB0, 0xE7, 0x20, 0x7A, 0x09, 0x18, 0x60, 0xA0, 0x00, 0xA2, 0x56, 0xCA, 0x30,
    0xFB, 0xB9, 0x00, 0x02, 0x5E, 0x00, 0x03, 0x2A, 0x5E, 0x00, 0x03, 0x2A, 0x91, 0x3E, 0xC8, 0xD0,
    0xED, 0x60, 0xA0, 0x20, 0x88, 0xF0, 0x61, 0xBD, 0x8C, 0xC0, 0x10, 0xFB, 0x49, 0xD5, 0xD0, 0xF4,
    0xEA, 0xBD, 0x8C, 0xC0, 0x10, 0xFB, 0xC9, 0xAA, 0xD0, 0xF2, 0xA0, 0x56, 0xBD, 0x8C, 0xC0, 0x10,
    0xFB, 0xC9, 0xAD, 0xD0, 0xE7, 0xA9, 0x00, 0x88, 0x84, 0x26, 0xBC, 0x8C, 0xC0, 0x10, 0xFB, 0x59,
    0xD6, 0x02, 0xA4, 0x26, 0x99, 0x00, 0x03, 0xD0, 0xEE, 0x84, 0x26, 0xBC, 0x8C, 0xC0, 0x10, 0xFB,
    0x59, 0xD6, 0x02, 0xA4, 0x26, 0x99, 0x00, 0x02, 0xC8, 0xD0, 0xEE, 0xBC, 0x8C, 0xC0, 0x10, 0xFB,
    0xD9, 0xD6, 0x02, 0xD0, 0x13, 0xBD, 0x8C, 0xC0, 0x10, 0xFB, 0xC9, 0xDE, 0xD0, 0x0A, 0xEA, 0xBD,
    0x8C, 0xC0, 0x10, 0xFB, 0xC9, 0xAA, 0xF0, 0x5C, 0x38, 0x60, 0xA0, 0xFC, 0x84, 0x26, 0xC8, 0xD0
];

pub const PASCAL_525_BLOCK1: [u8;512] = [
    0x04, 0xE6, 0x26, 0xF0, 0xF3, 0xBD, 0x8C, 0xC0, 0x10, 0xFB, 0xC9, 0xD5, 0xD0, 0xF0, 0xEA, 0xBD,
    0x8C, 0xC0, 0x10, 0xFB, 0xC9, 0xAA, 0xD0, 0xF2, 0xA0, 0x03, 0xBD, 0x8C, 0xC0, 0x10, 0xFB, 0xC9,
    0x96, 0xD0, 0xE7, 0xA9, 0x00, 0x85, 0x27, 0xBD, 0x8C, 0xC0, 0x10, 0xFB, 0x2A, 0x85, 0x26, 0xBD,
    0x8C, 0xC0, 0x10, 0xFB, 0x25, 0x26, 0x99, 0x2C, 0x00, 0x45, 0x27, 0x88, 0x10, 0xE7, 0xA8, 0xD0,
    0xB7, 0xBD, 0x8C, 0xC0, 0x10, 0xFB, 0xC9, 0xDE, 0xD0, 0xAE, 0xEA, 0xBD, 0x8C, 0xC0, 0x10, 0xFB,
    0xC9, 0xAA, 0xD0, 0xA4, 0x18, 0x60, 0x86, 0x2B, 0x85, 0x2A, 0xCD, 0x78, 0x04, 0xF0, 0x48, 0xA9,
    0x00, 0x85, 0x26, 0xAD, 0x78, 0x04, 0x85, 0x27, 0x38, 0xE5, 0x2A, 0xF0, 0x37, 0xB0, 0x07, 0x49,
    0xFF, 0xEE, 0x78, 0x04, 0x90, 0x05, 0x69, 0xFE, 0xCE, 0x78, 0x04, 0xC5, 0x26, 0x90, 0x02, 0xA5,
    0x26, 0xC9, 0x0C, 0xB0, 0x01, 0xA8, 0x20, 0xA7, 0x0A, 0xB9, 0xC8, 0x0A, 0x20, 0xB7, 0x0A, 0xA5,
    0x27, 0x29, 0x03, 0x0A, 0x05, 0x2B, 0xAA, 0xBD, 0x80, 0xC0, 0xB9, 0xD4, 0x0A, 0x20, 0xB7, 0x0A,
    0xE6, 0x26, 0xD0, 0xBF, 0x20, 0xB7, 0x0A, 0xAD, 0x78, 0x04, 0x29, 0x03, 0x0A, 0x05, 0x2B, 0xAA,
    0xBD, 0x81, 0xC0, 0xA6, 0x2B, 0x60, 0xEA, 0xA2, 0x11, 0xCA, 0xD0, 0xFD, 0xE6, 0x46, 0xD0, 0x02,
    0xE6, 0x47, 0x38, 0xE9, 0x01, 0xD0, 0xF0, 0x60, 0x01, 0x30, 0x28, 0x24, 0x20, 0x1E, 0x1D, 0x1C,
    0x1C, 0x1C, 0x1C, 0x1C, 0x70, 0x2C, 0x26, 0x22, 0x1F, 0x1E, 0x1D, 0x1C, 0x1C, 0x1C, 0x1C, 0x1C,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];