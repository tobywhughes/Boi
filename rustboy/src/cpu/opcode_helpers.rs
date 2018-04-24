pub fn cycle_parse(opcode: u8) -> u8
{
    match opcode
    {
        0x00 => 1,
        0x01 => 3,
        0x02 => 2,
        0x03 => 2,
        0x04 => 1,
        0x05 => 1,
        0x06 => 2,
        0x07 => 1,
        0x08 => 5,
        0x09 => 2,
        0x0A => 2,
        0x0B => 2,
        0x0C => 1,
        0x0D => 1,
        0x0E => 2,
        0x0F => 1,

        0x10 => 1,
        0x11 => 3,
        0x12 => 2,
        0x13 => 2,
        0x14 => 1,
        0x15 => 1,
        0x16 => 2,
        0x17 => 1,
        0x18 => 3,
        0x19 => 2,
        0x1A => 2,
        0x1B => 2,
        0x1C => 1,
        0x1D => 1,
        0x1E => 2,
        0x1F => 1,

        0x20 => 2, //Branch
        0x21 => 3,
        0x22 => 2,
        0x23 => 2,
        0x24 => 1,
        0x25 => 1,
        0x26 => 2,
        0x27 => 1,
        0x28 => 2, //Branch
        0x29 => 2,
        0x2A => 2,
        0x2B => 2,
        0x2C => 1,
        0x2D => 1,
        0x2E => 2,
        0x2F => 1,

        0x30 => 2, //Branch
        0x31 => 3,
        0x32 => 2,
        0x33 => 2,
        0x34 => 3,
        0x35 => 3,
        0x36 => 3,
        0x37 => 1,
        0x38 => 2, //Branch
        0x39 => 2,
        0x3A => 2,
        0x3B => 2,
        0x3C => 1,
        0x3D => 1,
        0x3E => 2,
        0x3F => 1,

        0x40 => 1,
        0x41 => 1,
        0x42 => 1,
        0x43 => 1,
        0x44 => 1,
        0x45 => 1,
        0x46 => 2,
        0x47 => 1,
        0x48 => 1,
        0x49 => 1,
        0x4A => 1,
        0x4B => 1,
        0x4C => 1,
        0x4D => 1,
        0x4E => 2,
        0x4F => 1,

        0x50 => 1,
        0x51 => 1,
        0x52 => 1,
        0x53 => 1,
        0x54 => 1,
        0x55 => 1,
        0x56 => 2,
        0x57 => 1,
        0x58 => 1,
        0x59 => 1,
        0x5A => 1,
        0x5B => 1,
        0x5C => 1,
        0x5D => 1,
        0x5E => 2,
        0x5F => 1,

        0x60 => 1,
        0x61 => 1,
        0x62 => 1,
        0x63 => 1,
        0x64 => 1,
        0x65 => 1,
        0x66 => 2,
        0x67 => 1,
        0x68 => 1,
        0x69 => 1,
        0x6A => 1,
        0x6B => 1,
        0x6C => 1,
        0x6D => 1,
        0x6E => 2,
        0x6F => 1,

        0x70 => 2,
        0x71 => 2,
        0x72 => 2,
        0x73 => 2,
        0x74 => 2,
        0x75 => 2,
        0x76 => 1,
        0x77 => 2,
        0x78 => 1,
        0x79 => 1,
        0x7A => 1,
        0x7B => 1,
        0x7C => 1,
        0x7D => 1,
        0x7E => 2,
        0x7F => 1,

        0x80 => 1,
        0x81 => 1,
        0x82 => 1,
        0x83 => 1,
        0x84 => 1,
        0x85 => 1,
        0x86 => 2,
        0x87 => 1,
        0x88 => 1,
        0x89 => 1,
        0x8A => 1,
        0x8B => 1,
        0x8C => 1,
        0x8D => 1,
        0x8E => 2,
        0x8F => 1,

        0x90 => 1,
        0x91 => 1,
        0x92 => 1,
        0x93 => 1,
        0x94 => 1,
        0x95 => 1,
        0x96 => 2,
        0x97 => 1,
        0x98 => 1,
        0x99 => 1,
        0x9A => 1,
        0x9B => 1,
        0x9C => 1,
        0x9D => 1,
        0x9E => 2,
        0x9F => 1,

        0xA0 => 1,
        0xA1 => 1,
        0xA2 => 1,
        0xA3 => 1,
        0xA4 => 1,
        0xA5 => 1,
        0xA6 => 2,
        0xA7 => 1,
        0xA8 => 1,
        0xA9 => 1,
        0xAA => 1,
        0xAB => 1,
        0xAC => 1,
        0xAD => 1,
        0xAE => 2,
        0xAF => 1,

        0xB0 => 1,
        0xB1 => 1,
        0xB2 => 1,
        0xB3 => 1,
        0xB4 => 1,
        0xB5 => 1,
        0xB6 => 2,
        0xB7 => 1,
        0xB8 => 1,
        0xB9 => 1,
        0xBA => 1,
        0xBB => 1,
        0xBC => 1,
        0xBD => 1,
        0xBE => 2,
        0xBF => 1,

        0xC0 => 2, //Branch
        0xC1 => 3, 
        0xC2 => 3, //Branch
        0xC3 => 4,
        0xC4 => 3, //Branch
        0xC5 => 4,
        0xC6 => 2,
        0xC7 => 4,
        0xC8 => 2, //Branch
        0xC9 => 4, 
        0xCA => 3, //Branch
        0xCB => 0, //CB
        0xCC => 3, //Branch
        0xCD => 6,
        0xCE => 2,
        0xCF => 4,

        0xD0 => 2, //Branch
        0xD1 => 3,
        0xD2 => 3, //Branch
        0xD3 => 0, //Illegal
        0xD4 => 3, //Branch
        0xD5 => 4,
        0xD6 => 2,
        0xD7 => 4,
        0xD8 => 2, //Branch
        0xD9 => 4,
        0xDA => 3, //Branch
        0xDB => 0, //Illegal
        0xDC => 3, //Branch
        0xDD => 0, //Illegal
        0xDE => 2,
        0xDF => 4, 

        0xE0 => 3,
        0xE1 => 3,
        0xE2 => 2,
        0xE3 => 0, //Illegal
        0xE4 => 0, //Illegal
        0xE5 => 4,
        0xE6 => 2,
        0xE7 => 4,
        0xE8 => 4,
        0xE9 => 1,
        0xEA => 4,
        0xEB => 0, //Illegal
        0xEC => 0, //Illegal
        0xED => 0, //Illegal
        0xEE => 2,
        0xEF => 4,

        0xF0 => 3,
        0xF1 => 3,
        0xF2 => 2,
        0xF3 => 1,
        0xF4 => 0, //Illegal
        0xF5 => 4,
        0xF6 => 2,
        0xF7 => 4,
        0xF8 => 3,
        0xF9 => 2,
        0xFA => 4,
        0xFB => 1,
        0xFC => 0, //Illegal
        0xFD => 0, //Illegal
        0xFE => 2,
        0xFF => 4,
        _ => 0,
    }
}

pub fn cb_cycle_parse(opcode: u8) -> u8
{
    match opcode
    {
        0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x76 | 0x7E => 3,
        0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E |
        0x86 | 0x8E | 0x96 | 0x9E | 0xA6 | 0xAE | 0xB6 | 0xBE | 
        0xC6 | 0xCE | 0xD6 | 0xDE | 0xE6 | 0xEE | 0xF6 | 0xFE => 4,
        _ => 2,
    }
}