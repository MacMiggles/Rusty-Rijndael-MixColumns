use std::io;

fn g_multi(mut a: u8, mut b: u8) -> u8 {
    let mut p: u8 = 0;
    let _counter = 0;
    let mut hi_bit_set = 0;
    for _ in 0..8 {
        if b & 1 == 1 {
            p ^= a;
        }
        hi_bit_set = a & 0x80;
        a <<= 1;
        if hi_bit_set == 0x80 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    p
}

fn mix_column(column: &mut [u8]) {
    let mut copy = [0u8; 4];
    for i in 0..4 {
        copy[i] = column[i];
    }
    column[0] = g_multi(copy[0], 2) ^ 
                g_multi(copy[1], 3) ^
                g_multi(copy[2], 1) ^
                g_multi(copy[3], 1);

    column[1] = g_multi(copy[0], 1) ^
                g_multi(copy[1], 2) ^
                g_multi(copy[2], 3) ^
                g_multi(copy[3], 1);

    column[2] = g_multi(copy[0], 1) ^
                g_multi(copy[1], 1) ^
                g_multi(copy[2], 2) ^
                g_multi(copy[3], 3);

    column[3] = g_multi(copy[0], 3) ^
                g_multi(copy[1], 1) ^
                g_multi(copy[2], 1) ^
                g_multi(copy[3], 2) ;
}

fn main() {
    println!("Enter 4 bytes in hex (Reminder: 0x is not needed and must be UPPERCASE)");
    let mut column = [0u8; 4];
    for i in 0..4 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        column[i] = u8::from_str_radix(&input.trim(), 16).expect("Failed to parse hex");
    }
    mix_column(&mut column);
    println!("0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}", column[0], column[1], column[2], column[3]);
}
