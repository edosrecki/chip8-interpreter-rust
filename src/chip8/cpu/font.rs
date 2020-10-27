/// CHIP-8 contains built-in font which consists of 16 hexadecimal digits (sprites).
/// Each digit is 4px wide and 5px high, and occupies 5 bytes of memory (only 4 most
/// significant bits of each byte are used).
/// 
/// These digits are loaded in the interpreter area of CHIP-8 memory, from address
/// 0x000 to 0x050.
/// 
/// [Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#font)
/// 
/// # Digits
/// ```
/// Digit   Binary  Hex  Digit   Binary  Hex  Digit   Binary  Hex  Digit   Binary  Hex
/// ----------------------------------------------------------------------------------
/// ****    1111    F      *     0010    2    ****    1111    F    ****    1111    F
/// *  *    1001    9     **     0110    6       *    0001    1       *    0001    1
/// *  *    1001    9      *     0010    2    ****    1111    F    ****    1111    F
/// *  *    1001    9      *     0010    2    *       1000    8       *    0001    1
/// ****    1111    F     ***    0111    7    ****    1111    F    ****    1111    F
///
/// *  *    1001    9    ****    1111    F    ****    1111    F    ****    1111    F
/// *  *    1001    9    *       1000    8    *       1000    8       *    0001    1
/// ****    1111    F    ****    1111    F    ****    1111    F      *     0010    2
///    *    0001    1       *    0001    1    *  *    1001    9     *      0100    4
///    *    0001    1    ****    1111    F    ****    1111    F     *      0100    4
/// 
/// ****    1111    F    ****    1111    F    ****    1111    F    ***     1110    E
/// *  *    1001    9    *  *    1001    9    *  *    1001    9    *  *    1001    9
/// ****    1111    F    ****    1111    F    ****    1111    F    ***     1110    E
/// *  *    1001    9       *    0001    1    *  *    1001    9    *  *    1001    9
/// ****    1111    F    ****    1111    F    *  *    1001    9    ***     1110    E
/// 
/// ****    1111    F    ***     1110    E    ****    1111    F    ****    1111    F
/// *       1000    8    *  *    1001    9    *       1000    8    *       1000    8
/// *       1000    8    *  *    1001    9    ****    1111    F    ****    1111    F
/// *       1000    8    *  *    1001    9    *       1000    8    *       1000    8
/// ****    1111    F    ***     1110    E    ****    1111    F    *       1000    8
/// ```
pub const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
